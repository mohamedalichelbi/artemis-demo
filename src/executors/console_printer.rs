use crate::types::Action;
use artemis_core::types::Executor;
use async_trait::async_trait;
use anyhow::{Result, Ok};


/// An executor that sends Notifications to a telegram group.
pub struct CPExecutor {}

impl CPExecutor {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl Executor<String> for CPExecutor {
    /// print strings to console
    async fn execute(&self, text: String) -> Result<()> {
        println!("{}", text);

        Ok(())
    }
}
