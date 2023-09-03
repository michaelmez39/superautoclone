use super::reactions::*;
use super::{Pet, Pets};
use crate::events::Position;
use crate::Reaction;
use serde::Deserialize;

const PETS_TOML: &'static str = include_str!("pets.toml");

pub struct PetBuilder {
    name: String,
    description: String,
    health: Option<u8>,
    attack: Option<u8>,
    location: Option<usize>,
    icon: Option<char>,
    shop_level: Option<u8>,
    react_func: Option<Reaction>,
    team: Option<Position>,
}

impl PetBuilder {
    pub fn build(&mut self) -> Pet {
        Pet {
            name: self.name.clone(),
            description: self.description.clone(),
            health: self.health.unwrap_or(1),
            attack: self.attack.unwrap_or(1),
            location: self.location.unwrap_or(0),
            icon: self.icon.unwrap_or('❓'),
            shop_level: self.shop_level.unwrap_or(1),
            react_func: self.react_func.unwrap_or(default_handle),
            team: self.team.unwrap_or(Position::Neither),
        }
    }
    pub fn health(&mut self, health: u8) -> &mut Self {
        self.health = Some(health);
        self
    }
    pub fn attack(&mut self, attack: u8) -> &mut Self {
        self.attack = Some(attack);
        self
    }
    pub fn at(&mut self, at: usize) -> &mut Self {
        self.location = Some(at);
        self
    }
    pub fn icon(&mut self, icon: char) -> &mut Self {
        self.icon = Some(icon);
        self
    }
    pub fn reaction(&mut self, reaction: Reaction) -> &mut Self {
        self.react_func = Some(reaction);
        self
    }
    pub fn team(&mut self, team: Position) -> &mut Self {
        self.team = Some(team);
        self
    }
}

#[derive(Deserialize)]
struct PetsConfig {
    name: String,
    description: String,
    health: Option<u8>,
    attack: Option<u8>,
    icon: Option<char>,
    shop_level: Option<u8>,
}

impl PetsConfig {
    fn make_builder(self, reaction: Reaction) -> PetBuilder {
        PetBuilder {
            name: self.name,
            description: self.description,
            health: self.health,
            attack: self.attack,
            icon: self.icon,
            shop_level: self.shop_level,
            react_func: Some(reaction),
            ..Default::default()
        }
    }
}

impl PetBuilder {
    pub fn make(pet: Pets) -> PetBuilder {
        let pets_table: toml::Table = toml::from_str(PETS_TOML).expect("Could not parse pets.toml");
        let get_config = |name: &str, reaction: Reaction| -> PetBuilder {
            pets_table[name]
                .clone()
                .try_into::<PetsConfig>()
                .expect("Could not parse pet in pets.toml")
                .make_builder(reaction)
        };
        match pet {
            Pets::Tiger => get_config("Tiger", default_handle),
            Pets::Crab => get_config("Crab", spawn_handle),
            Pets::Shark => get_config("Shark", shark_handle),
            _ => PetBuilder::default(),
        }
    }
}

impl Default for PetBuilder {
    fn default() -> Self {
        PetBuilder {
            name: String::from("Error Pet"),
            description: String::from("The Pet that you tried to make cannot be found..."),
            health: Some(50),
            attack: Some(50),
            location: Some(0),
            icon: Some('❓'),
            team: Some(Position::Neither),
            shop_level: Some(1),
            react_func: Some(default_handle),
        }
    }
}

impl Pet {
    pub fn custom(name: String, description: String) -> PetBuilder {
        PetBuilder {
            name,
            description,
            ..PetBuilder::default()
        }
    }
}
