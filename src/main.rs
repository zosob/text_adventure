use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug)]
struct Player{
    name: String,
    health: i32,
    inventory: Vec<Item>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
struct Enemy{
    name: String,
    health: i32,
    attack_power: i32,
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
        description: "You are at the entrance of a dark dungeon.".to_string(),
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
        has_enemy: true,
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
                            println!("Your inventory is empty!");
                        } else {
                            println!("You have: ");
                            for item in &player.inventory{
                                println!("{:?}", item);
                            }
                        }
                    }
                    "take" => {
                        let room = rooms.get_mut(&current_room).unwrap();

                        if room.items.is_empty(){
                            println!("there's nothing to take.");
                        } else {
                            println!("You pick up:");
                            for item in &room.items{
                                println!(" - {:?}", item);
                                player.inventory.push(item.clone());
                            }
                            room.items.clear();
                        }
                    }
                    "use potion" => {
                        if let Some(pos) = player.inventory.iter().position(|i| matches!(i, Item::Potion)){
                            player.health+=20;
                            println!("You used a potion. Health: {}", player.health);
                            player.inventory.remove(pos);
                        } else {
                            println!("You don't have any potions!");
                        }
                    }
                    "help" => print_help(),
                    "look" => { 
                        let room = &rooms[&current_room];
                        println!("{}", room.description);

                        if room.has_enemy {
                            let mut enemy = Enemy{
                                name: "Goblin".to_string(),
                                health: 40,
                                attack_power: 10,
                            };
                            fight(&mut player, &mut enemy);
                        }
                    },
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
    
    println!("Available commands:");
    println!("look - Describe the current room");
    println!("inventory - Show your items");
    println!("go <direction> - Move to another room (e.g., 'go north')");
    println!("quit - Exit the game");
    println!("help - Show this help message");
    println!("take - Pick up items in the room");
    println!("use potion - Heals 20 health if you have one");
}

fn fight(player: &mut Player, enemy: &mut Enemy){
    println!("⚔️⚔️ A wild {} appears!", enemy.name);

    while player.health > 0 && enemy.health> 0{
        println!("You hit the {}!", enemy.name);
        enemy.health -= 20;

        if enemy.health <= 0 {
            println!("You defeated the {}!", enemy.name);
            break;
        }

        print!("The {} strikes back!", enemy.name);
        player.health -= enemy.attack_power;
        println!("Your health: {}", player.health);
    }

    if player.health <=0 {
        println!("☠️☠️ You have been defeated...");
    }
}