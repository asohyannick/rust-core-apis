mod greet;
mod users;

fn main() {
    greet::greet();
    users::users();

    let x: i32 = 5;
    let y: f64 = x as f64;
    println!("{}", y);

    let user: (&str, u32, bool) = ("Alice", 40, true);
    /*
        Compound Types — Tuples & Arrays
        Tuples — fixed size, mixed types (no direct Java equivalent)
    */
    // Access by index
    println!("Name: {}", user.0);
    println!("Age: {}", user.1);

    // Or destructure
    let (name, age, active) = user;
    println!("{} is {}, active: {}", name, age, active);

    /*
      Arrays — fixed size, single type (like Java arrays, but size is part of the type)
    */
    let scores: [i32; 5] = [10, 20, 30, 40, 50];
    let persons: [&str; 5] = ["James", "John", "Peter", "Prince", "Joy"];

    // Initialize with same value
    let _zeros = [0; 10]; // prefixed with _ to suppress unused variable warning

    println!("{:?}", scores);   // debug format required for arrays
    println!("{:?}", persons);  // debug format required for arrays

    const  MAX_CONNECTION: u32 = 100; // These are constants in Rust
    const APP_NAME: &str = "my-api";
    println!("{}", MAX_CONNECTION);
    println!("{}", APP_NAME);
}