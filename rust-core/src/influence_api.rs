use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use crate::rate_limiter::ApiRateLimiter;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Asteroid {
    pub id: u64,
    pub name: String,
    pub area: u64,
    // Add other fields as per API spec
}

pub struct InfluenceClient {
    client: Client,
    base_url: String,
    limiter: ApiRateLimiter,
}

impl InfluenceClient {
    pub fn new() -> Result<Self> {
        let base_url = env::var("INFLUENCE_API_URL")
            .unwrap_or_else(|_| "https://api.influenceth.io".to_string());
        
        let client = Client::new();
        // Influence API limits are strict. Conservative default: 2 req/s.
        let limiter = ApiRateLimiter::new(2)?;
        
        Ok(InfluenceClient {
            client,
            base_url,
            limiter,
        })
    }

    pub async fn get_asteroid(&self, asteroid_id: u64) -> Result<Asteroid> {
        self.limiter.check().await;
        
        let url = format!("{}/v1/asteroids/{}", self.base_url, asteroid_id);
        let resp = self.client.get(&url)
            .send()
            .await
            .context("Failed to send request to Influence API")?;
            
        if !resp.status().is_success() {
             return Err(anyhow::anyhow!("Influence API Error: {}", resp.status()));
        }

        let asteroid = resp.json::<Asteroid>()
            .await
            .context("Failed to parse Asteroid JSON")?;
            
        Ok(asteroid)
    }

    // Add other methods: get_market_data, get_inventory, etc.
}
