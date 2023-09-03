use crate::{team::{Team, TeamError}, Pet, shop::Food, ReactionResult};
use std::collections::VecDeque;

// TODO: Replace weird trigger logic in main with TriggerQueue
pub struct EventQueue(VecDeque<Event>);
impl EventQueue {
    pub fn new() -> Self {
        EventQueue(VecDeque::new())
    }

    pub fn add(&mut self, event: Event) {
        self.0.push_back(event);
    }

    pub fn resolve(&mut self, team: &mut Team, team2: &mut Team) -> ReactionResult {
        while let Some(trigger) = self.0.pop_front() {
            for pet in team.pets.iter_mut() {
                match pet {
                    Some(anim) => anim.react(self, &trigger)?,
                    None => continue,
                }
            }
            for pet in team2.pets.iter_mut() {
                match pet {
                    Some(anim) => anim.react(self, &trigger)?,
                    None => continue,
                }
            }
            // println!("Queue has {} events", self.0.len());
            team.react(self, &trigger)?;
            team2.react(self, &trigger)?;
        }
        Ok(())
    }

    pub fn resolve_single(&mut self, team: &mut Team) -> ReactionResult {
        while let Some(trigger) = self.0.pop_front() {
            for pet in team.pets.iter_mut() {
                match pet {
                    Some(anim) => {
                        anim.react(self, &trigger)?
                    }
                    None => continue,
                }
            }
            team.react(self, &trigger)?
        }
        Ok(())
    }
}

pub struct Event {
    pub team: Position,
    pub event: EventType,
}

impl Event {
    pub fn left(event: EventType) -> Self {
        Event {
            team: Position::Left,
            event
        }
    }
    pub fn right(event: EventType) -> Self {
        Event {
            team: Position::Right,
            event
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Position {
    Left,
    Right,
    Both,
    Neither,
}

impl Position {
    pub fn other(&self) -> Self {
        match self {
            Position::Left => Self::Right,
            Position::Right => Self::Left,
            _ => self.clone(),
        }
    }
}

pub struct ShopEvent {
    pub at: usize,
    pub gold: u8,
}
impl ShopEvent {
    pub fn new(at: usize, gold: u8) -> Self {
        ShopEvent { at, gold }
    }
}
pub enum EventType {
    // generally will follow these phases
    // combat -> attacked -> (fPleaseSpawaint / hurt)
    Combat(usize, usize),    // attacking position, defending position
    Attacked(usize, u8),     // the pet at the position is attacked with 8 power
    Faint(usize),            // pet at position has fainted
    Hurt(usize),             // pet at position has been hurt
    StartCombat,             // combat has started
    Spawn(usize, Pet), // try to spawn in if can fit in the team, (position, name, attack, health)
    Spawned(usize, Pet),       // pet spawned onto the team
    // Shop
    BuyFood(ShopEvent, Food), // pet position
    BuyPet(ShopEvent, Pet),
}

#[derive(Debug)]
pub enum EventError {
    Shop,
    Battle,
    Team(TeamError)
}

impl std::error::Error for EventError {

}

impl std::fmt::Display for EventError {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       writeln!(f, "Error while trying to perform action\n{:?}", self)
   } 
}
