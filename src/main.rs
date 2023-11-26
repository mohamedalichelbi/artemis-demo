mod wallets;
mod types;
mod collectors;
mod strategies;
mod executors;

use std::{sync::Arc, error::Error};
use tokio::{self, time::Duration};
use wallets::near_wallet::NearWallet;
use dotenv::dotenv;
use artemis_core::{
    engine::Engine,
    types::{CollectorMap, ExecutorMap},
};
use collectors::block_collector::BlockCollector;
use strategies::new_block::NewBlockStrat;
use executors::console_printer::CPExecutor;
use types::{Event, Action};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let wallet = Arc::new(NearWallet::new());

    // Artemis engine
    let mut engine: Engine<Event, Action> = Engine::new();

    // set up block collector.
    let block_collector = Box::new(BlockCollector::new(
        Arc::clone(&wallet),
        Duration::from_millis(500)
    ));

    // add collectors
    let block_collector = CollectorMap::new(
        block_collector,
        |new_block| { Event::NewBlock(new_block) },
    );
    engine.add_collector(Box::new(block_collector));

    // add strategies
    let new_block_strat = NewBlockStrat::new();
    engine.add_strategy(Box::new(new_block_strat));

    // set up console-printing executor
    let cp_executor = Box::new(CPExecutor::new());

    // add executors
    let cp_executor = ExecutorMap::new(
        cp_executor,
        |action| match action {
            Action::PrintToConsole(txt) => Some(txt),
            _ => None,
        }
    );
    engine.add_executor(Box::new(cp_executor));

    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            println!("res: {:?}", res);
        }
    }

    Ok(())
}
