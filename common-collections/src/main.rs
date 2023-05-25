use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let a = vec![1, 99, 2, 6, 3];
    let b = a.to_vec();
    println!("{:?}", get_median(&a));
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
