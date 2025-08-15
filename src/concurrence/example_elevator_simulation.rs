use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
};

const NUM_PEOPLE: usize = 5;

#[derive(Debug)]
struct Request { person_id: usize, floor: u32 }

pub fn run() {
    let requests = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
    let finished = Arc::new(AtomicBool::new(false));

    // Spawn people threads to generate requests
    let mut handles = Vec::new();
    for i in 0..NUM_PEOPLE {
        let reqs = Arc::clone(&requests);
        let handle = thread::spawn(move || {
            let floor = (i as u32 + 1) * 2;
            let request = Request {
                person_id: i,
                floor,
            };
            let (lock, cvar) = &*reqs;
            let mut queue = lock.lock().unwrap();
            println!("👤 Person {} requests floor {}", i, floor);
            queue.push_back(request);
            cvar.notify_one();
        });
        handles.push(handle);
    }

    // Wait for all request threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Mark that no more requests will come
    finished.store(true, Ordering::SeqCst);
    // Wake elevator thread in case it is waiting
    {
        let (lock, cvar) = &*requests;
        let _queue = lock.lock().unwrap();
        cvar.notify_one();
    }

    // Elevator thread processes requests until finished & queue empty
    let elevator = {
        let reqs = Arc::clone(&requests);
        let finished = Arc::clone(&finished);

        thread::spawn(move || {
            let (lock, cvar) = &*reqs;
            let mut current_floor = 0;

            loop {
                let mut queue = lock.lock().unwrap();

                while queue.is_empty() {
                    if finished.load(Ordering::SeqCst) {
                        println!("🎉 Elevator has completed all requests.");
                        return;
                    }
                    queue = cvar.wait(queue).unwrap();
                }

                if let Some(request) = queue.pop_front() {
                    println!(
                        "\n🚪 Elevator moving from floor {} to floor {} (requested by person {})",
                        current_floor, request.floor, request.person_id
                    );
                    let target_floor = request.floor as i32;
                    let mut floor_pos = current_floor as i32;

                    while floor_pos != target_floor {
                        if target_floor > floor_pos {
                            floor_pos += 1;
                        } else {
                            floor_pos -= 1;
                        }
                        print!("\r🚪 Elevator at floor {}  ", floor_pos);
                        use std::io::{stdout, Write};
                        stdout().flush().unwrap();
                        thread::sleep(Duration::from_millis(500));
                    }
                    current_floor = target_floor as u32;

                    println!("\n✅ Arrived at floor {} for person {}", current_floor, request.person_id);
                }
            }
        })
    };
    elevator.join().unwrap();
}