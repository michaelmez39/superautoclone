use std::collections::VecDeque;
use crate::{team::Team, Pet};

// TODO: Replace weird trigger logic in main with TriggerQueue
pub struct TriggerQueue(VecDeque<Event>);
impl TriggerQueue {
    pub fn new() -> Self {
        TriggerQueue(VecDeque::new())
    }

    pub fn add(&mut self, event: Event) {
        self.0.push_back(event);
    }

    pub fn resolve(&mut self, team: &mut Team, team2: &mut Team) {
        while let Some(trigger) = self.0.pop_front() {
            for pet in team.pets.iter_mut() {
                match pet {
                    Some(anim) => anim.react(self, &trigger),
                    None => continue,
                }
            }
            for pet in team2.pets.iter_mut() {
                match pet {
                    Some(anim) => anim.react(self, &trigger),
                    None => continue,
                }
            }
            // println!("Queue has {} events", self.0.len());
            team.react(self, &trigger).expect("yikes");
            team2.react(self, &trigger).expect("yikes");
        }
    }
}

pub struct Event {
    pub team: Position,
    pub event: EventType
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Position {
    Left,
    Right,
    Both,
    Neither
}

impl Position {
    pub fn other(&self) -> Self {
        match self {
            Position::Left => Self::Right,
            Position::Right => Self::Left,
            _ => self.clone()
        }
    }
}

pub enum EventType {
    // generally will follow these phases
    // combat -> attacked -> (faint / hurt)
    Combat(usize, usize),    // attacking position, defending position
    Attacked(usize, u8),     // the pet at the position is attacked with 8 power
    Faint(usize),            // pet at position has fainted
    Hurt(usize),             // pet at position has been hurt
    StartCombat,                // combat has started
    PleaseSpawn(usize, Pet), // try to spawn in if can fit in the team, (position, name, attack, health)
    Spawn(usize, Pet),       // pet spawned onto the team
}
