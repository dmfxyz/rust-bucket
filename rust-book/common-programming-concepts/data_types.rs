fn main(){
    // ** Scalar types ** //
    let int: i32 = -1;

    // Integer overflow
    // In debug mode, overflow is checked. Wrapping happens in release mode.
    let overflow: u8 = 0xff;
    println!("Overflow: {}", overflow);
    let overflow = overflow.saturating_add(1);
    println!("Saturated Overflow: {}", overflow);
    let overflow = overflow.wrapping_add(1);
    println!("Wrapping Overflow: {}", overflow);
    let overflow = match 0xff_u8.checked_add(1) {
        Some(x) => x,
        None => panic!("Overflow"),
    };
    println!("Checked Overflow: {}", overflow);

    // Integer Division
    let floored = 9 / 6;
    println!("Floored: {}", floored);

    // Floating point types
    // f32 and f64 are the default floating point types in Rust.

    // Boolean types
    let t = true;
    println!("T is {}, and has size {} bytes", t, std::mem::size_of_val(&t)); 

    // Character types
    // Are 4 bytes, respresent Unicode scalar values.
    let c = 'Z'
}