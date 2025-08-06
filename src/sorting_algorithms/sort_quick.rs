fn partition(arr: &mut [f64]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

fn quick_sort(arr: &mut [f64]) {
    if arr.len() <= 1 { return; }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

pub fn execution(arr: &mut [f64]) {

    println!("\n---------------------------------------------------------------------------------");
    
    println!("⚙️  QUICK SORT | Original array: {:?}", arr);
    
    quick_sort(arr);
}