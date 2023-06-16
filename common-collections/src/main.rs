pub mod handle_str;
pub mod handle_vector;
use handle_str::pig_latin;
use handle_str::text_interface;
use handle_vector::median;
use handle_vector::mode;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

fn main() {
    // let a = vec![1, 1, 7, 4, 12, 9, 2, 6, 3];
    // println!("{:?}", median::get_median(&a));
    // println!("{:?}", mode::get_mode(&a));

    // let text = String::from("hi my old friend p");
    // println!("{}", pig_latin::convert_text_to_pig_latin(&text));
    let mut department_data: HashMap<String, Vec<String>> = HashMap::new();
    let statement = String::from("AdD pro to  sales");
    let statement2 = String::from("Add dddd none to  BOD");
    let statement3 = String::from("Add DEv-rust  TO  sales");
    let incorrect_statement3 = String::from("Add DEv-rust  TaaO  sales");

    text_interface::add_employee_to_department(&statement, &mut department_data);
    text_interface::add_employee_to_department(&statement2, &mut department_data);
    text_interface::add_employee_to_department(&statement3, &mut department_data);
    text_interface::add_employee_to_department(&incorrect_statement3, &mut department_data);
    println!("{:?}", department_data);

    // part_with_data

    // println!("{}", text_interface::add_employee_to_department(&statement));
    // text_interface::add_employee_to_department(&statement)
}
