pub mod handle_str;
pub mod handle_vector;
use handle_str::pig_latin;
use handle_vector::median;
use handle_vector::mode;

fn main() {
    let a = vec![1, 1, 7, 4, 12, 9, 2, 6, 3];
    println!("{:?}", median::get_median(&a));
    println!("{:?}", mode::get_mode(&a));

    let text = String::from("hi my old friend p");
    println!("{}", pig_latin::convert_text_to_pig_latin(&text));
}
