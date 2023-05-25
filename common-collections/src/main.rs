use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let a = vec![1, 1, 3, 4, 5, 3, 2, 6, 3];
    let b = a.to_vec();
    println!("{:?}", get_median(&a));

    let mut counter: HashMap<i32, i32> = HashMap::new();
    println!("{:?}", get_mode(&a));
}

fn get_median(origin: &Vec<i32>) -> Option<i32> {
    if origin.is_empty() {
        return None;
    }
    let mut copy = origin.to_vec();
    copy.sort_unstable();
    let vec_len = copy.len();
    if vec_len % 2 != 0 {
        return Some(copy[vec_len / 2]);
    }
    return Some(copy[vec_len / 2 - 1]);
}

fn get_mode(origin: &Vec<i32>) -> Option<i32> {
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
