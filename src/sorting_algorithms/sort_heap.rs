use crate::sorting_algorithms::SortStrategy;

pub struct HeapSort;

impl SortStrategy for HeapSort {
    fn name(&self) -> &str { "HEAP" }

    fn sort(&self, arr: &mut [f64]) {
        self.print_header(arr);
        let n = arr.len();
        println!("--- Building the max heap ---");
        
        for i in (0..n / 2).rev() {
            println!("Building heap from index {}", i);
            Self::heapify(arr, n, i);
            println!("Heap after processing index {}: {:?}", i, arr);
        }

        println!("--- Extracting elements from the heap ---");

        for i in (1..n).rev() {
            println!("Swapping root element {} with element at index {} ({})", arr[0], i, arr[i]);
            arr.swap(0, i);
            println!("Array after swap: {:?}", arr);
            Self::heapify(arr, i, 0);
            println!("Heap after heapify: {:?}", arr);
        }
    }
}

impl HeapSort {
    fn heapify(arr: &mut [f64], heap_size: usize, root_index: usize) {
        let mut largest = root_index;
        let left = 2 * root_index + 1;
        let right = 2 * root_index + 2;

        println!(
            "Heapifying at index {} (value: {}), heap size: {}",
            root_index, arr[root_index], heap_size
        );

        if left < heap_size && arr[left] > arr[largest] {
            println!(
                "Left child {} (value: {}) is greater than root {} (value: {})",
                left, arr[left], largest, arr[largest]
            );
            largest = left;
        }

        if right < heap_size && arr[right] > arr[largest] {
            println!(
                "Right child {} (value: {}) is greater than current largest {} (value: {})",
                right, arr[right], largest, arr[largest]
            );
            largest = right;
        }

        if largest != root_index {
            println!(
                "Swapping elements at indices {} and {} => {} <-> {}",
                root_index, largest, arr[root_index], arr[largest]
            );
            arr.swap(root_index, largest);
            println!("Array after swap: {:?}", arr);
            Self::heapify(arr, heap_size, largest);
        } else {
            println!("  No swap needed at index {}", root_index);
        }
    }
}