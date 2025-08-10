use std::time::Instant;
use crate::utils::format_input;
use crate::sorting_algorithms::{
    strategy::SortStrategy,
    sort_bubble::BubbleSort,
    sort_selection::SelectionSort,
    sort_insertion::InsertionSort,
    sort_merge::MergeSort,
    sort_quick::QuickSort,
    sort_heap::HeapSort
};

pub fn implementation() {
    let mut arr = [15.75, -21.55, 83.25, -11.15, 75.05, 93.45, -57.10, 89.50];
    
    println!("\n{}", "=".repeat(100));
    println!("📂 SECONDARY MENU - SORTING ALGORITHMS");
    println!("1. Bubble Sort");
    println!("2. Selection Sort");
    println!("3. Insertion Sort");
    println!("4. Merge Sort");
    println!("5. Quick Sort");
    println!("6. Heap Sort");
    println!("0. Exit");

    let choice = format_input::read_u32("Enter your choice (1-6): ");
    
    let strategy: Box<dyn SortStrategy> = match choice {
        1 => Box::new(BubbleSort),
        2 => Box::new(SelectionSort),
        3 => Box::new(InsertionSort),
        4 => Box::new(MergeSort),
        5 => Box::new(QuickSort),
        6 => Box::new(HeapSort),
        
        // Exit 
        0 => { println!("Exiting program."); return; }
        _ => { println!("Invalid option. Please enter a number from 0 to 6."); return; }
    };

    let start_time = Instant::now();
    strategy.sort(&mut arr);
    let elapsed = start_time.elapsed();

    println!("\nSorted array: {:?}", arr);
    println!("Execution time: {:.6} seconds", elapsed.as_secs_f64());
    println!("\n{}", "-".repeat(100));  
}