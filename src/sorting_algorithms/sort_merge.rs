fn merge_sort(arr: &mut [f64]) {
    let len = arr.len();
    if len <= 1 { return; }

    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged = Vec::with_capacity(len);
    let (mut left, mut right) = (0, mid);

    while left < mid && right < len {
        if arr[left] <= arr[right] {
            merged.push(arr[left]);
            left += 1;
        } else {
            merged.push(arr[right]);
            right += 1;
        }
    }

    merged.extend_from_slice(&arr[left..mid]);
    merged.extend_from_slice(&arr[right..]);
    arr.copy_from_slice(&merged);
}

pub fn execution(arr: &mut [f64]) {

    println!("\n---------------------------------------------------------------------------------");
    
    println!("⚙️  MERGE SORT | Original array: {:?}", arr);
    
    merge_sort(arr);
}