use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use url::Url;
use anyhow::{Context, Result};

pub struct StarknetClient {
    provider: JsonRpcClient<HttpTransport>,
}

impl StarknetClient {
    pub fn new(rpc_url: &str) -> Result<Self> {
        let url = Url::parse(rpc_url).context("Invalid RPC URL")?;
        let provider = JsonRpcClient::new(HttpTransport::new(url));
        Ok(StarknetClient { provider })
    }

    pub async fn get_block_number(&self) -> Result<u64> {
        let block_number = self.provider.block_number().await
            .map_err(|e| anyhow::anyhow!("Failed to fetch block number: {}", e))?;
        Ok(block_number)
    }

    // Placeholder for other methods: call, invoke, etc.
}
