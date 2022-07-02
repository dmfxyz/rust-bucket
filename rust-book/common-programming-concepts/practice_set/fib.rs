use std::io;

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2),
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let n: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input an integer > 0.{} is not an integer > 0", input);
            return;
        }
    };
    println!("{}", fib(n));
}