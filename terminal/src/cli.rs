//!
//! Cli trait for implementing a user-side command-line processor.
//!

use crate::result::Result;
use crate::terminal::Terminal;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Cli: Sync + Send {
    fn init(&self, _term: &Arc<Terminal>) -> Result<()> {
        Ok(())
    }
    async fn digest(&self, term: Arc<Terminal>, cmd: String) -> Result<()>;
    async fn complete(&self, term: Arc<Terminal>, cmd: String) -> Result<Vec<String>>;
}
