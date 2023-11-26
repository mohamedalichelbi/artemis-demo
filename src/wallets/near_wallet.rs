use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::{
    types::BlockReference,
    views::BlockView,
};


pub struct NearWallet {
    rpc: JsonRpcClient,
}


impl NearWallet {
    pub fn new() -> Self {
        Self { rpc: JsonRpcClient::connect("https://rpc.mainnet.near.org") }
    }

    pub async fn block_details(&self, block_ref: BlockReference) -> Result<BlockView, Box<dyn std::error::Error>> {
        let block_details_request = methods::block::RpcBlockRequest {
            block_reference: block_ref,
        };
        let response = self.rpc.call(&block_details_request).await?;

        Ok(response)
    }
}
