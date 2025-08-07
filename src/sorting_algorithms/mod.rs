pub mod menu;
pub mod sort_bubble;
pub mod sort_selection;
pub mod sort_insertion;
pub mod sort_merge;
pub mod sort_quick;
pub mod sort_heap;

pub trait SortStrategy {
    fn name(&self) -> &str;

    fn sort(&self, arr: &mut [f64]);
    
    fn print_header(&self, arr: &mut [f64]) {
        println!();
        println!("---------------------------------------------------------------------------------");
        println!("⚙️ SORTING ALGORITHMS - {} SORT", self.name());
        println!("Original array = {:?} \n", arr);
    }
}