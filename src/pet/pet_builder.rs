use crate::triggers::Position;
use crate::Reaction;
use super::{Pet, Pets};
use super::reactions::*;

pub struct PetBuilder {
    name: String,
    description: String,
    health: Option<u8>,
    attack: Option<u8>,
    location: Option<usize>,
    icon: Option<char>,
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

impl PetBuilder {
    pub fn make(pet: Pets) -> PetBuilder {
        match pet {
            Pets::Tiger => PetBuilder {
                name: String::from("Tiger"),
                description: String::from("It roars"),
                health: Some(5),
                attack: Some(2),
                icon: Some('🐅'),
                ..PetBuilder::default()
            },
            Pets::Crab => PetBuilder {
                name: String::from("Crab"),
                description: String::from("Nature loves making crabs"),
                health: Some(2),
                attack: Some(2),
                icon: Some('🦀'),
                react_func: Some(spawn_handle),
                ..PetBuilder::default()
            },
            Pets::Shark => PetBuilder {
                name: String::from("Shark"),
                description: String::from("Teeth are pointy"),
                health: Some(1),
                attack: Some(2),
                icon: Some('🦈'),
                react_func: Some(shark_handle),
                ..PetBuilder::default()
            },
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
