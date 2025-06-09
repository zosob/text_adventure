use std::io::{self, Write};

fn main() {
    println!("Welcome to the Dungeion Adventure!");
    println!("Type 'help' to see available commands");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input){
            Ok(_) => {
                let command = input.trim().to_lowercase();

                match command.as_str(){
                    "help" => print_help(),
                    "look" => println!("You see a dark room with a flickering torch."),
                    "quit" => {
                        println!("Thanks for playing!");
                        break;
                    }
                    _ => println!("I don't understand that command."),
                }
            }
            Err(e) => println!("Failed to read input: {}", e),
        }
    }
}

fn print_help(){
    
    println!("Availble commands:");
    println!("look - Describe the current room");
    println!("quit - Exit the game");
    println!("help - Show this help message");
}