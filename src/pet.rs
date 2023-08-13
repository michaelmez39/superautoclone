use crate::triggers::{Event, Position, TriggerQueue};
use crate::Reaction;

pub enum Pets {
    Tiger,
    Crab,
    Shark,
    Unknown,
}

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

impl Pet {
    pub fn custom(name: String, description: String) -> PetBuilder {
        PetBuilder {
            name,
            description,
            ..PetBuilder::default()
        }
    }

    pub fn new(pet: Pets) -> PetBuilder {
        PetBuilder::make(pet)
    }

    pub fn react(&mut self, queue: &mut TriggerQueue, trigger: &Event) {
        (self.react_func)(self, queue, trigger)
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

use crate::triggers::EventType::*;
fn default_handle(pet: &mut Pet, queue: &mut TriggerQueue, event: &Event) {
    if event.team == pet.team || event.team == Position::Both {
        match event.event {
            Combat(us, them) if us == pet.location => {
                println!(
                    "Pet combat started attacking {:?} with {} attack",
                    them,
                    pet.attack()
                );
                queue.add(Event {
                    event: Attacked(them, pet.attack()),
                    team: pet.team.other(),
                })
            }
            Attacked(position, amount) if position == pet.location => {
                println!("Attacked {}!", pet.name);
                if amount >= pet.health {
                    queue.add(Event {
                        team: event.team,
                        event: Faint(pet.location),
                    })
                } else {
                    pet.with_health(pet.health() - amount);
                    queue.add(Event {
                        event: Hurt(pet.location),
                        team: pet.team,
                    })
                }
            }
            _ => (),
        }
    }
}

fn spawn_handle(pet: &mut Pet, queue: &mut TriggerQueue, event: &Event) {
    if event.team == pet.team || event.team == Position::Both {
        match event.event {
            Attacked(position, amount) if position == pet.location => {
                println!("Attacked {}!", pet.name);
                if amount >= pet.health {
                    queue.add(Event {
                        team: event.team,
                        event: Faint(pet.location),
                    });

                    let tiger = Pet::new(Pets::Tiger)
                        .attack(1)
                        .health(3)
                        .at(position)
                        .team(pet.team)
                        .build();

                    queue.add(Event {
                        event: PleaseSpawn(pet.location, tiger),
                        team: event.team,
                    });
                } else {
                    pet.with_health(pet.health() - amount);
                    queue.add(Event {
                        event: Hurt(pet.location),
                        team: pet.team,
                    });
                }
            }
            _ => default_handle(pet, queue, event),
        }
    }
}

fn shark_handle(pet: &mut Pet, queue: &mut TriggerQueue, event: &Event) {
    match event.event {
        Faint(pos) if event.team == pet.team && pos < pet.location => {
            pet.with_attack(pet.attack + 1);
            pet.with_health(pet.health + 1);
        },
        _ => default_handle(pet, queue, event)
    }
}