fn main() {
    // s is in scope for the entirety of the main function
    // s is valid as long as it's in scope
    // s becomes invalid when the main function ends
    let s = "hello";

    // __Generally__, with limitations, variables of known size at compile are stored on the stack
    // Variables remain on the stack as long as they're owned and valid
    // But other variables or large variables are stored in the heap
    let mut s = String::from("hello"); // asks memory allocator for new heap memory for our string
    s.push_str(", world!"); // can mutate!
    println!("{}", s);
    // memory will be returned when the variable that owns s goes out of scope.
    // automatically does this by calling the drop function on the variable (implementor would need to write drop)

    // 5 is pushed to stack twice
    let x = 5;
    let y = x;

    // S1 and S2 are references to the same string in the heap, because they share the same ptr value
    // Strings are made up of ptr to heap, lengths, and capacity. these three are stored on stack.
    let s1 = String::from("goodbyte");
    let s2 = s1;

    // problem:: when s1 and s2 go out of scope, drop will be called twice. This is a double free error
    // Err! s1 has been "moved" to s2. Only s2 remains valid.
    //println!("{}", s1);
    println!("{}", s2);

    // Rust will never automatically make a deep copy of an object, but we can manually with .clone()
    let s3 = s2.clone();
    println!("{}", s3);
    println!("{}", s2);

    // If a type implements the "Copy" trait (like the ints x and y on line 17 and 18), variables do not move
    // and are trivially copied

    // When passed as arguments to functions, variables are moved or copied the same way as assignment
    let x = 11;
    double(x);
    println!("x is {}", x);
    let mut my_str = String::from("hello");
    add_goodbyte(my_str);
    //println!("my_str is {}", my_str); ERR! add_goodbyte has borrowed as my_str goes out of scope when it returns

    // functions with return values will return the ownership to the higher scope
    let mut my_str = String::from("hello");
    let my_str_2 = add_goodbyte_2(my_str);
    println!("my_str is {}", my_str_2);

}

fn double(x: u32) {
    println!("{}", x * 2);
}

fn add_goodbyte(mut s: String){
    s.push_str(", goodbyte!");
    println!("{}", s);
}

fn add_goodbyte_2(mut s: String) -> String{
    s.push_str(", goodbyte!");
    s.to_string()
}