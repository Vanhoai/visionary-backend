use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::{Barrier, Notify, RwLock, Semaphore, broadcast};
use tokio::task::JoinSet;

async fn pure_thread() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("Main: {}", i);
        tokio::time::sleep(Duration::from_millis(1)).await;
    }

    // Wait for the thread to finish
    handle.join().unwrap();
}

async fn message_passing() {
    let (tx, rx) = mpsc::channel();

    for i in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            tx.send(i).unwrap();
        });
    }

    drop(tx); // Close the original sender
    for received in rx {
        println!("Received: {}", received);
    }
}

async fn shared_state() {
    // Arc: Atomic Reference Counting
    // Mutex: Mutual Exclusion

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..1000 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

async fn readers_writer() {
    use std::sync::{Arc, RwLock};
    use std::thread;

    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // Readers
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            let r = data.read().unwrap();
            println!("Reader {}: {:?}", i, *r);
        });

        handles.push(handle);
    }

    // Writer
    let data = Arc::clone(&data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let mut w = data.write().unwrap();
        w.push(4);
        println!("Writer done");
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}

async fn async_function() -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;
    String::from("Async Function Result")
}

fn async_await_future() -> impl Future<Output = String> {
    async {
        let result = async_function().await;
        println!("{}", result);
        result
    }
}

fn custom_runtime() {
    // Single-threaded runtime
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(async {
        println!("Running on single-threaded runtime");
    });

    // Multithreaded runtime
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();

    rt.block_on(async {
        println!("Running on multi-threaded runtime");
    });
}

async fn tokio_tasks() {
    let handle = tokio::spawn(async {
        println!("Task running");
        42
    });

    let result = handle.await.unwrap();
    println!("Task result: {}", result);

    let mut handles = vec![];
    for i in 0..10 {
        let handle = tokio::spawn(async move {
            println!("Task {}", i);
            i * 2
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        println!("Result: {}", result);
    }
}

async fn tokio_selects() {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut counter = 0;

    loop {
        tokio::select! {
            _ = interval.tick() => {
                counter += 1;
                println!("Tick {}", counter);
            }

            _ = tokio::signal::ctrl_c() => {
                println!("Ctrl-C received, exiting");
                break;
            }
        }

        if counter >= 10 {
            break;
        }
    }
}

async fn tokio_channels() {
    // MPSC: Multi-producer, single-consumer channel
    let (tx, mut rx) = tokio::sync::mpsc::channel(100); // buffer size 100

    // Producer
    tokio::spawn(async move {
        for i in 0..10 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            tx.send(i).await.unwrap();
        }
    });

    // Consumer
    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }

    // Multiple producers
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    for i in 0..20 {
        let tx = tx.clone();
        tokio::spawn(async move {
            for j in 0..10 {
                tokio::time::sleep(Duration::from_millis(100)).await;
                tx.send(format!("Producer {} sends {}", i, j)).await.unwrap();
            }
        });
    }

    drop(tx); // Drop original sender

    while let Some(msg) = rx.recv().await {
        println!("{}", msg);
    }
}

async fn broadcast_channel() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();
    let mut rx3 = tx.subscribe();

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).unwrap();
        }
    });

    tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("Receiver 1: {}", msg);
        }
    });

    tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("Receiver 2: {}", msg);
        }
    });

    while let Ok(msg) = rx3.recv().await {
        println!("Receiver 3: {}", msg);
    }
}

async fn oneshot_channel() {
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send("Hello from task").unwrap();
    });

    match rx.await {
        Ok(msg) => println!("Received: {}", msg),
        Err(_) => println!("Sender dropped"),
    }
}

async fn watch_channel() {
    let (tx, mut rx) = tokio::sync::watch::channel(String::from("initial"));

    tokio::spawn(async move {
        for i in 0..5 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            tx.send(format!("update {}", i)).unwrap();
        }
    });

    while rx.changed().await.is_ok() {
        println!("Received: {}", *rx.borrow());
    }
}

async fn read_write_lock() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Multiple readers
    let mut handles = vec![];
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let read = data.read().await;
            println!("Reader {}: {:?}", i, *read);
        });
        handles.push(handle);
    }

    // Writer
    let data = Arc::clone(&data);
    let handle = tokio::spawn(async move {
        let mut write = data.write().await;
        write.push(4);
    });
    handles.push(handle);

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn tokio_semaphore() {
    let semaphore = Arc::new(Semaphore::new(2)); // Max 2 concurrent permits
    let mut handles = vec![];

    for i in 0..10 {
        let permit = semaphore.clone();
        let handle = tokio::spawn(async move {
            let _guard = permit.acquire().await.unwrap();
            println!("Task {} running", i);
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Task {} done", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn tokio_barrier() {
    let barrier = Arc::new(Barrier::new(5));
    let mut handles = vec![];

    for i in 0..10 {
        let barrier = Arc::clone(&barrier);
        let handle = tokio::spawn(async move {
            println!("Task {} before barrier", i);
            barrier.wait().await;
            println!("Task {} after barrier", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn tokio_notify() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    tokio::spawn(async move {
        println!("Waiting for notification");
        notify2.notified().await;
        println!("Received notification");
    });

    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Sending notification");
    notify.notify_one();

    tokio::time::sleep(Duration::from_secs(1)).await;
}

async fn long_operation() -> String {
    tokio::time::sleep(Duration::from_secs(5)).await;
    String::from("Done")
}

async fn tokio_time() {
    // Sleep
    println!("Sleeping...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Awake!");

    // Interval
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    for i in 0..5 {
        interval.tick().await;
        println!("Tick {}", i);
    }

    // Timeout
    match tokio::time::timeout(Duration::from_secs(2), long_operation()).await {
        Ok(result) => println!("Completed: {}", result),
        Err(_) => println!("Timeout!"),
    }

    // Instant
    let start = tokio::time::Instant::now();
    tokio::time::sleep(Duration::from_secs(1)).await;
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

async fn tokio_join_set() {
    let mut set = JoinSet::new();
    for i in 0..10 {
        set.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100 * i)).await;
            i
        });
    }

    // Wait for all tasks
    while let Some(res) = set.join_next().await {
        match res {
            Ok(val) => println!("Task completed: {}", val),
            Err(e) => println!("Task error: {}", e),
        }
    }
}

fn expensive_computation() -> i32 {
    // Heavy computation
    42
}

async fn avoid_operation_blocking() {
    // ❌ Bad - blocks the executor
    // std::thread::sleep(Duration::from_secs(1));

    // ✅ Good
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // ❌ Bad - CPU intensive
    // let result = expensive_computation();

    // ✅ Good - offload to blocking thread pool
    let result = tokio::task::spawn_blocking(|| expensive_computation()).await.unwrap();
    println!("Computation result: {}", result);
}

pub fn study_concurrency() {
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    rt.block_on(async {
        avoid_operation_blocking().await;
    });
}
