#![allow(dead_code)] // remove this later
pub mod pet;
// mod shop;
pub mod team;
mod triggers;
mod shop;
mod formatting;

use pet::Pet;
use team::Team;
use triggers::{Event, EventType::*, Position, TriggerQueue};

pub type Reaction = fn(&mut Pet, &mut TriggerQueue, &Event) -> ();

struct Score {
    wins: u8,
    lives: u8,
}

#[derive(Debug)]
pub enum BattleOutcome {
    Win,
    Draw,
    Loss,
}

// shold pass a clone into this function,
// what happens in battle should not effect the team overall
pub fn battle(mut team1: Team, mut team2: Team) -> BattleOutcome {
    let mut queue: TriggerQueue = TriggerQueue::new();
    let mut phases = 0;
    while team1.alive() && team2.alive() && phases < 90 {
        team1.realign();
        team2.realign();
        println!("Phase {: >2}:{} 🆚 {}",phases, team1, team2);
        queue.add(Event {
            event: Combat(0, 0),
            team: Position::Both,
        });
        queue.resolve(&mut team1, &mut team2);
        println!();
        phases += 1;
    }
    match (team1.alive(), team2.alive()) {
        (true, false) => BattleOutcome::Win,
        (false, true) => BattleOutcome::Loss,
        _ => BattleOutcome::Draw,
    }
}

#[cfg(test)]
mod tests {
    use crate::{Pet, Team};
    use crate::pet::Pets;
    use crate::triggers::Position;

    #[test]
    fn shop() {

    }

    #[test]
    fn simple_battle() {
        let tiger = Pet::new(Pets::Tiger).build();
        let crab = Pet::new(Pets::Crab).build();
        let shark = Pet::new(Pets::Shark).build();
        let team1 = Team::new(
            [
                Some(crab.clone()),
                Some(crab.clone()),
                Some(tiger.clone()),
                Some(tiger.clone()),
                Some(shark.clone()),
            ],
            Position::Left,
        );
        let team2 = Team::new(
            [
                Some(tiger.clone()),
                Some(tiger.clone()),
                Some(tiger.clone()),
                Some(tiger.clone()),
                Some(shark.clone()),
            ],
            Position::Right,
        );
        let outcome = crate::battle(team1, team2);
        println!("{:?}", outcome);
    }
}
