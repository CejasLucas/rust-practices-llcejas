use crate::sorting_algorithms::strategy::SortStrategy;
pub struct InsertionSort;

impl SortStrategy for InsertionSort {
    fn name(&self) -> &str { "INSERTION" }

    fn sort(&self, arr: &mut [f64]) {
        self.print_header(arr);
        for i in 1..arr.len() {
            let key = arr[i];
            let mut j = i;

            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1];
                j -= 1;
            }

            if j != i {
                arr[j] = key;
                println!("🔀 Inserted value {} at index {}", key, j);
                println!("{:?}", arr);
            }
        }
    }
}