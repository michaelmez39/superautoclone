use crate::pet::Pet;
use crate::pet::Pets;
use crate::triggers::{Event, EventType, ShopEvent};
use crate::Position;
use crate::Reaction;
use text_io::read;

#[derive(Clone)]
pub struct Food {
    name: String,
    description: String,
    apply: Reaction,
    icon: char,
}

impl std::fmt::Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.name, self.description)
    }
}

#[derive(Clone)]
struct ShopItem {
    frozen: bool,
    item: Item,
}

#[derive(Clone)]
enum Item {
    Food(Food),
    Pet(Pet),
}

impl ShopItem {
    fn random() -> Self {
        ShopItem {
            frozen: false,
            item: Item::Pet(Pet::new(Pets::Tiger).build()),
        }
    }
}

impl std::fmt::Display for ShopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.item {
            Item::Pet(pet) => write!(f, "{}", pet.icon())?,
            Item::Food(food) => write!(f, "{}", food.icon)?,
        }
        Ok(())
    }
}
pub struct Shop {
    items: Vec<Option<ShopItem>>,
    money: u8,
}

impl Shop {
    fn new() -> Self {
        let apple = ShopItem {
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
            Some(ShopItem::random()),
            Some(ShopItem::random()),
            Some(ShopItem::random()),
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
        writeln!(f, "Shop")?;
        writeln!(f, "┌{:─^width$}┐", "", width = self.items.len() * 3 + 1)?;

        write!(f, "│")?;
        for (idx, shelf) in self.items.iter().enumerate() {
            write!(
                f,
                "{: ^3}",
                shelf.as_ref().map_or(" ".to_string(), |_| idx.to_string())
            )?;
        }
        writeln!(f, " │")?;

        write!(f, "│")?;
        for shelf in self.items.iter() {
            match shelf {
                Some(item) => write!(f, " {}", item),
                None => write!(f, "   "),
            }?;
        }
        writeln!(f, " │")?;
        writeln!(f, "└{:─^width$}┘", "", width = self.items.len() * 3 + 1)?;
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
