use crate::error::Error;
use crate::result::Result;
use futures::future::{join_all, BoxFuture, FutureExt};
use js_sys::{Array, Uint8Array};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use web_sys::{Blob, Document, Url};
use workflow_core::channel::oneshot;
use workflow_core::lookup::*;
use workflow_core::time::*;
use workflow_log::*;
use workflow_wasm::callback::*;

pub type Id = u64;
pub type ContentMap = HashMap<Id, Arc<Content>>;
pub type ContentList<'l> = &'l [(Id, Arc<Content>)];

static mut DOCUMENT_ROOT: Option<web_sys::Element> = None;

pub fn document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

pub fn root() -> web_sys::Element {
    unsafe {
        match DOCUMENT_ROOT.as_ref() {
            Some(root) => root.clone(),
            None => {
                let root = {
                    let collection = document().get_elements_by_tag_name("head");
                    if collection.length() > 0 {
                        collection.item(0).unwrap()
                    } else {
                        document().get_elements_by_tag_name("body").item(0).unwrap()
                    }
                };
                DOCUMENT_ROOT = Some(root.clone());
                root
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    Module,
    Script,
    Style,
}

impl ContentType {
    pub fn is_js(&self) -> bool {
        self == &ContentType::Script || self == &ContentType::Module
    }
}

#[allow(dead_code)]
pub enum Reference {
    Module,
    Script,
    Style,
    Export,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ContentStatus {
    Loaded,
    Exists,
    Error,
}

pub struct Content {
    pub content_type: ContentType,
    pub url: Mutex<Option<String>>,
    pub id: Id,
    pub ident: &'static str,
    pub content: &'static str,
    pub references: Option<&'static [(Reference, Option<&'static str>, Id)]>,
    pub is_loaded: AtomicBool,
}

// unsafe impl Send for Module {}
// unsafe impl Sync for Module {}

impl Content {
    pub fn url(&self) -> Option<String> {
        self.url.lock().unwrap().clone()
    }

    // fn content(&self, ctx: &Context) -> Result<String> {
    fn content(&self, ctx: &Context) -> Result<String> {
        let mut text = String::new();

        if let Some(references) = &self.references {
            let mut imports = Vec::new();
            let mut exports = Vec::new();

            for (kind, what, id) in references.iter() {
                let module = ctx
                    .get(id)
                    .ok_or(format!("unable to lookup module `{}`", self.ident))?;
                let url = module
                    .url()
                    .ok_or(format!("[{}] module is not loaded `{}`", self.ident, id))?;
                match kind {
                    Reference::Module => match what {
                        Some(detail) => {
                            imports.push(format!("import {detail} from \"{url}\";"));
                        }
                        None => {
                            imports.push(format!("import \"{url}\";"));
                        }
                    },
                    Reference::Export => {
                        let module = ctx
                            .get(id)
                            .ok_or(format!("unable to lookup module `{}`", self.ident))?;
                        let url = module
                            .url()
                            .ok_or(format!("[{}] module is not loaded `{}`", self.ident, id))?;
                        exports.push(format!("export {} from \"{}\";", what.unwrap(), url));
                    }
                    _ => {}
                }
            }

            let imports = imports.join("\n");
            let exports = exports.join("\n");

            text += &imports;
            text += self.content;
            text += &exports;
            Ok(text)
        } else {
            Ok(self.content.to_string())
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded.load(Ordering::SeqCst)
    }

    fn load_deps(self: Arc<Self>, ctx: Arc<Context>) -> BoxFuture<'static, Result<()>> {
        async move {
            if let Some(references) = &self.references {
                let futures = references
                    .iter()
                    .filter_map(|(_, _, id)| {
                        if let Some(content) = ctx.get(id) {
                            if !content.is_loaded.load(Ordering::SeqCst) {
                                Some(content.load(&ctx))
                            } else {
                                None
                            }
                        } else {
                            log_error!("Unable to locate module {}", id);
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                join_all(futures).await;

                // for future in futures {
                //     future.await?;
                // }
            }
            Ok(())
        }
        .boxed()
    }

    pub async fn load(self: Arc<Self>, ctx: &Arc<Context>) -> Result<ContentStatus> {
        ctx.load_content(self).await
    }

    fn create_blob_url(&self, ctx: &Arc<Context>) -> Result<String> {
        let content = self.content(ctx)?;
        let args = Array::new_with_length(1);
        args.set(0, unsafe { Uint8Array::view(content.as_bytes()).into() });
        let mut options = web_sys::BlobPropertyBag::new();
        match self.content_type {
            ContentType::Module | ContentType::Script => {
                options.type_("application/javascript");
            }
            ContentType::Style => {
                options.type_("text/css");
            }
        }

        let blob = Blob::new_with_u8_array_sequence_and_options(&args, &options)?;
        let url = Url::create_object_url_with_blob(&blob)?;
        self.url.lock().unwrap().replace(url.clone());
        Ok(url)
    }

    async fn load_impl(self: &Arc<Self>, ctx: &Arc<Context>) -> Result<ContentStatus> {
        if self.is_loaded() {
            return Ok(ContentStatus::Exists);
        }

        self.clone().load_deps(ctx.clone()).await?;
        // log_info!("load ... {}", self.ident);

        let (sender, receiver) = oneshot();
        let url = self.create_blob_url(ctx)?;

        // let ident = self.ident.clone();
        let callback = callback!(move |_event: web_sys::CustomEvent| {
            // log_info!("{} ... done", ident);
            // TODO - analyze event
            let status = ContentStatus::Loaded;
            sender.try_send(status).expect("unable to post load event");
        });

        match &self.content_type {
            ContentType::Module | ContentType::Script => {
                self.inject_script(&url, &callback)?;
            }
            ContentType::Style => {
                self.inject_style(&url, &callback)?;
            }
        };
        let status = receiver.recv().await.expect("unable to recv() load event");
        self.is_loaded.store(true, Ordering::SeqCst);
        Ok(status)
    }

    fn inject_script<C>(&self, url: &str, callback: &C) -> Result<()>
    where
        C: AsRef<js_sys::Function>,
    {
        let script = document().create_element("script")?;
        script.add_event_listener_with_callback("load", callback.as_ref())?;

        match &self.content_type {
            ContentType::Module => {
                script.set_attribute("module", "true")?;
                script.set_attribute("type", "module")?;
            }
            ContentType::Script => {
                script.set_attribute("type", "application/javascript")?;
            }
            _ => {
                panic!(
                    "inject_script() unsupported content type `{:?}`",
                    self.content_type
                )
            }
        }
        script.set_attribute("src", url)?;
        script.set_attribute("id", self.ident)?;
        root().append_child(&script)?;
        Ok(())
    }

    fn inject_style<C>(&self, url: &str, callback: &C) -> Result<()>
    where
        C: AsRef<js_sys::Function>,
    {
        let style = document().create_element("link")?;
        style.add_event_listener_with_callback("load", callback.as_ref())?;
        style.set_attribute("type", "text/css")?;
        style.set_attribute("rel", "stylesheet")?;
        style.set_attribute("href", url)?;
        style.set_attribute("id", self.ident)?;
        root().append_child(&style)?;
        println!("injecting style `{}`", self.ident);
        Ok(())
    }
}

pub struct Context {
    pub content: Arc<Mutex<ContentMap>>,
    pub lookup_handler: LookupHandler<Id, ContentStatus, Error>,
    pub loaded: AtomicUsize,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            content: Arc::new(Mutex::new(ContentMap::new())),
            lookup_handler: LookupHandler::new(),
            loaded: AtomicUsize::new(0),
        }
    }
}

impl Context {
    // pub fn new(content : ContentMap) -> Context {
    //     Context {
    //         content : Arc::new(Mutex::new(content)),
    //         lookup_handler: LookupHandler::new(),
    //         loaded : AtomicUsize::new(0),
    //     }
    // }

    pub fn declare(&self, content: ContentList) {
        self.content.lock().unwrap().extend(content.iter().cloned());
        // let mut map = self.content.lock().unwrap();
        // for (id, content) in content.iter() {
        //     map.insert(*id,content.clone());
        // }
    }

    pub fn get(&self, id: &Id) -> Option<Arc<Content>> {
        self.content.lock().unwrap().get(id).cloned()
    }

    pub async fn load_content(self: &Arc<Self>, content: Arc<Content>) -> Result<ContentStatus> {
        if content.is_loaded() {
            Ok(ContentStatus::Exists)
        } else {
            match self.lookup_handler.queue(&content.id).await {
                RequestType::New(receiver) => {
                    self.loaded.fetch_add(1, Ordering::SeqCst);
                    let result = content.load_impl(self).await;
                    self.lookup_handler.complete(&content.id, result).await;
                    receiver.recv().await?
                }
                RequestType::Pending(receiver) => receiver.recv().await?,
            }
        }
    }

    pub async fn load_ids(self: &Arc<Self>, list: &[Id]) -> Result<()> {
        let start = Instant::now();

        // let mut futures = Vec::with_capacity(list.len());
        // for id in list {
        //     if let Some(module) = self.get(id) {
        //         futures.push(module.load(self));
        //     }
        // }
        let futures = list
            .iter()
            .filter_map(|id| {
                if let Some(module) = self.get(id) {
                    Some(module.load(self))
                } else {
                    log_error!("Unable to locate module {}", id);
                    // TODO: panic
                    None
                }
            })
            .collect::<Vec<_>>();

        for future in futures {
            match future.await {
                Ok(_event) => {}
                Err(err) => {
                    log_error!("{}", err);
                }
            }
        }

        let elapsed = start.elapsed();
        let loaded = self.loaded.load(Ordering::SeqCst);
        log_info!(
            "Loaded {} references in {} msec",
            loaded,
            elapsed.as_millis()
        );

        Ok(())
    }
}

static mut CONTEXT: Option<Arc<Context>> = None;

pub fn context() -> Arc<Context> {
    unsafe {
        if let Some(context) = CONTEXT.as_ref() {
            context.clone()
        } else {
            let context = Arc::new(Context::default());
            CONTEXT = Some(context.clone());
            context
        }
    }
}

pub fn declare(content: ContentList) -> Arc<Context> {
    let ctx = context();
    ctx.declare(content);
    ctx
}
