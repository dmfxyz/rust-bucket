fn main() {
    println!("Hello, world!");
    goodbye_world();
    bed_time(9);
    s_and_p();
    println!("{}", returns_an_u32());
}

// functions use snake case by convention
fn goodbye_world() {
    println!("Goodbye, world!");
}

// function signatures REQUIRE type annotations
fn bed_time(time: u8){
    println!("Bedtime is {}", time);
}

// Statements and expressions
// statements perform an action and do not return a value
// expressions evaluate to a resulting value
fn s_and_p(){
    let y = 5; // statement 
    // let x = (let y = 5) invalid, y is not an expression

    let x = {
        let y = 5;
        y + 1 // expression. Note that there's no semicolon. Semicolons turn the expression into a statement.
    };
    println!("x is {:?}", x);
}

// Return values
fn returns_an_u32() -> u32 {
   return 5+5;
}
