use crate::sorting_algorithms::SortStrategy;
pub struct MergeSort;

impl SortStrategy for MergeSort {
    fn name(&self) -> &str { "MERGE" }

    fn sort(&self, arr: &mut [f64]) {
        self.print_header(arr);
        Self::merge_sort(arr, 0); // Start at depth 0
    }
}

impl MergeSort {
    fn merge_sort(arr: &mut [f64], depth: usize) {
        let len = arr.len();
        if len <= 1 { return; }
        let mid = len / 2;

        // Recursively sort both halves, increasing the depth
        Self::merge_sort(&mut arr[..mid], depth + 1);
        Self::merge_sort(&mut arr[mid..], depth + 1);

        let left_part = arr[..mid].to_vec();
        let right_part = arr[mid..].to_vec();

        println!("🔀 Merging (depth {}):", depth);
        println!(" Left : {:?}", left_part);
        println!(" Right: {:?}", right_part);

        // Merge the two sorted halves
        let mut merged = Vec::with_capacity(len);
        let (mut left, mut right) = (0, 0);

        while left < left_part.len() && right < right_part.len() {
            if left_part[left] <= right_part[right] {
                merged.push(left_part[left]);
                left += 1;
            } else {
                merged.push(right_part[right]);
                right += 1;
            }
        }

        merged.extend_from_slice(&left_part[left..]);
        merged.extend_from_slice(&right_part[right..]);
        
        arr.copy_from_slice(&merged);
        println!("→ Result: {:?}", arr);
    }
}