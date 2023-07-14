use std::io;

fn main() {
    println!("Welcome to the kitchen adventure!");
    println!("You are in a kitchen with three cupboards. You must find the hidden cookie!");

    loop {
        println!("Enter 1, 2 or 3 to open a cupboard:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid cupboard!");
                continue;
            },
        };

        match input {
            1 => {
                println!("You open the first cupboard. It's full of plates, but no cookie.");
            },
            2 => {
                println!("You found the cookie in the second cupboard! Congratulations!");
                break;
            },
            3 => {
                println!("You open the third cupboard. It's full of cups, but no cookie.");
            },
            _ => {
                println!("That's not a valid cupboard!");
            },
        }
    }
}
