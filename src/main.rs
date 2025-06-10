use std::io::{self, Write};
use std::collections::HashMap;

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
    exits: HashMap<String, String>,
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

    let mut rooms: HashMap<String, Room> = HashMap::new();
    let mut current_room = "entrance".to_string();

    rooms.insert("entrance".to_string(), Room {
        description: "You are at the entrace of a dark dungeion.".to_string(),
        exits: HashMap::from([
            ("north". to_string(), "hallway".to_string())
        ]),
        items:vec![],
        has_enemy: false,
    });

    rooms.insert("hallway".to_string(), Room {
        description: "A long dimly lit hallway.".to_string(),
        exits: HashMap::from([
            ("south". to_string(), "entrance".to_string()),
            ("east". to_string(), "treasure".to_string())
        ]),
        items:vec![Item::Key],
        has_enemy: false,
    });

    rooms.insert("treasure".to_string(), Room {
        description: "You've found the treasure room!".to_string(),
        exits: HashMap::from([
            ("west". to_string(), "hallway".to_string())
        ]),
        items:vec![Item::Potion],
        has_enemy: false,
    });


    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input){
            Ok(_) => {
                let command = input.trim().to_lowercase();

                match command.as_str(){
                    command if command.starts_with("go ")=> {
                        let direction = command.trim_start_matches("go").trim();

                        if let Some(next_room) = rooms[&current_room].exits.get(direction){
                            current_room = next_room.clone();
                            println!("{}", rooms[&current_room].description);
                        } else {
                            println!("You can't go that way!")
                        }
                    }
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
    println!("go <direction> - Move to another room (e.g., 'go north')");
    println!("quit - Exit the game");
    println!("help - Show this help message");
}