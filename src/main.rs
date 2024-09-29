extern "C" {
    fn cFoo();
}

fn main() {
    println!("Calling C code from Rust...");
    
    //call the C function
    unsafe {
        cFoo();
    }
}