#[cfg(feature = "rate-limiting")]
use governor::{Quota, RateLimiter, state::{InMemoryState, NotKeyed}, clock::DefaultClock};
#[cfg(feature = "rate-limiting")]
use nonzero_ext::nonzero;
#[cfg(feature = "rate-limiting")]
use std::num::NonZeroU32;

#[cfg(feature = "rate-limiting")]
pub struct JikanRateLimiter {
    limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
}

#[cfg(feature = "rate-limiting")]
impl JikanRateLimiter {
    pub fn new() -> Self {
        let quota = Quota::per_second(nonzero!(3u32))
            .allow_burst(nonzero!(5u32));
        
        Self {
            limiter: RateLimiter::direct(quota),
        }
    }

    pub fn with_custom_rate(requests_per_second: u32, burst_size: u32) -> Self {
        let requests_per_second = NonZeroU32::new(requests_per_second).unwrap_or(nonzero!(1u32));
        let burst_size = NonZeroU32::new(burst_size).unwrap_or(nonzero!(1u32));
        
        let quota = Quota::per_second(requests_per_second)
            .allow_burst(burst_size);
        
        Self {
            limiter: RateLimiter::direct(quota),
        }
    }

    pub async fn wait_for_permit(&self) {
        self.limiter.until_ready().await;
    }

    pub fn check_permit(&self) -> bool {
        self.limiter.check().is_ok()
    }
}

#[cfg(feature = "rate-limiting")]
impl Default for JikanRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "rate-limiting"))]
pub struct JikanRateLimiter;

#[cfg(not(feature = "rate-limiting"))]
impl JikanRateLimiter {
    pub fn new() -> Self {
        Self
    }

    pub fn with_custom_rate(_requests_per_second: u32, _burst_size: u32) -> Self {
        Self
    }

    pub async fn wait_for_permit(&self) {
    }

    pub fn check_permit(&self) -> bool {
        true
    }
}

#[cfg(not(feature = "rate-limiting"))]
impl Default for JikanRateLimiter {
    fn default() -> Self {
        Self::new()
    }
} 