use std::io;

fn main() {
    println!("which nth fibonacci number you want to print > ");

    let mut nth = String::new();
    let mut count = 0;

    io::stdin()
    .read_line(&mut nth)
    .expect("Can't read input data");

    let nth:u8 = match nth.trim().parse(){
        Ok(num) => num,
        Err(_) => 1,
    };
    let mut previous_1 = 0;
    let mut previous_2 = 1;
    let mut next;

    while count != nth {
        count += 1;
        if count == 1 {
            print!("0 ");
        }
        else if count == 2 {
            print!("1 ");
        } else {
            next = previous_1 + previous_2;
            previous_1 = previous_2;
            previous_2 = next;
            print!("{} ", next);
        }
    }
    println!()
    
}
