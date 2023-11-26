use crate::types::{Event, Action, NewBlock};
use artemis_core::types::Strategy;
use anyhow::Result;
use async_trait::async_trait;

pub struct NewBlockStrat {
    last_height: u64,
}

impl NewBlockStrat {
    pub fn new() -> Self {
        Self { last_height: 0 }
    }
}

#[async_trait]
impl Strategy<Event, Action> for NewBlockStrat {
    async fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::NewBlock(block) => self.handle_new_block(block),
            _ => vec![Action::NOP],
        }
    }
}

impl NewBlockStrat {
    fn handle_new_block(&mut self, data: NewBlock) -> Vec<Action> {
        if data.height > self.last_height {
            self.last_height = data.height;
            vec![Action::PrintToConsole(format!(r#"
                New block!
                Hash: {}
                Height:{}"#,
                data.hash,
                data.height
            ))]
        }
        else { vec![Action::NOP] }
    }
}
