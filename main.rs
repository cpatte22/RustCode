use std::io;
fn main(){
    //Test the basic
    /*
    let mut num: i8 = 11;
    let num3: i8 = 13;

    let j = num + num3;
    println!("{}", j);
    println!("this is rust");
    println!("value of num: {}", num);
    //num = 13;
    println!("value of num: {}", num);
    */

    //Testing basic I/O
    println!("Enter two numbers to add\nnum_1: ");
    let mut user_in_num1 = String::new(); //string variable for holding user input
    let mut user_in_num2 = String::new();

    io::stdin().read_line(&mut user_in_num1).expect("Failed to read input"); //storing user input in variable
    println!("num_2: ");
    io::stdin().read_line(&mut user_in_num2).expect("Failed to read second input");

    let user_in_num1_int: i32 = user_in_num1.trim().parse().unwrap(); //convert user string to int
    let user_in_num2_int: i32 = user_in_num2.trim().parse().unwrap();
    
    let sum = user_in_num1_int + user_in_num2_int; //perform addition calculation
    println!("The sum is {}", sum);


    
}