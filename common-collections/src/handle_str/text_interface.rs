use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

fn remove_redundant_spaces(value: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    let valid_value = re.replace_all(value, " ").trim().to_string();
    valid_value
}

fn extract_valid_input(value: &str) -> Option<String> {
    let processed_value = remove_redundant_spaces(value);
    let re = RegexBuilder::new(r"^add\s+.+to\s+.+$")
        .case_insensitive(true)
        .build()
        .unwrap();
    match re.is_match(&processed_value) {
        true => Some(processed_value.to_string()),
        false => None,
    }
}

pub fn extract_value(value: &str) -> Vec<&str> {
    let to_location = RegexBuilder::new(r"to")
        .case_insensitive(true)
        .build()
        .unwrap()
        .find(value)
        .unwrap();
    // let to_location = Regex::new(r"to").unwrap().find(value).unwrap();
    let start_to = to_location.start();
    let employee_name = &value[4..start_to - 1];
    let department_name = &value[start_to + 4..];
    vec![employee_name, department_name]
}

pub fn add_employee_to_department(
    statement: &str,
    department_data: &mut HashMap<String, Vec<String>>,
) -> bool {
    let valid_input = extract_valid_input(statement).unwrap_or(String::from("invalid"));
    if valid_input == "invalid" {
        return false;
    }
    let new_data = extract_value(statement);
    let employee_name = new_data[0].to_string();
    let department_name = new_data[1].to_string();
    if let Some(x) = department_data.get_mut(&department_name) {
        x.push(employee_name);
    } else {
        department_data.insert(department_name, vec![employee_name]);
    }
    return true;
}
