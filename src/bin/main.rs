// My, very naive, implementation of using a thread-pool to share work in parallel.
// Uses the thread-pool in lib.rs (from https://www.youtube.com/watch?v=2mwwYbBRJSo)
// Finds sum(sine(1), sine(2), sine(3) ... sine(n)), where n = 50_000_000.

extern crate num_cpus;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use threadpool::ThreadPool;

fn main() {
    // Count logical cores this process could try to use.
    let num_cores = num_cpus::get();

    println!("number of logical cores available: {}", num_cores);
    println!("Spawning 6 threads (total = 7, including main)");
    println!("Find sum(sine(1), sine(2), sine(3) ... sine(n)), where n = 50_000_000\n");

    let num_threads = 6;

    let start_time = Instant::now();

    // Counter used to divide work between threads.
    let thread_counter = Arc::new(AtomicU32::new(0));
    let thread_counter_clone = thread_counter.clone();
    // Counter used to register thread completion(s).
    let completed_counter = Arc::new(AtomicU32::new(0));
    let completed_counter_clone = completed_counter.clone();

    // Mutex 64 bit float for threads to add result(s) to.
    let sum: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));
    let sum_clone = sum.clone();

    // Set up a thread-pool with the number of threads specified by user input.
    let pool = ThreadPool::new(num_threads as u8);

    // Closure containing the work to be done in each thread.
    let foo = move || {
        let task_index = thread_counter_clone.fetch_add(1, Ordering::SeqCst);

        let result = task(task_index, num_threads);

        let mut sum = sum_clone.lock().unwrap();
        *sum += result;
        completed_counter_clone.fetch_add(1, Ordering::SeqCst);
    };

    // Start the threads, passing in the closure containing the work for each.
    for _ in 0..num_threads {
        pool.execute(foo.clone());
    }

    let mut completed_threads_count = completed_counter.load(Ordering::SeqCst);

    // Loop until every spawned thread reports completion of allocated task.
    while completed_threads_count < num_threads {
        completed_threads_count = completed_counter.load(Ordering::SeqCst);
    }

    let elapsed = start_time.elapsed();

    println!("sum = {:.6}", sum.lock().unwrap());
    println!("Threadpool: {:?} ms\n", elapsed.as_millis());

    run_rayon();
}

fn task(task_index: u32, num_threads: u32) -> f64 {
    let default_task_size = 50000000 / num_threads;
    let start = default_task_size * task_index; // This method can be adjusted to set various vars for each thread. E.g. name of face (might be only thing needed!)
    let mut result: f64 = 0.0;

    if task_index == num_threads - 1 {
        for i in (start + 1)..50000001 {
            result += (i as f64).sin();
        }
    } else {
        for i in (start + 1)..(default_task_size * (task_index + 1)) + 1 {
            result += (i as f64).sin();
        }
    }

    result
}

fn run_rayon() {
    let start_time = Instant::now();

    // run task in parallel using rayon
    let mut result_a: f64 = 0.0;
    let mut result_b: f64 = 0.0;
    let mut result_c: f64 = 0.0;
    let mut result_d: f64 = 0.0;
    let mut result_e: f64 = 0.0;
    let mut result_f: f64 = 0.0;
    rayon::scope(|s| {
        s.spawn(|_| result_a = task(0, 6));
        s.spawn(|_| result_b = task(1, 6));
        s.spawn(|_| result_c = task(2, 6));
        s.spawn(|_| result_d = task(3, 6));
        s.spawn(|_| result_e = task(4, 6));
        s.spawn(|_| result_f = task(5, 6));
    });
    let sum = result_a + result_b + result_c + result_d + result_e + result_f;

    println!("sum = {:.6}", sum);
    let elapsed = start_time.elapsed();
    println!("Rayon: {:?} ms\n", elapsed.as_millis());
}
