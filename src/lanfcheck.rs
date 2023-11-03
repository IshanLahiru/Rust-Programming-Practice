use std::io;

fn main() {
  println!("Enter a number: ");

  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  // Parse the input into an integer
  let number: i32 = input.trim().parse().expect("Invalid input");

  if number > 5 {
    println!("The number is greater than 5.");
  } else {
    println!("The number is not greater than 5.");
  }
}
