use std::thread;
use std::time::Duration;
use std::sync::{Arc, Condvar, Mutex};

#[derive(Clone, Copy, PartialEq)]
enum TrafficLight {
    GreenForCars,
    GreenForPedestrians,
}

pub fn run() {
    let state = Arc::new((Mutex::new(TrafficLight::GreenForCars), Condvar::new()));

    // Spawn pedestrian threads
    for i in 0..5 {
        let state_clone = Arc::clone(&state);
        thread::spawn(move || {
            // Simulate staggered arrival
            thread::sleep(Duration::from_millis(500 * i as u64));
            let (lock, cvar) = &*state_clone;
            let mut light = lock.lock().unwrap();

            println!("🚶 Pedestrian {i} arrived at the crossing.");

            // Wait until the light is green for pedestrians (i.e., white pedestrian light)
            while *light != TrafficLight::GreenForPedestrians {
                println!("⏳ Pedestrian {i} waiting for WHITE light to cross...");
                light = cvar.wait(light).unwrap();
            }

            // Simulate crossing
            println!("🚶 Pedestrian {i} is crossing...");
            thread::sleep(Duration::from_secs(1));
            println!("✅ Pedestrian {i} has crossed safely.");
        });
    }

    // Traffic light controller thread
    let controller = Arc::clone(&state);
    thread::spawn(move || {
        let (lock, cvar) = &*controller;

        for cycle in 0..3 {
            thread::sleep(Duration::from_secs(4)); // Wait between light changes
            let mut light = lock.lock().unwrap();

            // Toggle the light state
            *light = match *light {
                TrafficLight::GreenForCars => TrafficLight::GreenForPedestrians,
                TrafficLight::GreenForPedestrians => TrafficLight::GreenForCars,
            };

            // Display current light state
            let message = match *light {
                TrafficLight::GreenForPedestrians => {
                    "⚪ WHITE light for PEDESTRIANS (🔴 RED light for cars)"
                }
                TrafficLight::GreenForCars => {
                    "🟢 GREEN light for CARS (🚷 Pedestrians must wait)"
                }
            };

            println!("\n======= CYCLE {} =======", cycle + 1);
            println!("{message}\n");

            cvar.notify_all(); // Wake up all waiting pedestrians
        }
    })
    .join()
    .unwrap();

    // Allow time for remaining threads to finish
    thread::sleep(Duration::from_secs(10));
}