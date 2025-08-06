pub fn execution(arr: &mut [f64]) {
    println!("\n---------------------------------------------------------------------------------");
    
    println!("⚙️  SELECTION SORT | Original array: {:?}", arr);
    
    let n = arr.len();
    for i in 0..n {
        let mut min_index = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}