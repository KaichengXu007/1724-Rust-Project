use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Rate limiting state
pub struct RateLimiter {
    requests: Arc<DashMap<String, Vec<Instant>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(DashMap::new()),
        }
    }

    pub fn check_rate_limit(&self, key: &str, limit: u32) -> bool {
        let now = Instant::now();
        let window = Duration::from_secs(60);

        let mut entry = self
            .requests
            .entry(key.to_string())
            .or_insert_with(Vec::new);

        // Remove old entries
        entry.retain(|&time| now.duration_since(time) < window);

        // Check limit
        if entry.len() >= limit as usize {
            return false;
        }

        // Add current request
        entry.push(now);
        true
    }

    /// Clean up old entries periodically
    pub fn cleanup(&self) {
        let now = Instant::now();
        let window = Duration::from_secs(60);

        self.requests.retain(|_, times| {
            times.retain(|&time| now.duration_since(time) < window);
            !times.is_empty()
        });
    }
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self {
            requests: self.requests.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new();

        // Should allow up to limit
        assert!(limiter.check_rate_limit("test-key", 3));
        assert!(limiter.check_rate_limit("test-key", 3));
        assert!(limiter.check_rate_limit("test-key", 3));

        // Should deny after limit
        assert!(!limiter.check_rate_limit("test-key", 3));
    }

    #[test]
    fn test_rate_limiter_different_keys() {
        let limiter = RateLimiter::new();

        assert!(limiter.check_rate_limit("key1", 1));
        assert!(limiter.check_rate_limit("key2", 1));

        // First key should be at limit
        assert!(!limiter.check_rate_limit("key1", 1));
        // Second key should still work
        assert!(!limiter.check_rate_limit("key2", 1));
    }
}
