//!
//! Cli trait for implementing a user-side command-line processor.
//!

use crate::error::Error;
use crate::parse;
pub use crate::result::Result;
use crate::terminal::Terminal;
use async_trait::async_trait;
use downcast::{downcast_sync, AnySync};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};
pub use workflow_terminal_macros::{declare_handler, register_handlers, Handler};

#[async_trait]
pub trait Cli: Sync + Send {
    fn init(&self, _term: &Arc<Terminal>) -> Result<()> {
        Ok(())
    }
    async fn digest(self: Arc<Self>, term: Arc<Terminal>, cmd: String) -> Result<()>;
    async fn complete(
        self: Arc<Self>,
        term: Arc<Terminal>,
        cmd: String,
    ) -> Result<Option<Vec<String>>>;
    fn prompt(&self) -> Option<String>;
}

pub trait Context: Sync + Send + AnySync {
    fn term(&self) -> Arc<Terminal>;
}
downcast_sync!(dyn Context);
downcast_sync!(dyn Context + Sync + Send);

impl From<&dyn Context> for Arc<Terminal> {
    fn from(ctx: &dyn Context) -> Arc<Terminal> {
        ctx.term()
    }
}

#[async_trait]
pub trait Handler: Sync + Send + AnySync {
    fn verb(&self, _ctx: &Arc<dyn Context>) -> Option<&'static str> {
        None
    }
    fn condition(&self, ctx: &Arc<dyn Context>) -> bool {
        self.verb(ctx).is_some()
    }
    fn help(&self, _ctx: &Arc<dyn Context>) -> &'static str {
        ""
    }
    fn dyn_help(&self, _ctx: &Arc<dyn Context>) -> String {
        "".to_owned()
    }
    async fn complete(&self, _ctx: &Arc<dyn Context>, _cmd: &str) -> Result<Option<Vec<String>>> {
        Ok(None)
    }
    async fn start(self: Arc<Self>, _ctx: &Arc<dyn Context>) -> Result<()> {
        Ok(())
    }
    async fn stop(self: Arc<Self>, _ctx: &Arc<dyn Context>) -> Result<()> {
        Ok(())
    }
    async fn handle(
        self: Arc<Self>,
        ctx: &Arc<dyn Context>,
        argv: Vec<String>,
        cmd: &str,
    ) -> Result<()>;
}

downcast_sync!(dyn Handler);

pub fn get_handler_help(handler: Arc<dyn Handler>, ctx: &Arc<dyn Context>) -> String {
    let s = handler.help(ctx);
    if s.is_empty() {
        handler.dyn_help(ctx)
    } else {
        s.to_string()
    }
}

#[derive(Default)]
struct Inner {
    handlers: HashMap<String, Arc<dyn Handler>>,
}

#[derive(Default)]
pub struct HandlerCli {
    inner: Arc<Mutex<Inner>>,
}

impl HandlerCli {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner::default())),
        }
    }

    fn inner(&self) -> MutexGuard<Inner> {
        self.inner.lock().unwrap()
    }

    pub fn collect(&self) -> Vec<Arc<dyn Handler>> {
        self.inner().handlers.values().cloned().collect::<Vec<_>>()
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Handler>> {
        self.inner().handlers.get(name).cloned()
    }

    pub fn register<T, H>(&self, ctx: &Arc<T>, handler: H)
    where
        T: Context + Sized,
        H: Handler + Send + Sync + 'static,
    {
        let ctx: Arc<dyn Context> = ctx.clone();
        match handler.verb(&ctx) {
            Some(name) if handler.condition(&ctx) => {
                self.inner()
                    .handlers
                    .insert(name.to_lowercase(), Arc::new(handler));
            }
            _ => {}
        }
    }

    pub fn register_arc<T, H>(&self, ctx: &Arc<T>, handler: &Arc<H>)
    where
        T: Context + Sized,
        H: Handler + Send + Sync + 'static,
    {
        let ctx: Arc<dyn Context> = ctx.clone();
        match handler.verb(&ctx) {
            Some(name) if handler.condition(&ctx) => {
                self.inner()
                    .handlers
                    .insert(name.to_lowercase(), handler.clone());
            }
            _ => {}
        }
    }

    pub fn unregister(&self, name: &str) -> Option<Arc<dyn Handler>> {
        self.inner().handlers.remove(name)
    }

    pub fn clear(&self) -> Result<()> {
        self.inner().handlers.clear();
        Ok(())
    }

    pub async fn start<T>(&self, ctx: &Arc<T>) -> Result<()>
    where
        T: Context + Sized,
    {
        let ctx: Arc<dyn Context> = ctx.clone();
        let handlers = self.collect();
        for handler in handlers.iter() {
            handler.clone().start(&ctx).await?;
        }
        Ok(())
    }

    pub async fn stop<T>(&self, ctx: &Arc<T>) -> Result<()>
    where
        T: Context + Sized,
    {
        let handlers = self.collect();
        let ctx: Arc<dyn Context> = ctx.clone();
        for handler in handlers.into_iter() {
            handler.clone().start(&ctx).await?;
        }
        Ok(())
    }

    pub async fn execute<T>(&self, ctx: &Arc<T>, cmd: &str) -> Result<()>
    where
        T: Context + Sized,
    {
        let ctx: Arc<dyn Context> = ctx.clone();

        let argv = parse(cmd);
        let action = argv[0].to_lowercase();

        let handler = self.get(action.as_str());
        if let Some(handler) = handler {
            handler
                .clone()
                .handle(&ctx, argv[1..].to_vec(), cmd)
                .await?;
            Ok(())
        } else {
            Err(Error::CommandNotFound(action))
        }
    }

    pub async fn complete<T>(&self, ctx: &Arc<T>, cmd: &str) -> Result<Option<Vec<String>>>
    where
        T: Context + Sized,
    {
        let ctx: Arc<dyn Context> = ctx.clone();

        let argv = parse(cmd);
        let action = argv[0].to_lowercase();

        let handler = self.get(action.as_str());
        if let Some(handler) = handler {
            Ok(handler.clone().complete(&ctx, cmd).await?)
        } else {
            Err(Error::CommandNotFound(action))
        }
    }
}
