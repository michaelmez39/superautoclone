use crate::{team::TeamError, Pet, shop::Food};
use std::collections::VecDeque;

macro_rules! resolve {
    ($queue:expr, $($team:expr),*) => {'foo: {
        while let Some(ref event) = $queue.pop() {
            $(
                if let Err(error) = $team.react($queue, event) {
                    break 'foo Err(error)
                }
            )*
        }
        Ok(())
    }}
}

macro_rules! step {
    ($queue:expr, $($team:expr),*) => {
        if let Some(ref event) = $queue.0.pop_front() {
            $(
                $team.react($queue, event)?;
            )*
        }
        Ok(())
    }
}

pub(crate) use resolve;
pub(crate) use step;
pub struct EventQueue(VecDeque<Event>);
impl EventQueue {
    pub fn new() -> Self {
        EventQueue(VecDeque::new())
    }

    pub fn add(&mut self, event: Event) {
        self.0.push_back(event);
    }
    pub fn pop(&mut self) -> Option<Event> {
        self.0.pop_front()
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

#[cfg(test)]
mod tests {
    #[test]
    fn resolve_simple() {

    }
}