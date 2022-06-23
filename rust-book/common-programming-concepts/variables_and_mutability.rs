// Must explicitly set type for constants
const ONE_YEAR: u32 = 525_600;

fn main() {
    let mut x = 5;
    //let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("{} minutes", ONE_YEAR);

    // shadowing and scoping
    let z = 3.14;
    println!("The value of z is: {}", z);
    {
        let z = 2.718;
        println!("The value of z is: {}", z);
    }
    println!("The value of z is: {}", z);

    let beta = "hello";
    println!("The value of beta is: {}", beta);
    {
        let beta = 420;
        println!("The value of beta is: {}", beta);
    }

    let mut rho = 0.9;
    // rho is mutable but you can't mutate the type of rho
    //rho = "hello";
}