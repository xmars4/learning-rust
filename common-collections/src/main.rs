pub mod handle_str;
pub mod handle_vector;
use handle_str::pig_latin;
use handle_str::text_interface;
use handle_vector::median;
use handle_vector::mode;
use regex::{Regex, RegexBuilder};

fn main() {
    // let a = vec![1, 1, 7, 4, 12, 9, 2, 6, 3];
    // println!("{:?}", median::get_median(&a));
    // println!("{:?}", mode::get_mode(&a));

    // let text = String::from("hi my old friend p");
    // println!("{}", pig_latin::convert_text_to_pig_latin(&text));

    let statement = String::from("AdD mex to y");
    let mat = Regex::new(r"to").unwrap().find(&statement).unwrap();
    let start_to = mat.start();
    let employee_name = &statement[4..start_to - 1];
    let department_name = &statement[start_to + 3..];
    println!("{} {}", employee_name, department_name);
    // part_with_data

    // println!("{}", text_interface::add_employee_to_department(&statement));
    // text_interface::add_employee_to_department(&statement)
}
