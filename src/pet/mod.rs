mod pet_builder;
mod reactions;

use crate::triggers::{Event, Position, TriggerQueue};
use crate::Reaction;
use crate::formatting::Emojify;
pub use pet_builder::PetBuilder;
use reactions::default_handle;
pub enum Pets {
    Tiger,
    Crab,
    Shark,
    Unknown,
}

#[derive(Clone)]
pub struct Pet {
    name: String,
    description: String,
    health: u8,
    attack: u8,
    location: usize,
    icon: char,
    react_func: Reaction,
    team: Position,
}

impl std::fmt::Debug for Pet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name:{}\ndescription:{}\nhealth:{}\nattack:{}\nlocation:{:?}",
            self.name, self.description, self.health, self.attack, self.location
        )
    }
}

impl Default for Pet {
    fn default() -> Self {
        Pet {
            name: String::from("Error Pet"),
            description: String::from("The Pet that you tried to make cannot be found..."),
            health: 50,
            attack: 50,
            location: 0,
            icon: '❓',
            team: Position::Neither,
            react_func: default_handle,
        }
    }
}

impl std::fmt::Display for Pet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.team {
            Position::Left => write!(
                f,
                " {} {} \u{202E}{}\u{202D}",
                self.health, self.attack, self.icon
            ),
            _ => write!(f, "{} {} {} ", self.icon, self.attack, self.health),
        }
    }
}

impl Emojify for Pet {
    fn icon(&self) -> char {
        self.icon
    }
}

impl Pet {
    pub fn new(pet: Pets) -> PetBuilder {
        PetBuilder::make(pet)
    }

    pub fn react(&mut self, queue: &mut TriggerQueue, trigger: &Event) {
        (self.react_func)(self, queue, trigger)
    }

    pub fn raise_stats(&mut self, attack: u8, health: u8) {
        self.attack += attack;
        self.health += health;
    }

    pub fn with_health(&mut self, health: u8) {
        self.health = health;
    }

    pub fn with_attack(&mut self, attack: u8) {
        self.attack = attack;
    }

    pub fn attack(&self) -> u8 {
        self.attack
    }
    pub fn health(&self) -> u8 {
        self.health
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn at(&mut self, location: usize) {
        self.location = location;
    }
    pub fn team(&mut self, team: Position) {
        self.team = team;
    }
}
