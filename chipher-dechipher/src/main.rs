use std::io;       //Uses standard input/output operation
fn main() {                           //This is the main function
    let mut  mode = String::new();
    println!("Please enter the mode (Chipier/Dichipher)");
    io::stdin().read_line(&mut mode)
    .expect("Failed to read line");   
    mode=mode.trim().to_lowercase();
    if mode == "chipher"{
        println!("You entered Chiphered mode");
    }
    else if mode == "dechipher"{
        println!("You entered dichiphered mode");
    }
    else {
        println!("You entered the wrong mode")
    }
