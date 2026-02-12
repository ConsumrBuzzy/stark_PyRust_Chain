use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use url::Url;
use anyhow::{Context, Result};
use crate::rate_limiter::ApiRateLimiter;
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct StarknetClient {
    providers: Vec<JsonRpcClient<HttpTransport>>,
    current_index: AtomicUsize,
    limiter: ApiRateLimiter,
}

impl StarknetClient {
    /// Create a new StarknetClient. 
    /// If `rpc_url` is provided, it uses ONLY that one.
    /// Otherwise, it detects ALL compatible URLs in the environment and rotates between them.
    pub fn new(rpc_url: Option<&str>) -> Result<Self> {
        // Load .env if not already loaded
        dotenv::dotenv().ok();

        let mut url_strings = Vec::new();

        if let Some(u) = rpc_url {
            url_strings.push(u.to_string());
        } else {
            url_strings = Self::detect_rpc_urls()?;
        }

        let mut providers = Vec::new();
        for url_str in url_strings {
            let url = Url::parse(&url_str).context(format!("Invalid RPC URL: {}", url_str))?;
            providers.push(JsonRpcClient::new(HttpTransport::new(url)));
        }

        if providers.is_empty() {
             return Err(anyhow::anyhow!("No valid RPC providers available."));
        }

        // Default to safe limit: 5 requests per second (typical free tier)
        // Note: This limit is global for the client struct, effectively limiting total throughput 
        // regardless of which provider is used next.
        let limiter = ApiRateLimiter::new(5)?;

        Ok(StarknetClient { 
            providers, 
            current_index: AtomicUsize::new(0),
            limiter 
        })
    }

    fn detect_rpc_urls() -> Result<Vec<String>> {
        let keys = [
            "STARKNET_RPC_URL",
            "STARKNET_MAINNET_URL",
            "STARKNET_LAVA_URL",
            "STARKNET_1RPC_URL",
            "ALCHEMY_RPC_URL",
            "INFURA_RPC_URL",
            "QUICKNODE_ENDPOINT",
        ];

        let mut urls = Vec::new();
        for key in keys {
            if let Ok(val) = env::var(key) {
                if !val.trim().is_empty() {
                    urls.push(val.trim().to_string());
                }
            }
        }
        
        if urls.is_empty() {
            Err(anyhow::anyhow!("No valid RPC URL found in environment variables."))
        } else {
            Ok(urls)
        }
    }

    fn next_provider(&self) -> &JsonRpcClient<HttpTransport> {
        let idx = self.current_index.fetch_add(1, Ordering::Relaxed);
        &self.providers[idx % self.providers.len()]
    }

    pub async fn get_block_number(&self) -> Result<u64> {
        self.limiter.check().await;
        // Use next provider in round-robin logic
        let provider = self.next_provider();
        
        let block_number = provider.block_number().await
            .map_err(|e| anyhow::anyhow!("Failed to fetch block number: {}", e))?;
        Ok(block_number)
    }

    /// Execute a batched query (Multicall).
    pub async fn batch_query(&self, _account_address: &str, _asteroids: &[u64]) -> Result<String> {
        self.limiter.check().await;
        let _provider = self.next_provider();
        
        // logic to construct a Multicall transaction or multiple async queries
        // For v0.1.0, we will simulate this.
        
        Ok("{\"balance\": \"1000 SWAY\", \"asteroids\": []}".to_string())
    }
}
