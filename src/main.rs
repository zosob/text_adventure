use std::io::{self, Write};

#[derive(Debug)]
struct Player{
    name: String,
    health: i32,
    inventory: Vec<Item>,
}

#[derive(Debug)]
enum Item{
    Potion, 
    Key,
    Sword,
}

#[derive(Debug)]
struct Room {
    description: String,
    has_enemy: bool,
    items: Vec<Item>,
}

fn main() {

    let mut player = Player{
        name: String::from("Hero"),
        health: 100,
        inventory: vec![],
    };

    println!("Welcome to the Dungeon Adventure!");
    println!("You are {}, with {} health.", player.name, player.health);
    println!("Type 'help' to see available commands");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input){
            Ok(_) => {
                let command = input.trim().to_lowercase();

                match command.as_str(){
                    "inventory" => {
                        if player.inventory.is_empty(){
                            println!("Your inventoryt is empty!");
                        } else {
                            println!("You have: ");
                            for item in &player.inventory{
                                println!("{:?}", item);
                            }
                        }
                    }
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
    println!("inventory - Show your items");
    println!("quit - Exit the game");
    println!("help - Show this help message");
}