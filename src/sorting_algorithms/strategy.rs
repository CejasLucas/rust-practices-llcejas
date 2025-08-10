pub trait SortStrategy {
    fn name(&self) -> &str;

    fn sort(&self, arr: &mut [f64]);
    
    fn print_header(&self, arr: &mut [f64]) {
        println!("\n{}", "-".repeat(100));
        println!("⚙️ SORTING ALGORITHMS - {} SORT", self.name());
        println!("Original array = {:?} \n", arr);
    }
}