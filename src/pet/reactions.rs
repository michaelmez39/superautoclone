use super::{Pet, Pets};
use crate::events::{Event, Position, EventQueue};
use crate::events::EventType as E;
use crate::ReactionResult;

pub fn default_handle(pet: &mut Pet, queue: &mut EventQueue, event: &Event) -> ReactionResult {
    if event.team != pet.team && event.team != Position::Both {
        return Ok(());
    }
    match event.event {
        E::Combat(us, them) if us == pet.location => {
            println!(
                "Pet combat started attacking {:?} with {} attack",
                them,
                pet.attack()
            );
            queue.add(Event {
                event: E::Attacked(them, pet.attack()),
                team: pet.team.other(),
            });
        }
        E::Attacked(position, amount) if position == pet.location => {
            println!("Attacked {}!", pet.name);
            if amount >= pet.health {
                queue.add(Event {
                    team: event.team,
                    event: E::Faint(pet.location),
                });
            } else {
                pet.with_health(pet.health() - amount);
                queue.add(Event {
                    event: E::Hurt(pet.location),
                    team: pet.team,
                });
            }
        }
        _ => (),
    }
    Ok(())
}

pub fn spawn_handle(pet: &mut Pet, queue: &mut EventQueue, event: &Event) -> ReactionResult {
    if event.team != pet.team && event.team != Position::Both {
        return Ok(());
    }
    match event.event {
        E::Attacked(position, amount) if position == pet.location => {
            println!("Attacked {}!", pet.name);
            if amount >= pet.health {
                queue.add(Event {
                    team: event.team,
                    event: E::Faint(pet.location),
                });

                let tiger = Pet::new(Pets::Tiger)
                    .attack(1)
                    .health(3)
                    .at(position)
                    .team(pet.team)
                    .build();

                queue.add(Event {
                    event: E::Spawn(pet.location, tiger),
                    team: event.team,
                });
            } else {
                pet.with_health(pet.health() - amount);
                queue.add(Event {
                    event: E::Hurt(pet.location),
                    team: pet.team,
                });
            }
        }
        _ => default_handle(pet, queue, event)?,
    }
    Ok(())
}

pub fn shark_handle(pet: &mut Pet, queue: &mut EventQueue, event: &Event) -> ReactionResult {
    match event.event {
        E::Faint(pos) if event.team == pet.team && pos < pet.location => {
            pet.raise_stats(1, 1)
        }
        _ => default_handle(pet, queue, event),
    }
}
