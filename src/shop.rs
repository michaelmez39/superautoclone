use crate::formatting::{self, Emoji, Emojify};
use crate::pet::Pet;
use crate::pet::Pets;
use crate::triggers::{Event, EventType, ShopEvent};
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
        inventory.push_str(" \n");
        for item in self.items.iter() {
            inventory.push(' ');
            inventory.push_str(&item.as_ref().map_or("  ".to_string(), |e| e.item.icon().to_string()));
        }
        inventory.push(' ');
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
        println!("What would you like to do buy(1) freeze(2) roll(3) or combat(4)");
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn show_shop() {
        let shop = Shop::new();
        println!("{}", shop);
    }
}
