use std::time::Instant;
use crate::utils::assistant;
use crate::sorting_algorithms::sort_bubble;
use crate::sorting_algorithms::sort_merge;
use crate::sorting_algorithms::sort_selection;
use crate::sorting_algorithms::sort_insertion;
use crate::sorting_algorithms::sort_quick;
use crate::sorting_algorithms::sort_heap;


pub fn menu() {
    let mut arr = [15.75, -21.55, 83.25, -11.15, 75.05, 93.45, -57.10, 89.50];
    
    println!();  
    println!("==========================================");  
    println!("📂 SECONDARY MENU - SORTING ALGORITHMS");
    println!("↪︎ Select an option.");
    println!("1. Bubble Sort");
    println!("2. Selection Sort");
    println!("3. Insertion Sort");
    println!("4. Merge Sort");
    println!("5. Quick Sort");
    println!("6. Heap Sort");

    let choice = assistant::read_u32("Enter your choice (1-6): ");
      
    let start_time = Instant::now();
    
    match choice {
        1 => sort_bubble::execution(&mut arr),
        2 => sort_selection::execution(&mut arr),
        3 => sort_insertion::execution(&mut arr),
        4 => sort_merge::execution(&mut arr),
        5 => sort_quick::execution(&mut arr),
        6 => sort_heap::execution(&mut arr),
        _ => { println!("Invalid choice. Please enter 1 to 6."); return; }
    }
   
    let elapsed = start_time.elapsed();

    println!("Sorted array: {:?}", arr);

    println!("Execution time: {:.6} seconds", elapsed.as_secs_f64());
}