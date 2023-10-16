fn main() {
    // Scalar Types
    let x: i32 = 42;
    let y: u64 = 100;
    let a: f32 = 3.14;
    let b: f64 = 2.71828;
    let is_rust_fun: bool = true;
    let heart_emoji: char = '❤';

    // Compound Types
    let person: (String, i32, bool) = ("Alice".to_string(), 30, true);
    let numbers: [i32; 3] = [1, 2, 3];

    // Printing variables to the terminal
    println!("Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    println!("Single Precision Float: {}", a);
    println!("Double Precision Float: {}", b);
    println!("Is Rust Fun? {}", is_rust_fun);
    println!("Heart Emoji: {}", heart_emoji);

    // Destructuring a tuple
    let (name, age, is_happy) = person;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Happy? {}", is_happy);

    // Accessing array elements
    println!("Array Element 1: {}", numbers[0]);
    println!("Array Element 2: {}", numbers[1]);
    println!("Array Element 3: {}", numbers[2]);
}
