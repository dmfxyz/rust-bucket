fn main() {
    // If we want to use a variable in a function without transferring ownership, we can use a reference.
    let s = String::from("hello");
    let len = calculate_len(&s); // because calculate_len doesn't own s, it's not dropped after the function returns
    println!("The lenght of {} is {}", s, len);

    // We can also pass mutable references
    let mut my_str = String::from("hello");
    add_goodbyte(&mut my_str);
    println!("my_str is {}", my_str);

    // Note! You cannot have more than one reference to a mutible value
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // This prevents possible data races at compile time
}   

fn calculate_len(s: &String) -> usize{
    s.len()
}

fn add_goodbyte(s: &mut String){
    s.push_str(", goodbyte!");
}