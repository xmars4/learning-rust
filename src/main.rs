use std::io;

fn main() {
    convert_temperature()
}

fn convert_temperature(){
    println!("Welcome to temperature converter.");
    println!("1. Convert C to F");
    println!("2. Convert F to C");
    println!("3. Exit");
    let mut value: f32;
    let mut result:f32;
        
    loop {
        let mut option = String::new();
        println!("Please select an options (1, 2 or 3)");
        io::stdin()
            .read_line(&mut option)
            .expect("Falied to read line");
        
        let option:u8 = match option.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if option == 1 {
            value = get_temperature_from_user_input();
            result = convert_c_to_f(value);
            println!("{}°C = {}°F", value, result)
        } else if option == 2{
            value = get_temperature_from_user_input();
            result = convert_f_to_c(value);
            println!("{}°C = {}°F", value, result)
        } else if option == 3{
            break;
        } else {
            continue;
        }
    }
}

fn get_temperature_from_user_input() -> f32{
    let mut value = String::new();
    println!("> ");
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    let value: f32 = match value.trim().parse(){
        Ok(num) => num,
        Err(_) => 0.0,
    };
    return value;
}

fn convert_c_to_f(c_temp: f32) -> f32 {
    c_temp * 9.0 / 5.0 + 32.0
}

fn convert_f_to_c(f_temp: f32) -> f32 {
    (f_temp - 32.0) * 5.0 / 9.0
}
