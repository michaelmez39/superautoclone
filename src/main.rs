#![allow(dead_code)] // remove this later
mod pet;
mod team;
mod triggers;
use text_io::read;

use pet::{Pet, PetConstructor};
use team::Team;
use triggers::{Trigger, TriggerQueue, Event, Position};

pub type Reaction = fn(&mut Pet, &mut TriggerQueue, &Trigger, &mut Shop) -> ();
struct Score {
    wins: u8,
    lives: u8,
}

pub struct Equipment {
    react: Reaction,
}

pub struct Food {
    name: String,
    description: String,
    apply: Box<dyn FnMut(&mut Pet)>,
}
impl std::fmt::Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.name, self.description)
    }
}

struct Item {
    frozen: bool,
    item: ItemType,
}
enum ItemType {
    F(Food),
    A(Pet),
}

impl Item {
    fn random() -> Self {
        Item {
            frozen: false,
            item: ItemType::A(PetConstructor::make("tiger")),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.item {
            ItemType::A(Pet) => write!(f, "{}", Pet)?,
            ItemType::F(food) => write!(f, "{}", food)?,
        }
        Ok(())
    }
}
pub struct Shop {
    items: [Option<Item>; 5],
    money: u8,
}

impl Shop {
    fn new() -> Self {
        let items = [
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
                None => writeln!(f, "Item {} is empty", n)?,
            }
        }
        Ok(())
    }
}

impl Shop {
    fn roll(&mut self) {}
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
enum BattleOutcome {
    Win,
    Draw,
    Loss,
}

// shold pass a clone into this function,
// what happens in battle should not effect the team overall
fn battle(mut team1: Team, mut team2: Team) -> BattleOutcome {
    let mut queue: TriggerQueue = TriggerQueue::new();
    let mut shop = Shop::new();
    while team1.0.iter().any(Option::is_some) && team2.0.iter().any(Option::is_some) {
        let (position1, combatant1) = team1
            .0
            .iter_mut()
            .enumerate()
            .skip_while(|(n, x)| x.is_none())
            .next()
            .unwrap();
        let combatant1 = combatant1.as_mut().unwrap();
        let (position2, combatant2) = team2
            .0
            .iter_mut()
            .enumerate()
            .skip_while(|(n, x)| x.is_none())
            .next()
            .unwrap();
        let combatant2 = combatant2.as_mut().unwrap();
        queue.add(Position::Left(position1 as u8), Event::PetAttacked(combatant2.attack()));
        queue.add(Position::Right(position2 as u8), Event::PetAttacked(combatant1.attack()));
        queue.resolve(&mut team1, &mut team2, &mut shop);
    
    }
    BattleOutcome::Draw
}

fn main() {
    let mut score = Score { wins: 0, lives: 2 };
    let mut shop = Shop::new();
}
