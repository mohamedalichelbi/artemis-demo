use std::sync::Arc;
use crate::{
    wallets::near_wallet::NearWallet,
    types::NewBlock,
};
use artemis_core::types::{Collector, CollectorStream};
use near_primitives::types::{BlockReference, Finality};
use anyhow::Result;
use async_trait::async_trait;
use tokio::time::{Duration, interval};
use async_stream::stream;

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct BlockCollector {
    wallet: Arc<NearWallet>,
    duration: Duration,
}

impl BlockCollector {
    pub fn new(wallet: Arc<NearWallet>, duration: Duration) -> Self {
        Self { wallet, duration }
    }
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new blocks.
#[async_trait]
impl Collector<NewBlock> for BlockCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewBlock>> {
        let mut interval = interval(self.duration);
        let block_stream = stream! {
            // polling loop
            loop {
                interval.tick().await;
                // fetch latest block
                let block = self.wallet.block_details(BlockReference::Finality(Finality::None)).await.unwrap();
                yield NewBlock { hash: block.header.hash, height: block.header.height };
            }
        };

        Ok(Box::pin(block_stream))
    }
}
