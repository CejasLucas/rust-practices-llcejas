use crate::sorting_algorithms::strategy::SortStrategy;
pub struct HeapSort;

impl SortStrategy for HeapSort {
    fn name(&self) -> &str { "HEAP" }

    fn sort(&self, arr: &mut [f64]) {
        self.print_header(arr);
        let n = arr.len();

        for i in (0..n / 2).rev() {
            Self::heapify(arr, n, i);
        }

        for i in (1..n).rev() {
            arr.swap(0, i);
            println!("\n🔀  Swapped root with index {} => {:?}", i, arr);
            Self::heapify(arr, i, 0);
        }
    }
}

impl HeapSort {
    fn heapify(arr: &mut [f64], heap_size: usize, root_index: usize) {
        let mut largest = root_index;
        let left = 2 * root_index + 1;
        let right = 2 * root_index + 2;

        if left < heap_size && arr[left] > arr[largest] {
            largest = left;
        }

        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        if largest != root_index {
            arr.swap(root_index, largest);
            println!(
                "\n🔀 Swapped elements at indices {} and {} => {:?}",
                root_index, largest, arr
            );
            Self::heapify(arr, heap_size, largest);
        }
    }
}