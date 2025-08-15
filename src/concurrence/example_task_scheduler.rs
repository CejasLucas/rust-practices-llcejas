use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Task { id: usize, duration: u64 }

pub fn run() {
    let (sender, receiver) = mpsc::channel::<Task>();
    let receiver = Arc::new(Mutex::new(receiver)); // Wrap in Mutex

    let num_workers = 3;
    let mut handles = Vec::with_capacity(num_workers);

    for i in 0..num_workers {
        let rx = Arc::clone(&receiver);
        let handle = thread::spawn(move || {
            loop {
                let task = {
                    let lock = rx.lock().unwrap();
                    lock.recv()
                };

                match task {
                    Ok(task) => {
                        println!("🔧 Worker {} | Executing task {}", i, task.id);
                        thread::sleep(Duration::from_secs(task.duration));
                        println!("✅ Worker {} | Finished task {}", i, task.id);
                    }
                    Err(_) => {
                        println!("💤 Worker {} | No more tasks. Exiting.", i);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for i in 0..6 {
        let task = Task {
            id: i,
            duration: (i % 3 + 1) as u64,
        };
        println!("📥 Sending task {}", task.id);
        sender.send(task).unwrap();
    }

    drop(sender); // Close channel

    for handle in handles {
        handle.join().unwrap();
    }

    println!("🏁 All workers have finished.");
}