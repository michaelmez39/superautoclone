use crate::{team::Team, Shop};
pub struct Trigger {
    position: Position,
    event: Event,
}
// TODO: Replace weird trigger logic in main with TriggerQueue
pub struct TriggerQueue(Vec<Trigger>);
impl TriggerQueue {
    pub fn new() -> Self {
        TriggerQueue(Vec::new())
    }

    pub fn add(&mut self, position: Position, event: Event) {
        self.0.push(Trigger { event, position });
    }

    pub fn resolve(&mut self, team: &mut Team, team2: &mut Team, shop: &mut Shop) {
        while let Some(ref trigger) = self.0.pop() {
            for Pet in team.0.iter_mut() {
                match Pet {
                    Some(anim) => anim.react(self, trigger, shop),
                    None => continue,
                }
            }
        }
    }
}
pub enum Position {
    Left(u8),
    Right(u8)
}
pub enum Event {
    PetAttacked(u8), // attack, position
    PetFaint,
    PetHurt,
    EndTurn,
    StartCombat,
    StartTurn,
    PetSpawn,
}
