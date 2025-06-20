use crate::Player;


#[derive(Debug)]
pub struct Enemy {
    pub name: String,
    pub health: i32,
    pub attack_power: i32,
}

pub fn fight(player: &mut Player, enemy: &mut Enemy) {
    println!("⚔️⚔️ A wild {} appears!", enemy.name);

    while player.health > 0 && enemy.health > 0 {
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

    if player.health <= 0 {
        println!("☠️☠️ You have been defeated...");
    }
}
