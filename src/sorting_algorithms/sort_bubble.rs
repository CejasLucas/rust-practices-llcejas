pub fn execution(arr: &mut [f64]) {
    let n = arr.len();
    
    println!();
    
    println!("---------------------------------------------------------------------------------");  
    
    println!("⚙️ BUBBLE SORT | Array is {:?}", arr);
   
    for _ in 0..n {
        for j in 0..n - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}