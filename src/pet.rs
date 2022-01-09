use crate::triggers::{Trigger, TriggerQueue};
use crate::Equipment;
use crate::Reaction;
use crate::Shop;
pub struct PetConstructor;
impl PetConstructor {
    pub fn make(name: &str) -> Pet {
        match name {
            "tiger" => Pet {
                name: String::from("tiger"),
                description: String::from("a tiger"),
                health: 5,
                attack: 5,
                react_func: |Pet, queue, trigger, shop| (),
                pre_equipment: None,
                post_equipment: None,
            },
            _ => Pet {
                name: String::from("Error Pet"),
                description: String::from("The Pet that you tried to make cannot be found..."),
                health: 50,
                attack: 50,
                react_func: |Pet, queue, trigger, shop| (),
                pre_equipment: None,
                post_equipment: None,
            },
        }
    }
}
pub struct Pet {
    name: String,
    description: String,
    health: u8,
    attack: u8,
    pre_equipment: Option<Equipment>,
    post_equipment: Option<Equipment>,
    react_func: Reaction,
}

impl Pet {
    pub fn react(&mut self, queue: &mut TriggerQueue, trigger: &Trigger, shop: &mut Shop) {
        (self.react_func)(self, queue, trigger, shop)
    }

    pub fn with_health(&mut self, health: u8) {
        self.health = health;
    }

    pub fn with_attack(&mut self, health: u8) {
        self.health = health;
    }

    pub fn attack(&self) -> u8 {
        self.attack
    }
}

impl<'a> std::fmt::Display for Pet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{} attack and {} health",
            self.name, self.description, self.attack, self.health
        )
    }
}
