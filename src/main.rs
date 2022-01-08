#![allow(dead_code)] // remove this later
mod triggers;

use text_io::read;
use triggers::Trigger;

type Reaction = fn(&mut Animal, &Trigger, &mut Shop) -> ();
struct Score {
    wins: u8,
    lives: u8
}

struct Equipment {
    react: Reaction
}
struct Animal {
    name: String,
    description: String,
    health: u8,
    attack: u8,
    equipment: Option<Equipment>,
    react: Reaction,
}

impl<'a> std::fmt::Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{} attack and {} health", self.name, self.description, self.attack, self.health)
    }
}
struct Food {
    name: String,
    description: String,
    apply: Box<dyn FnMut(&mut Animal)>
}
impl std::fmt::Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.name, self.description)
    }
}

struct Item {
    frozen: bool,
    item: ItemType
}
enum ItemType {
    F(Food),
    A(Animal)
}

impl Item {
    fn random() -> Self {
        Item{
            frozen: false,
            item: ItemType::A(
                Animal {
                    name: String::from("tiger"),
                    description: String::from("a tiger"),
                    health: 5,
                    attack: 5,
                    react: |animal, trigger, shop| (),
                    equipment: None
                }
            )
        }
    }
}

fn react(trigger: Trigger, team: &mut Team, shop: &mut Shop) {
    for animal in team.0.iter_mut() {
        match animal {
            Some(a) => (a.react)(a, &trigger, shop),
            None => ()
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.item {
            ItemType::A(animal) => write!(f, "{}", animal)?,
            ItemType::F(food) => write!(f, "{}", food)?
        }
        Ok(())
    }
}
struct Shop {
    items: [Option<Item>; 5],
    money: u8
}

impl Shop {
    fn new() -> Self {
        let items = 
        [
        Some(Item::random()),
        Some(Item::random()),
        Some(Item::random()),
        Some(Item::random()),
        Some(Item::random()),
        ];
        let money = 10;
        Self { items, money }
    }
}

impl std::fmt::Display for Shop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (n, item) in self.items.iter().enumerate() {
            match item {
                Some(i) => writeln!(f, "Item {} \n {}", n, i)?,
                None => writeln!(f, "Item {} is empty", n)?
            
            }
        }
        Ok(())
    }
}
struct Team([Option<Animal>; 5]);

impl Shop {
    fn roll(&mut self) {

    }
}

fn buy(shop: &mut Shop) {
    println!("What would you like to buy?");
}

fn freeze(shop: &mut Shop) {
    println!("What to freeze?");
}
fn roll(shop: &mut Shop) {
    println!("roll?");
}
fn start_shop(shop: &mut Shop) {
    println!("Welcome the shop");
    roll(shop);
    loop {
        println!("{}", shop);
        println!("What would you like to buy(1) freeze(2) roll(3) or combat(4)");
        let response: i32 = read!();
        match response {
            1 => buy(shop),
            2 => freeze(shop),
            3 => roll(shop),
            4 => {
                println!("Starting combat!");
                return;
            }
            _ => {
                println!("Invalid option! Choose again...");
            }
        }
    }
}
fn start_combat(score: &mut Score) {
    score.lives -= 1;
}

fn main() {
    let mut score = Score {wins: 0, lives: 2};
    let mut shop = Shop::new();
    while score.wins <= 10 && score.lives > 0 {
        shop.money = 10;
        start_shop(&mut shop);
        start_combat(&mut score);
    }
    if score.wins == 10 {
        println!("You won the game!")
    } else {
        println!("You won {} out of ten", score.wins)
    }
}
