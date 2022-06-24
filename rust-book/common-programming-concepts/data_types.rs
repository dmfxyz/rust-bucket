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
        None => 0
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
    let c = 'Z';

    // ** Compound types ** //
    // Tuples are a collection of values that may have different types
    let tuple = (420, "hello", "false");
    println!("{:?}", tuple);
    let (x, y, z) = tuple;
    println!("x is {}, y is {}, z is {}", x, y, z);
    println!("tuple.1 is {}", tuple.1);

    // Arrays are collections of values that all have the same type
    // Arrays in rust have a fixed length
    // Arrays allocate on the stack rather than the heap
    // <Vectors> are arrays that can grow and shrink
    // OOB references will cause an immediate panic
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    let arr: [&str; 3] = ["bye";3];
    println!("{:?}", arr);
}