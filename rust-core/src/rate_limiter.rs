use governor::{Quota, RateLimiter};
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use std::sync::Arc;
use std::num::NonZeroU32;
use anyhow::{Result, Context};

/// A wrapper around governor's RateLimiter to provide a simplified interface.
/// Uses a direct (not keyed) rate limiter for global API limits.
#[derive(Clone)]
pub struct ApiRateLimiter {
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl ApiRateLimiter {
    /// Create a new rate limiter with a specified quota (requests per second).
    pub fn new(requests_per_second: u32) -> Result<Self> {
        let nonzero = NonZeroU32::new(requests_per_second)
            .context("Requests per second must be > 0")?;
        
        let quota = Quota::per_second(nonzero);
        let limiter = RateLimiter::direct(quota);
        
        Ok(ApiRateLimiter {
            limiter: Arc::new(limiter),
        })
    }

    /// Block (async) until a permit is available.
    pub async fn check(&self) {
        self.limiter.until_ready().await;
    }
}
