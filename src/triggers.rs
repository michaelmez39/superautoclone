pub struct Trigger {
    position: u8,
    event: Event
}
enum Event {
    AnimalAttack, // may lead to hurt or faint
    AnimalFaint,
    AnimalHurt,
    EndTurn,
    StartCombat,
    AnimalSpawn
}