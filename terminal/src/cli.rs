//!
//! Cli trait for implementing a user-side command-line processor.
//!

use crate::error::Error;
use crate::parse;
pub use crate::result::Result;
use crate::terminal::Terminal;
use async_trait::async_trait;
use downcast::{AnySync, downcast_sync};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};
pub use workflow_terminal_macros::{Handler, declare_handler, register_handlers};

/// User-implemented command-line processor driven by a [`Terminal`].
///
/// An implementation receives the user's input lines and is responsible for
/// parsing, executing, and providing completion candidates for commands.
#[async_trait]
pub trait Cli: Sync + Send {
    /// Called once when the CLI is bound to a terminal, allowing setup.
    fn init(self: Arc<Self>, _term: &Arc<Terminal>) -> Result<()> {
        Ok(())
    }
    /// Processes a single command line entered by the user.
    async fn digest(self: Arc<Self>, term: Arc<Terminal>, cmd: String) -> Result<()>;
    /// Returns completion candidates for the current partial input, if any.
    async fn complete(
        self: Arc<Self>,
        term: Arc<Terminal>,
        cmd: String,
    ) -> Result<Option<Vec<String>>>;
    /// Returns the prompt string to display, or `None` for the default prompt.
    fn prompt(&self) -> Option<String>;
}

/// Shared execution context passed to command handlers, providing access to
/// the underlying terminal and allowing downcasting to a concrete type.
pub trait Context: Sync + Send + AnySync {
    /// Returns the terminal associated with this context.
    fn term(&self) -> Arc<Terminal>;
}
downcast_sync!(dyn Context);
downcast_sync!(dyn Context + Sync + Send);

impl From<&dyn Context> for Arc<Terminal> {
    fn from(ctx: &dyn Context) -> Arc<Terminal> {
        ctx.term()
    }
}

/// A single command handler, identified by a verb, that can be registered
/// with a [`HandlerCli`] and invoked when its verb is entered.
#[async_trait]
pub trait Handler: Sync + Send + AnySync {
    /// Returns the command verb this handler responds to, or `None` to disable it.
    fn verb(&self, _ctx: &Arc<dyn Context>) -> Option<&'static str> {
        None
    }
    /// Returns whether this handler should be registered in the given context.
    fn condition(&self, ctx: &Arc<dyn Context>) -> bool {
        self.verb(ctx).is_some()
    }
    /// Returns static help text describing the command.
    fn help(&self, _ctx: &Arc<dyn Context>) -> &'static str {
        ""
    }
    /// Returns dynamically generated help text, used when [`help`](Self::help) is empty.
    fn dyn_help(&self, _ctx: &Arc<dyn Context>) -> String {
        "".to_owned()
    }
    /// Returns completion candidates for the command's arguments, if any.
    async fn complete(&self, _ctx: &Arc<dyn Context>, _cmd: &str) -> Result<Option<Vec<String>>> {
        Ok(None)
    }
    /// Called when the owning [`HandlerCli`] starts, allowing setup.
    async fn start(self: Arc<Self>, _ctx: &Arc<dyn Context>) -> Result<()> {
        Ok(())
    }
    /// Called when the owning [`HandlerCli`] stops, allowing teardown.
    async fn stop(self: Arc<Self>, _ctx: &Arc<dyn Context>) -> Result<()> {
        Ok(())
    }
    /// Executes the command with the parsed argument vector and raw command line.
    async fn handle(
        self: Arc<Self>,
        ctx: &Arc<dyn Context>,
        argv: Vec<String>,
        cmd: &str,
    ) -> Result<()>;
}

downcast_sync!(dyn Handler);

/// Returns the help text for a handler, preferring static [`Handler::help`]
/// and falling back to [`Handler::dyn_help`] when the static text is empty.
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
/// A registry-based [`Cli`] implementation that dispatches command lines to
/// registered [`Handler`]s keyed by their verb.
pub struct HandlerCli {
    inner: Arc<Mutex<Inner>>,
}

impl HandlerCli {
    /// Creates a new, empty handler registry.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner::default())),
        }
    }

    fn inner(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().unwrap()
    }

    /// Returns all currently registered handlers.
    pub fn collect(&self) -> Vec<Arc<dyn Handler>> {
        self.inner().handlers.values().cloned().collect::<Vec<_>>()
    }

    /// Returns the handler registered under the given verb, if any.
    pub fn get(&self, name: &str) -> Option<Arc<dyn Handler>> {
        self.inner().handlers.get(name).cloned()
    }

    /// Registers a handler by value under its verb, if its
    /// [`condition`](Handler::condition) is met in the given context.
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

    /// Registers an already-`Arc`-wrapped handler under its verb, if its
    /// [`condition`](Handler::condition) is met in the given context.
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

    /// Removes and returns the handler registered under the given verb, if any.
    pub fn unregister(&self, name: &str) -> Option<Arc<dyn Handler>> {
        self.inner().handlers.remove(name)
    }

    /// Removes all registered handlers.
    pub fn clear(&self) -> Result<()> {
        self.inner().handlers.clear();
        Ok(())
    }

    /// Invokes [`Handler::start`] on every registered handler.
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

    /// Invokes the stop lifecycle on every registered handler.
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

    /// Parses the command line and dispatches it to the matching handler,
    /// returning [`Error::CommandNotFound`] if no handler matches the verb.
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

    /// Returns completion candidates for the command line by delegating to the
    /// matching handler, or [`Error::CommandNotFound`] if no handler matches.
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
