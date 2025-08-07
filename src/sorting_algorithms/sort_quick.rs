use crate::sorting_algorithms::SortStrategy;

pub struct QuickSort;

impl SortStrategy for QuickSort {
    fn name(&self) -> &str { "QUICK" }

    fn sort(&self, arr: &mut [f64]) {
        self.print_header(arr);
        Self::quick_sort(arr);
    }
}

impl QuickSort {
    fn partition(arr: &mut [f64]) -> usize {
        let mut i = 0;
        let pivot = arr[arr.len() - 1];
        println!("\n🔹 Partitioning: {:?} | Pivot: {}", arr, pivot);

        for j in 0..arr.len() - 1 {
            if arr[j] <= pivot {
                println!(
                    "arr[{}] (value: {}) <= pivot (value: {}) → swapping with arr[{}] (value: {})",
                    j, arr[j], pivot, i, arr[i]
                );
                arr.swap(i, j);
                println!("Array after swap: {:?}", arr);
                i += 1;
            } else {
                println!("arr[{}] (value: {}) > pivot → no swap", j, arr[j]);
            }
        }

        println!(
            "  Swapping pivot with element at index {} → arr[{}] <-> arr[{}]",
            i,
            i,
            arr.len() - 1
        );
        arr.swap(i, arr.len() - 1);
        println!("✅ Partitioned array: {:?}", arr);

        i // Return pivot index
    }

    fn quick_sort(arr: &mut [f64]) {
        if arr.len() <= 1 {
            return;
        }

        println!("\n📦 Quick sorting subarray: {:?}", arr);

        let pivot_index = Self::partition(arr);

        let (left, right) = arr.split_at_mut(pivot_index);
        let right = &mut right[1..]; // Exclude pivot

        println!("⬅️ Left side to sort: {:?}", left);
        println!("➡️ Right side to sort: {:?}", right);

        Self::quick_sort(left);
        Self::quick_sort(right);
    }
}