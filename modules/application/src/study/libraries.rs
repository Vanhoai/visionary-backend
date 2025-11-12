use crossbeam::deque::{Injector, Stealer, Worker};
use crossbeam::{channel, thread};
use rayon::prelude::*;

#[allow(dead_code)]
fn study_rayon() {
    // Parallel iteration
    let sum: i32 = (0..1000).into_par_iter().sum();
    println!("Sum: {}", sum);

    // Parallel map
    // let squares: Vec<i32> = (0..100).into_par_iter().map(|x| x * x).collect();

    // Parallel sort
    let mut data = [3, 1, 4, 1, 5, 9, 2, 6];
    data.par_sort();

    // Parallel for_each
    (0..10).into_par_iter().for_each(|i| {
        println!("Processing {}", i);
    });

    // Custom thread pool
    let pool = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    pool.install(|| {
        // Work happens in custom pool
        let result: i32 = (0..100).into_par_iter().sum();
        println!("Result: {}", result);
    });
}

#[allow(dead_code)]
fn study_crossbeam() {
    // Unbounded channel
    let (s, r) = channel::unbounded();
    thread::scope(|scope| {
        scope.spawn(|_| {
            s.send("Hello").unwrap();
        });

        scope.spawn(|_| {
            let msg = r.recv().unwrap();
            println!("Received: {}", msg);
        });
    })
    .unwrap();

    // Select over multiple channels
    let (s1, r1) = channel::unbounded();
    let (s2, r2) = channel::unbounded();

    thread::scope(|scope| {
        scope.spawn(|_| s1.send(1).unwrap());
        scope.spawn(|_| s2.send(2).unwrap());

        scope.spawn(|_| {
            crossbeam::select! {
                recv(r1) -> msg => println!("From r1: {:?}", msg),
                recv(r2) -> msg => println!("From r2: {:?}", msg),
            }
        });
    })
    .unwrap();
}

#[allow(dead_code)]
fn work_stealing() {
    let injector: Injector<i32> = Injector::new();
    let worker: Worker<i32> = Worker::new_fifo();
    let stealer: Stealer<i32> = worker.stealer();

    // Push tasks
    for i in 0..100 {
        injector.push(i);
    }

    // Workers steal tasks
    thread::scope(|scope| {
        for _ in 0..4 {
            let stealer = stealer.clone();
            scope.spawn(move |_| {
                while let Some(task) = stealer.steal().success() {
                    println!("Processing: {}", task);
                }
            });
        }
    })
    .unwrap();
}

#[allow(dead_code)]
pub fn study() {
    work_stealing();
}
