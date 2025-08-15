use std::thread;
use std::sync::{Arc, Mutex};

const N_THREADS: usize = 10;
const INCREMENTS_PER_THREAD: usize = 100;

pub fn run() {
    let counter = Arc::new(Mutex::new(0));
    let print_lock = Arc::new(Mutex::new(())); // Mutex just to print in an orderly fashion

    let mut handles = vec![];

    for _ in 0..N_THREADS {
        let counter_clone = Arc::clone(&counter);
        let print_lock_clone = Arc::clone(&print_lock);
        let handle = thread::spawn(move || {
            for _ in 0..INCREMENTS_PER_THREAD {
                // Increment the counter
                let mut num = counter_clone.lock().unwrap();
                *num += 1;

                let thread_id = thread::current().id();

                // Lock the print to avoid mixing outputs in the console
                let _print_guard = print_lock_clone.lock().unwrap();
                println!("🧵 Thread {:?} 📈 increased to: {}", thread_id, *num);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = *counter.lock().unwrap();
    println!("🎯 Final counter: {}", final_count);
}