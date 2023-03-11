use std::io;

//This program prompts the user to enter a number, reads the input
//as a string, converts it to an unsigned 64-bit integer using the parse() method,
//and then loops through all the numbers from 1 to 1000000000 (inclusive).
//For each number in the loop, it checks if the number is divisible by the user input
//without a remainder using the modulo operator %. If the number is divisible,
//it prints the number to the console using println!().

fn main() {

//the program takes a user input number
    println!("Enter a number:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input: u64 = user_input.trim().parse().expect("Please enter a valid number");

//and finds all the numbers in the range from 0 to 1000000000 that can be divided by the user input without a remainder
    
    println!("There are all numbers in the range from 0 to 1000000000 that can be divided by your number without a remainder:");
    
    for i in 1..=1000000000 {
        if user_input % i  == 0 {
            println!("{}", i);
        }
    }
}
