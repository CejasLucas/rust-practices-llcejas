use crate::sorting_algorithms::strategy::SortStrategy;
pub struct BubbleSort;

impl SortStrategy for BubbleSort {
    fn name(&self) -> &str { "BUBBLE" }

    fn sort(&self, arr: &mut [f64]) {
        self.print_header(arr);
        let n = arr.len();
        
        for i in 0..n {
            for j in 0..n - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                    println!("🔀 Swapped elements at indices {} and {}", j, j + 1);
                    println!("{:?}", arr);
                } 
            }
        }
    }
}