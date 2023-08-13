use crate::triggers::{Event, Position, TriggerQueue};
use crate::Reaction;
pub struct PetConstructor;
impl PetConstructor {
    pub fn make(name: &str, team: Position) -> Pet {
        match name {
            "tiger" => Pet {
                name: String::from("tiger"),
                description: String::from("a tiger"),
                health: 5,
                attack: 2,
                react_func: default_handle,
                location: 0,
                icon: '🐅',
                team,
            },
            "crab" => Pet {
                name: String::from("Crab"),
                description: String::from("a crab"),
                health: 2,
                attack: 2,
                icon: '🦀',
                react_func: spawn_handle,
                location: 0,
                team,
            },
            _ => Pet {
                name: String::from("Error Pet"),
                description: String::from("The Pet that you tried to make cannot be found..."),
                health: 50,
                attack: 50,
                location: 0,
                icon: '❓',
                team,
                react_func: |_pet, _queue, _trigger| (),
            },
        }
    }
}

// 

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

impl std::fmt::Display for Pet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.team {
            Position::Left => write!(f, " {} {} \u{202E}{}\u{202D}", self.health, self.attack, self.icon),
            _=> write!(f, "{} {} {} ", self.icon, self.attack, self.health),
        }
    }
}

impl Pet {
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
    if event.team == pet.team {
        match event.event {
            Combat(us, them) if us == pet.location => {
                println!(
                    "Pet combat started attacking {:?} with {} attack",
                    them,
                    pet.attack()
                );
                queue.add(Event {
                    event: Attacked(them, pet.attack()),
                    team: event.team.other(),
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
    if event.team == pet.team {
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
                    });

                    let mut tiger = PetConstructor::make("tiger", pet.team);
                    tiger.with_attack(1);
                    tiger.with_health(3);
                    tiger.at(position);
                    tiger.team = pet.team;
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
            _ => (),
        }
    }
}
