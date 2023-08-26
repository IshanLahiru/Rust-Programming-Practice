use std::io;

fn main(){
    //create a new instance of the standard input stream
    let stdin = io::stdin();

    //print a massage to prompt the user input
    println!("Please enter something:");

    //create a mutable string to store the user's input
    let mut input = String::new();

    //Read the user's input and store it in the string
    stdin.read_line(&mut input).expect("Failed to read the li9ne");

    //prinnt the user's input
    println!("you entered: {}",input);
    
}