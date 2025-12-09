use llm_inference::middleware::RateLimiter;

#[test]
fn test_rate_limiter_allows_within_limit() {
    let limiter = RateLimiter::new();
    
    assert!(limiter.check_rate_limit("key1", 5));
    assert!(limiter.check_rate_limit("key1", 5));
    assert!(limiter.check_rate_limit("key1", 5));
}

#[test]
fn test_rate_limiter_blocks_over_limit() {
    let limiter = RateLimiter::new();
    
    // Use up the limit
    for _ in 0..3 {
        assert!(limiter.check_rate_limit("key1", 3));
    }
    
    // Should be blocked now
    assert!(!limiter.check_rate_limit("key1", 3));
}

#[test]
fn test_rate_limiter_different_keys_independent() {
    let limiter = RateLimiter::new();
    
    // Fill up key1
    assert!(limiter.check_rate_limit("key1", 1));
    assert!(!limiter.check_rate_limit("key1", 1));
    
    // key2 should still work
    assert!(limiter.check_rate_limit("key2", 1));
}

#[test]
fn test_rate_limiter_cleanup() {
    let limiter = RateLimiter::new();
    
    // Add some requests
    limiter.check_rate_limit("key1", 10);
    limiter.check_rate_limit("key2", 10);
    limiter.check_rate_limit("key3", 10);
    
    // Cleanup
    limiter.cleanup();
    
    // Should still work after cleanup (entries within window)
    assert!(limiter.check_rate_limit("key1", 10));
}

#[tokio::test]
async fn test_rate_limiter_concurrent_access() {
    use std::sync::Arc;
    
    let limiter = Arc::new(RateLimiter::new());
    let mut handles = vec![];
    
    for i in 0..10 {
        let limiter = limiter.clone();
        let handle = tokio::spawn(async move {
            limiter.check_rate_limit(&format!("key{}", i % 3), 5)
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let _ = handle.await;
    }
}
