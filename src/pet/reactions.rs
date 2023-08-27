use crate::triggers::EventType::*;
use crate::triggers::{TriggerQueue, Event, Position};
use super::{Pet, Pets};

pub fn default_handle(pet: &mut Pet, queue: &mut TriggerQueue, event: &Event) {
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

pub fn spawn_handle(pet: &mut Pet, queue: &mut TriggerQueue, event: &Event) {
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
                        event: Spawn(pet.location, tiger),
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

pub fn shark_handle(pet: &mut Pet, queue: &mut TriggerQueue, event: &Event) {
    match event.event {
        Faint(pos) if event.team == pet.team && pos < pet.location => {
            pet.raise_stats(1, 1)
        },
        _ => default_handle(pet, queue, event)
    }
}