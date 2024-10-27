use std::io;
use std::io::Read;
use sha2::{Sha256, Digest};
use std::hash::{DefaultHasher, Hash, Hasher};
//use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

extern "C" {
    fn cFoo();
}

//Items for Hashing
#[derive(Hash)]
struct Player{
    username: String,
    id: u32,
    age: i32,
}

fn handle_client(mut stream: TcpStream) 
{
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let received = String::from_utf8_lossy(&buffer[..]);
            println!("Received message: {}", received);
        }
        Err(e) => {
            println!("Failed to receive message: {}", e);
        }
    }

}

fn main() {

    //Using Sockets for Computer Networking
    //bind the TCP server to the address and port
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to address");

    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    //Hashing in Rust-------------------------------
    let player_1 = Player{
        username: "xxXMonsterXxx".to_string(),
        id: 1234567,
        age: 23,
    };

    let player_2 = Player{
        username: "awesome123".to_string(),
        id: 8910111,
        age: 33
    };
    assert!(calculate_hash(&player_1) != calculate_hash(&player_2));
    
    //----------------------------------------------



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

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}