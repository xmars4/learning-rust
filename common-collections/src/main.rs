pub mod handle_vector;
use handle_vector::median;
use handle_vector::mode;

fn main() {
    let a = vec![1, 1, 7, 4, 12, 9, 2, 6, 3];
    println!("{:?}", median::get_median(&a));
    println!("{:?}", mode::get_mode(&a));
}
