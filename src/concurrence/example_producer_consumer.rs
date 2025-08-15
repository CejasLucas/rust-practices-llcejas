use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};

const BUFFER_SIZE: usize = 5;

pub fn run() {
    let buffer = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));

    // Clonamos buffer para productor
    let buffer_producer = Arc::clone(&buffer);

    let producer = thread::spawn(move || {
        for i in 0..10 {
            let (lock, cvar) = &*buffer_producer;
            let mut queue = lock.lock().unwrap();

            // Esperar si buffer está lleno
            while queue.len() == BUFFER_SIZE {
                queue = cvar.wait(queue).unwrap();
            }

            queue.push_back(i);
            println!(
                "🛠️ [Productor Thread] produce: {} (Buffer size: {})",
                i,
                queue.len()
            );
            cvar.notify_all();

            thread::sleep(Duration::from_millis(200));
        }
    });

    // Clonamos buffer para consumidor
    let buffer_consumer = Arc::clone(&buffer);

    let consumer = thread::spawn(move || {
        for _ in 0..10 {
            let (lock, cvar) = &*buffer_consumer;
            let mut queue = lock.lock().unwrap();

            // Esperar si buffer está vacío
            while queue.is_empty() {
                queue = cvar.wait(queue).unwrap();
            }

            if let Some(value) = queue.pop_front() {
                println!(
                    "📦 [Consumidor Thread] consume: {} (Buffer size: {})",
                    value,
                    queue.len()
                );
            }
            cvar.notify_all();

            thread::sleep(Duration::from_millis(300));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}