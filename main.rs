use std::io;


extern "C" {
    fn cFoo();
}

fn main() {
    println!("Calling C code from Rust...");
    
    //Testing basic I/O
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

    //call the C function
    unsafe {
        cFoo();
    }
}