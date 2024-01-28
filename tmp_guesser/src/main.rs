
use std::io;

fn main() {
    println!("Converting temp degrees!");

    loop {
        println!("Please input your temperature and the program will convert from Celsius to Fahrenheit. Type 'exit' to close this program.");

        let mut input= String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to real line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your input is: {input}");
        let converted = (input * (9/5))+32; 

        println!("Result: {input} degrees in Celsius correspond to {converted} degrees in Fahrenheit.");
        println!("What would you like to do next?");
        let mut decision = String::new();
        io::stdin()
            .read_line(&mut decision)
            .expect("Failed to real line");
        if decision.trim() == "exit" {
            println!("You've exited the process.");
            break;
        } else if decision == "continue" {
            continue;
        }
    }    
}
