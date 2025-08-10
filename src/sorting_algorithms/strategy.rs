use crate::utils::format_space;
pub trait SortStrategy {
    fn name(&self) -> &str;

    fn sort(&self, arr: &mut [f64]);
    
    fn print_header(&self, arr: &mut [f64]) {
        println!();
        format_space::space("=", 100);
        println!("⚙️ SORTING ALGORITHMS - {} SORT", self.name());
        println!("Original array = {:?} \n", arr);
    }
}