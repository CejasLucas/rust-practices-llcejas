use crate::sorting_algorithms::strategy::SortStrategy;
pub struct QuickSort;

impl SortStrategy for QuickSort {
    fn name(&self) -> &str { "QUICK" }

    fn sort(&self, arr: &mut [f64]) {
        self.print_header(arr);
        Self::quick_sort(arr);
    }
}

impl QuickSort {
    fn quick_sort(arr: &mut [f64]) {
        if arr.len() <= 1 { return; }

        let pivot_index = Self::partition(arr);

        let (left, right) = arr.split_at_mut(pivot_index);
        let right = &mut right[1..]; // exclude pivot

        Self::quick_sort(left);
        Self::quick_sort(right);
    }

    fn partition(arr: &mut [f64]) -> usize {
        let mut i = 0;
        let pivot = arr[arr.len() - 1];

        for j in 0..arr.len() - 1 {
            if arr[j] <= pivot {
                if i != j {
                    arr.swap(i, j);
                    println!("\n🔀 Swapped indices {} and {} => {:?}", i, j, arr);
                }
                i += 1;
            }
        }

        if i != arr.len() - 1 {
            arr.swap(i, arr.len() - 1);
            println!("\n🔀  Swapped pivot to index {} => {:?}", i, arr);
        }

        i
    }
}