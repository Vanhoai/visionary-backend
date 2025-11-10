use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

/// Token Bucket Rate Limiter
/// Allows a certain number of tokens to be consumed over a time period.

struct TokenBucket {
    tokens: Arc<Mutex<f64>>,          // current tokens
    capacity: f64,                    // maximum tokens
    refill_rate: f64,                 // tokens per second
    last_refill: Arc<Mutex<Instant>>, // last refill timestamp
}

impl TokenBucket {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        TokenBucket {
            tokens: Arc::new(Mutex::new(capacity)),
            capacity,
            refill_rate,
            last_refill: Arc::new(Mutex::new(Instant::now())),
        }
    }

    // Try to acquire tokens without waiting
    pub async fn try_acquire(&self, tokens_needed: f64) -> bool {
        let mut tokens = self.tokens.lock().await;
        let mut last_refill = self.last_refill.lock().await;

        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill).as_secs_f64();
        let new_tokens = elapsed * self.refill_rate;

        *tokens = (*tokens + new_tokens).min(self.capacity);
        *last_refill = now;

        if *tokens >= tokens_needed {
            *tokens -= tokens_needed;
            true
        } else {
            false
        }
    }

    // Wait until tokens are available and acquire them
    #[allow(dead_code)]
    pub async fn acquire(&self, tokens_needed: f64) {
        loop {
            if self.try_acquire(tokens_needed).await {
                return;
            }

            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    // Get the current number of available tokens
    pub async fn available_tokens(&self) -> f64 {
        *self.tokens.lock().await
    }
}

#[allow(dead_code)]
pub fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    rt.block_on(async {
        // 10 tokens capacity, refill 2 tokens/second
        let limiter = Arc::new(TokenBucket::new(10.0, 2.0));

        // Spawn multiple tasks to simulate concurrent requests
        let mut handles = vec![];
        for i in 0..10 {
            let limiter = Arc::clone(&limiter);
            let handle = tokio::spawn(async move {
                if limiter.try_acquire(1.0).await {
                    println!("Burst request {} succeeded", i);
                } else {
                    println!("Burst request {} denied", i);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        // Request fail
        tokio::time::sleep(Duration::from_millis(10)).await;
        if limiter.try_acquire(1.0).await {
            println!("Extra request succeeded");
        } else {
            println!("Extra request denied - bucket empty");
        }

        // refill
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("\nAvailable tokens after 1s: {}", limiter.available_tokens().await)
    });
}
