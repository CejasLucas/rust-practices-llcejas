use crate::sorting_algorithms::SortStrategy;
pub struct SelectionSort;

impl SortStrategy for SelectionSort {
    fn name(&self) -> &str { "SELECTION" }

    fn sort(&self, arr: &mut [f64]) {
        self.print_header(arr);
        let n = arr.len();
        
        for i in 0..n {
            let mut min_index = i;

            for j in (i + 1)..n {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }

            if min_index != i {
                println!("🔀 Swap positions: {} with {}", i, min_index);
                arr.swap(i, min_index);
                println!("{:?}", arr);
            }
        }
    }
}