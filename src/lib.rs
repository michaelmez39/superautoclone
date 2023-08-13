#![allow(dead_code)] // remove this later
pub mod pet;
// mod shop;
pub mod team;
mod triggers;

use pet::{Pet, PetConstructor};
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
        println!("Phase {}", phases);
        print!("{} 🆚 {}", team1, team2);
        println!("");
        // let position1 = Position::Left(team1.combatant_location()?);
        // let position2 = Position::Right(team2.combatant_location()?);
        // since we realigned the teams, the combatants should be at position 0
        queue.add(Event {
            event: Combat(0, 0),
            team: Position::Left,
        });
        queue.add(Event {
            event: Combat(0, 0),
            team: Position::Right,
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
    use crate::{PetConstructor, Team};
    use crate::triggers::Position;
    #[test]
    fn simple_battle() {
        let tiger = PetConstructor::make("tiger", Position::Both);
        let crab = PetConstructor::make("crab", Position::Both);
        let team1 = Team::new(
            [
                Some(crab.clone()),
                Some(tiger.clone()),
                Some(tiger.clone()),
                Some(tiger.clone()),
                Some(tiger.clone()),
            ],
            Position::Left,
        );
        let team2 = Team::new(
            [
                Some(tiger.clone()),
                Some(tiger.clone()),
                Some(tiger.clone()),
                Some(tiger.clone()),
                Some(tiger.clone()),
            ],
            Position::Right,
        );
        let outcome = crate::battle(team1, team2);
        println!("{:?}", outcome);
    }
}
