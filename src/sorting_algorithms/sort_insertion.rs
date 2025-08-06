pub fn execution(arr: &mut [f64]) {
    println!();
    
    println!("---------------------------------------------------------------------------------");  
    
    println!("⚙️ INSERTION SORT | Array is {:?}", arr);
   
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}