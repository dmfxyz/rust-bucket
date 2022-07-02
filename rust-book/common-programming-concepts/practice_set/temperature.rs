use std::io;

fn main(){
    println!("Enter the temperature to convert: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    println!("You entered: {}", input);
    let units = input.trim().chars().last().unwrap();
    let temp: i32 = {
        let mut chars = input.trim().chars();
        chars.next_back();
        match chars.as_str().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input: {}", input);
                return;
            }
        }
    };
    
    match units.to_lowercase().next() {
        Some('c') => println!("{}C is {}F", temp, (temp * 9 / 5) + 32),
        Some('f') => println!("{}F is {}C", temp, (temp - 32) * 5 / 9),
        _ => println!("Invalid uints: {}", units),
    }
}