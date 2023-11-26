use near_primitives::hash::CryptoHash;


#[derive(Debug, Clone)]
pub enum Event {
    NewBlock(NewBlock),
}
#[derive(Debug, Clone)]
pub enum Action {
    PrintToConsole(String),
    NOP,
}
#[derive(Debug, Clone)]
pub struct Config {}


/// event: new block, contains the block number and hash.
#[derive(Debug, Clone)]
pub struct NewBlock {
    pub hash: CryptoHash,
    pub height: u64,
}
