fn main() {
    let x = 5;
    // Conditions must be bools
    if x > 5 {
        println!("Big");
    } else if x > 3{
        println!("Medium");
    } else {
        println!("Small");
    }
// can't do don't have the dependency on rand
// let y = if Rng.thread_rng().gen_range(1..10) % 2 == 0 {"even"} else {"odd"};
//     println!("{}", y);
    let y = if x % 2 == 0 {"even"} else {"odd"};
    println!("{}", y);

    // Loops and other structions
    let mut i = 0; 
    let t = loop {  // loops are expressions!
        println!("{}", i);
        i += 1;
        if i == 10 {
            break i
        }
    };
    println!("The value of t is: {}", t);
    i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for num in a {
        println!("{}", num);
    }

    // ranges
    for num in 1..4{
        println!("{}", num);
    }


}