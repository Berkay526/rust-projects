use std::io::{self, Read};

fn main() {
    println!("Enter two numbers as operands.");
    println!("Then enter 'a' for add, 's' for subtract, 'm' for multiply, 'd' for divide, and 'r' for remainder.");
    println!("Type 'e' to exit the program.");

    let mut user_number1: String = String::new();
    let mut user_number2: String = String::new();
    let mut result: u32 = 0;
    let mut user_operation: [u8; 1] = [0u8; 1];

    loop {
        println!("Enter first number:");
        user_number1.clear(); 
        user_number2.clear(); 

        io::stdin()
            .read_line(&mut user_number1)
            .expect("Failed to read line");

        println!("Enter second number:");
        io::stdin()
            .read_line(&mut user_number2)
            .expect("Failed to read line");

        println!("Enter operation ('a', 's', 'm', 'd', 'r', 'e'):");
        match io::stdin().lock().read_exact(&mut user_operation) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read operation. Try again.");
                continue;
            }
        };

        let num1: u32 = match user_number1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Try again.");
                continue;
            }
        };

        let num2: u32 = match user_number2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Try again.");
                continue;
            }
        };

        let the_operation: u8 = user_operation[0]; 

        if the_operation == b'a' {
            result = num1 + num2;
            println!("{num1} + {num2} = {result}");
        } 

        else if the_operation == b's' {
            result = num1 - num2;
            println!("{num1} - {num2} = {result}");
        } 

        else if the_operation == b'm' {
            result = num1 * num2;
            println!("{num1} * {num2} = {result}");
        } 

        else if the_operation == b'd' {
            if num2 == 0 {
                println!("Division by zero is not allowed.");
                continue;
            }
            result = num1 / num2;
            println!("{num1} / {num2} = {result}");
        } 

        else if the_operation == b'r' {
            if num2 == 0 {
                println!("Division by zero is not allowed.");
                continue;
            }

            result = num1 % num2;
            println!("{num1} % {num2} = {result}");
        } 

        else if the_operation == b'e' {
            println!("Exiting the program.");
            break;
        } 

        else {
            println!("Invalid operation. Try again.");
        }
    }
}