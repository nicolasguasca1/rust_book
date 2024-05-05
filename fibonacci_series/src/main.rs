
use std::io;

fn get_nth(n:u32) -> u32 {
            let result:u32;
            if n == 0 {
                result = 0;
            } else if n == 1 {
                result = 1;
            } else {
                result = get_nth(n-1) + get_nth(n-2)
            }
            result
        }

fn main() {
    println!("Getting the Nth fibonacci number!");

    loop {
        println!("Please input the n number you'd like to get from the Fibonacci series");

        let mut input= String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to real line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your input is: {input}");
        
        let final_result = get_nth(input);
        println!("The result is: {final_result}");

        println!("Would you like to get another one? If yes, type yes. Otherwise type no.");
        let mut decision = String::new();
        io::stdin()
            .read_line(&mut decision)
            .expect("Failed to real line");
        if decision.trim() == "no" {
            println!("You've exited the process.");
            break;
        } else if decision == "yes" {
            continue;
        }
    }    
}
