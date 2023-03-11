# if_number_divisible
Here's a Rust program that takes a user input number and finds all the numbers in the range from 0 to 1000000000 that can be divided by the user input without a remainder

This program prompts the user to enter a number, reads the input as a string, converts it to an unsigned 64-bit integer using the parse() method, and then loops through all the numbers from 1 to 1000000000 (inclusive). For each number in the loop, it checks if the number is divisible by the user input without a remainder using the modulo operator %. If the number is divisible, it prints the number to the console using println!().
