fn main() {
    //compile the C code
    cc::Build::new()
        .file("src/hello.c") 
        .compile("hello"); //name of library being compiled
}
