use regex::{Regex, RegexBuilder};
// use std::collections::HashMap;

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

fn extract_value(value: &str) -> &str {
    let part_with_data = &value[3..];
    part_with_data
}

pub fn add_employee_to_department(statement: &str) -> bool {
    let valid_input = extract_valid_input(statement).unwrap_or(String::from("invalid"));
    if valid_input != "invalid" {
        false;
    }
    true
    // check_valid_input(statement)
}
