use std::collections::HashMap;

pub fn get_mode(origin: &Vec<i32>) -> Option<i32> {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for num in origin {
        counter
            .entry(*num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut max_count = 1;
    let mut mode = origin[0];
    for (num, occurrence) in counter.iter() {
        if *occurrence > max_count {
            max_count = *occurrence;
            mode = *num;
        }
    }
    Some(mode)
}
