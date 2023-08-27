use crate::formatting::{self, Emoji, Emojify};
use crate::pet::Pet;
use crate::pet::Pets;
use crate::team::Team;
use crate::triggers::{Event, EventType, Position, ShopEvent, TriggerQueue};
use crate::Reaction;
use text_io::read;

#[derive(Clone)]
pub struct Food {
    name: String,
    description: String,
    apply: Reaction,
    icon: char,
}

impl Emojify for Food {
    fn icon(&self) -> char {
        self.icon
    }
}

#[derive(Clone)]
struct Shelf {
    frozen: bool,
    item: Item,
}

#[derive(Clone)]
enum Item {
    Food(Food),
    Pet(Pet),
}

impl Emojify for Item {
    fn icon(&self) -> char {
        match self {
            Item::Food(food) => food.icon(),
            Item::Pet(pet) => pet.icon(),
        }
    }
}

impl Shelf {
    fn random() -> Self {
        Shelf {
            frozen: false,
            item: Item::Pet(Pet::new(Pets::Tiger).build()),
        }
    }
}

pub struct Shop {
    items: Vec<Option<Shelf>>,
    money: u8,
}

impl Shop {
    fn new() -> Self {
        let apple = Shelf {
            frozen: false,
            item: Item::Food(Food {
                icon: '🍎',
                name: "Apple".to_string(),
                description: "Raise stats by +1/+1".to_string(),
                apply: |pet, trigger_queue, event| {
                    pet.raise_stats(1, 1);
                },
            }),
        };
        let items = vec![
            Some(Shelf::random()),
            Some(Shelf::random()),
            Some(Shelf::random()),
            None,
            None,
            Some(apple.clone()),
            Some(apple.clone()),
        ];
        let money = 10;
        Self { items, money }
    }
}

impl std::fmt::Display for Shop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut inventory = String::new();
        for (idx, shelf) in self.items.iter().enumerate() {
            inventory.push(' ');
            inventory.push_str(&shelf.as_ref().map_or(" ".to_string(), |_| idx.to_string()));
            inventory.push(' ');
        }
        inventory.push_str("\n");
        for item in self.items.iter() {
            inventory.push(' ');
            inventory.push(item.as_ref().map_or(' ', |e| e.item.icon()));
        }
        writeln!(
            f,
            "{}",
            formatting::border(inventory, Some("Shop".to_string()))
        )
    }
}
impl Shop {
    fn roll(&mut self) {}
}

fn buy(shop: &mut Shop, event_queue: &mut TriggerQueue) {
    println!("Choose item? Back: 6");
    let choice: usize = read!();
    if choice >= shop.items.len() {
        println!("Invalid Choice!");
    }

    if let Some(shelf) = shop.items[choice].take() {
        println!("Choose spot for item");
        let spot: usize = read!();
        let event = match shelf.item {
            Item::Pet(pet) => Event::left(EventType::BuyPet(ShopEvent::new(spot, 3), pet)),
            Item::Food(food) => Event::left(EventType::BuyFood(ShopEvent::new(spot, 3), food)),
        };
        event_queue.add(event);
    } else {
        println!("Invalid Choice!");
    }
}

fn freeze(shop: &mut Shop) {
    println!("What to freeze?");
}

fn roll(shop: &mut Shop) {
    println!("roll?");
}

fn start_shop(shop: &mut Shop, team: &mut Team, event_queue: &mut TriggerQueue) {
    println!("Welcome the shop");
    roll(shop);
    loop {
        println!("{}\n{}", formatting::team_shop(team), shop);
        println!("What would you like to do buy(1) freeze(2) roll(3), combat(4), exit(5)");
        let response: i32 = read!();
        match response {
            1 => buy(shop, event_queue),
            2 => freeze(shop),
            3 => roll(shop),
            4 => {
                println!("Starting combat!");
                return;
            }
            5 => {
                println!("Goodbye!");
                return;
            }
            _ => {
                println!("Invalid option! Choose again...");
            }
        }
        event_queue.resolve_single(team);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn show_shop() {
        let shop = Shop::new();
        println!("{}", shop);
    }

    #[test]
    fn run_shop() {
        let mut shop = Shop::new();
        let mut event_queue = TriggerQueue::new();
        let mut team = Team::new(vec![None, None, None, None, None], Position::Left);
        start_shop(&mut shop, &mut team, &mut event_queue);
    }
}
