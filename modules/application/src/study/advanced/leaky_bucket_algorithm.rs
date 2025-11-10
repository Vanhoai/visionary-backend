use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{Duration, Instant, interval};

/// Leaky Bucket Rate Limiter
/// Allows a fixed rate of requests by processing them at a constant rate.

pub struct LeakyBucket {
    tx: mpsc::Sender<Request>,
}

struct Request {
    id: usize,
    respond_to: tokio::sync::oneshot::Sender<()>,
}

impl LeakyBucket {
    pub fn new(rate_per_second: u64, queue_size: usize) -> Self {
        let (tx, mut rx) = mpsc::channel::<Request>(queue_size);

        // Background worker to process requests at a fixed rate
        tokio::spawn(async move {
            let delay = Duration::from_millis(1000 / rate_per_second);
            let mut interval = interval(delay);
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            while let Some(request) = rx.recv().await {
                interval.tick().await;
                println!("Processing request {} at {:?}", request.id, Instant::now());
                let _ = request.respond_to.send(());
            }
        });

        LeakyBucket { tx }
    }

    // Process a request, waiting if necessary
    pub async fn process(&self, id: usize) -> Result<(), String> {
        let (respond_tx, respond_rx) = tokio::sync::oneshot::channel();
        self.tx.send(Request { id, respond_to: respond_tx }).await.map_err(|_| "Bucket full".to_string())?;
        respond_rx.await.map_err(|_| "Request cancelled".to_string())
    }

    // Try to process a request without waiting
    #[allow(dead_code)]
    pub async fn try_process(&self, id: usize) -> Result<(), String> {
        let (respond_tx, respond_rx) = tokio::sync::oneshot::channel();

        match self.tx.try_send(Request { id, respond_to: respond_tx }) {
            Ok(_) => respond_rx.await.map_err(|_| "Request cancelled".to_string()),
            Err(_) => Err("Queue full".to_string()),
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let bucket = Arc::new(LeakyBucket::new(5, 10));
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..15 {
            let bucket_clone = bucket.clone();
            let handle = tokio::spawn(async move {
                match bucket_clone.process(i).await {
                    Ok(_) => {
                        let elapsed = start.elapsed();
                        println!("✓ Request {} completed after {:?}", i, elapsed);
                    },
                    Err(e) => println!("✗ Request {} failed: {}", i, e),
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    });
}
