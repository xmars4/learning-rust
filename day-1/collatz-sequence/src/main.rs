/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut count: u32 = 0;
    loop {
        if n == 1 {
            return count;
        } else if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        };
        count += 1;
    }
}

fn main() {
    let num = collatz_length(27);
    println!("{num}");
}
