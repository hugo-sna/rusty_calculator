use std::io;

fn main() {
    // Reading the user's input
    let mut input = String::new(); // Creating a new mutable string
    let stdin = io::stdin(); // Creating an instance of io::stdin

    println!("Enter an operation:");

    let _ = stdin.read_line(&mut input); // Read the line in the console (example:'26 + 8')

    let input = &input[0..input.len()-2]; // Remove the 2 latest chars (they're format chars)

    // Operation related variables
    // Create there so they can access the for loop
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut op: u8 = 0;
    let mut result: f32 = 0f32;

    let mut num_buffer = String::new(); // Store the numbers as a string

    for (i, chr) in input.chars().enumerate() {
        if chr == ' ' { continue; } // If i is a space skip this iteration

        if chr.is_numeric() { // Check if the char is a numeric
            num_buffer.push(chr); // Append the char to num_buffer
        } else {
            match chr { // Set right op id
                '+' => op = 1,
                '-' => op = 2,
                '*' => op = 3,
                '/' => op = 4,
                _ => panic!("Invalid operator ({})", chr)
            }

            if x == 0 {
                x = num_buffer.parse::<i32>().unwrap(); // Convert num_buffer to i32 and store it into x
                num_buffer.clear()
            }
        }
        if i == input.len() - 1 { // if it's last iteration
            y = num_buffer.parse::<i32>().unwrap(); // Convert num_buffer to i32 and store it into x
        }
    }

    // Make the operations
    match op {
        1 => result = x as f32 + y as f32,
        2 => result = x as f32 - y as f32,
        3 => result = x as f32 * y as f32,
        4 => result = x as f32 / y as f32,
        _ => ()
    }
    
    println!("{result}");
}
