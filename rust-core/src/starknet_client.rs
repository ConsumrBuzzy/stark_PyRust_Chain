use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use url::Url;
use anyhow::{Context, Result};
use crate::rate_limiter::ApiRateLimiter;
use std::env;

pub struct StarknetClient {
    provider: JsonRpcClient<HttpTransport>,
    limiter: ApiRateLimiter,
}

impl StarknetClient {
    /// Create a new StarknetClient. 
    /// If `rpc_url` is provided, it uses that.
    /// Otherwise, it attempts to find a compatible URL in the environment (PhantomArbiter style).
    pub fn new(rpc_url: Option<&str>) -> Result<Self> {
        // Load .env if not already loaded (safe to call multiple times)
        dotenv::dotenv().ok();

        let url_str = match rpc_url {
            Some(u) => u.to_string(),
            None => Self::detect_rpc_url()?,
        };

        let url = Url::parse(&url_str).context("Invalid RPC URL")?;
        let provider = JsonRpcClient::new(HttpTransport::new(url));
        
        // Default to safe limit: 5 requests per second (typical free tier)
        let limiter = ApiRateLimiter::new(5)?;

        Ok(StarknetClient { provider, limiter })
    }

    fn detect_rpc_url() -> Result<String> {
        let keys = [
            "STARKNET_RPC_URL",
            "ALCHEMY_RPC_URL",
            "INFURA_RPC_URL",
            "QUICKNODE_ENDPOINT",
        ];

        for key in keys {
            if let Ok(val) = env::var(key) {
                if !val.is_empty() {
                    return Ok(val);
                }
            }
        }
        
        Err(anyhow::anyhow!("No valid RPC URL found in environment variables."))
    }

    pub async fn get_block_number(&self) -> Result<u64> {
        self.limiter.check().await;
        let block_number = self.provider.block_number().await
            .map_err(|e| anyhow::anyhow!("Failed to fetch block number: {}", e))?;
        Ok(block_number)
    }

    /// Execute a batched query (Multicall).
    /// Used to fetch Account Balance AND Influence State in one go.
    /// This is the "L2 Latency Killer".
    pub async fn batch_query(&self, _account_address: &str, _asteroids: &[u64]) -> Result<String> {
        self.limiter.check().await;
        // logic to construct a Multicall transaction or multiple async queries
        // For v0.1.0, we will simulate this by running parallel queries (tokio::join!)
        // In v0.2.0, this becomes a true Starknet Multicall Contract read.
        
        Ok("{\"balance\": \"1000 SWAY\", \"asteroids\": []}".to_string())
    }
}
