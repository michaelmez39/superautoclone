#![allow(dead_code)] // remove this later
pub mod pet;
// mod shop;
mod formatting;
mod shop;
pub mod team;
mod game;
mod triggers;

use pet::Pet;
use team::Team;
use triggers::{Event, EventType as E, Position, TriggerQueue};

pub type Reaction = fn(&mut Pet, &mut TriggerQueue, &Event) -> ();

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
        let emoji_background = "🌴🌼🌳🌵🌳🎋🌾🌳🌾🌿🌾🌳🌿🌳🌵🌾🌳🌾🌳🌿";
        let battle_view = format!("{}\n💥\n{}\n{}", team1, team2, emoji_background);
        println!(
            "{}",
            &formatting::border(battle_view, Some(format!("Phase {}", &phases)))
        );
        queue.add(Event {
            event: E::Combat(0, 0),
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
    use crate::pet::Pets;
    use crate::triggers::Position;
    use crate::{Pet, Team};

    #[test]
    fn shop() {}

    #[test]
    fn simple_battle() {
        let tiger = Pet::new(Pets::Tiger).build();
        let crab = Pet::new(Pets::Crab).build();
        let shark = Pet::new(Pets::Shark).build();
        let team1 = Team::new(
            vec![
                Some(crab.clone()),
                Some(crab.clone()),
                Some(tiger.clone()),
                Some(tiger.clone()),
                Some(shark.clone()),
            ],
            Position::Left,
        );
        let team2 = Team::new(
            vec![
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
