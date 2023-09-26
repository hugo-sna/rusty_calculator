use std::io;

mod expression_parser;

use expression_parser::Expression;

fn main() {
    // Reading the user's input
    let mut input = String::new(); // Creating a new mutable string
    let stdin = io::stdin(); // Creating an instance of io::stdin

    println!("Enter an operation:");

    let _ = stdin.read_line(&mut input); // Read the line in the console (example:'26 + 8')

    // Remove the newline from the string
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
     // Store the numbers as a string
    let expression: Expression = expression_parser::parse(&input);

    let result: f32 = expression_parser::process(expression);
    
    println!("{result}");
}
