// My, very naive, implementation of using a thread-pool to share work in parallel.
// Uses the thread-pool in lib.rs (from https://www.youtube.com/watch?v=2mwwYbBRJSo)
// Finds sum(sine(1), sine(2), sine(3) ... sine(n)), where n = 50000000.

extern crate num_cpus;

use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use threadpool::ThreadPool;

fn main() {
    // Count logical cores this process could try to use.
    let num_cores = num_cpus::get();

    println!("number of logical cores available: {}\n", num_cores);
    println!("Find sum(sine(1), sine(2), sine(3) ... sine(n)), where n = 50000000\n");

    // Infinte loop: Ctrl-C to break & exit.
    loop {
        // Get number of threads for thread-pool from user input.
        let num_threads = get_num_threads();

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
            let default_task_size = 50000000 / num_threads;
            let start = default_task_size * task_index;
            let mut result: f64 = 0.0;

            if task_index == num_threads - 1 {
                for i in start + 1..50000001 {
                    result += (i as f64).sin();
                }
            } else {
                for i in start + 1..(default_task_size * (task_index + 1)) + 1 {
                    result += (i as f64).sin();
                }
            }

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
        println!("{:?} ms\n", elapsed.as_millis());
    }
}

fn get_input() -> String {
    let mut s = String::new();
    print!("Split calculation across how many threads?\nEnter an integer from 1 to 12: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}

fn get_num_threads() -> u32 {
    let mut i: u32 = 127;

    while i > 12 {
        let input = get_input();

        i = match input.parse::<u32>() {
            Ok(i) => {
                if i > 0 && i < 13 {
                    i
                } else {
                    println!("\nERROR! Try again");
                    127
                }
            }
            Err(_) => {
                println!("\nERROR! Try again");
                127
            }
        };
    }

    i
}
