//use std::io;
use sha2::{Sha256, Digest};
use std::io::{self, Write};

extern "C" {
    fn cFoo();
}

fn main() {
    
    //Testing basic I/O--------------------------------------------------
    println!("Enter two numbers to add\nnum_1 (Rust): ");
    let mut user_in_num1 = String::new(); //string variable for holding user input
    let mut user_in_num2 = String::new();

    

    io::stdin().read_line(&mut user_in_num1).expect("Failed to read input"); //storing user input in variable
    println!("num2 (Rust): ");
    io::stdin().read_line(&mut user_in_num2).expect("Failed to read second input");

    let user_in_num1_int: i32 = user_in_num1.trim().parse().unwrap(); //convert user string to int
    let user_in_num2_int: i32 = user_in_num2.trim().parse().unwrap();
    
    let sum = user_in_num1_int + user_in_num2_int; //perform addition calculation
    println!("The sum is {}", sum);
    
    println!("Calling C code from Rust...");
    //---------------------------------------------------------------------


    //Hashing in using SHA-256 in Rust------------------------------------------------------
    let mut input_string: String = String::new();
    println!("Enter your user password: ");

    io::stdin().read_line(&mut input_string).unwrap();
    let input_string = input_string.trim();

    let mut hasher = Sha256::new();
    hasher.update(input_string);

    let result_string = hasher.finalize();

    let hash_string_hex = format!("{:x}", result_string);
    println!("Input password: {}", input_string);
    println!("Passwrod SHA-256 hash: {}", hash_string_hex);
    //---------------------------------------------------------------------------------------




    //call the C function
    unsafe {
        cFoo();
    }
}