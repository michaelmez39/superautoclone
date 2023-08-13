use crate::pet::Pet;
use crate::triggers::{Event, EventType::*, TriggerQueue};
use crate::Position;
// The team is left to right in the array
// next combatant is the leftmost pet
// should shift pets over before next
// combat phase
pub struct Team {
    pub pets: Vec<Option<Pet>>,
    team: Position,
    max: usize,
}

impl Team {
    pub fn add(&mut self, mut pet: Pet) {
        let (open, _) = self
            .pets
            .iter()
            .enumerate()
            .find(|(_, elem)| elem.is_none())
            .expect("cannot add pet to team that is full");
        pet.at(open);
        self.pets[open] = Some(pet);
    }

    pub fn alive(&self) -> bool {
        self.pets.iter().any(Option::is_some)
    }

    pub fn new(mut pets: [Option<Pet>; 5], team: Position) -> Team {
        for (i, pet) in pets.iter_mut().enumerate() {
            if let Some(p) = pet {
                p.at(i);
                p.team(team);
            }
        }
        Team {
            pets: pets.to_vec(),
            max: 5,
            team,
        }
    }

    pub fn combatant_location(&self) -> Option<usize> {
        self.pets.iter().position(|pet| pet.is_some())
    }

    pub fn realign(&mut self) {
        self.pets = self
            .pets
            .clone()
            .into_iter()
            .filter(|pet| pet.is_some())
            .collect();

        for location in 0..self.pets.len() {
            self.pets[location].as_mut().map(|pet| pet.at(location));
        }
    }

    fn team_full(&self) -> bool {
        self.pets.iter().all(|pet| pet.is_some()) && self.pets.len() == 5
    }

    fn spawn(&mut self, position: usize, mut pet: Pet) {
        if self.team_full() {
            return;
        }
        pet.at(position);
        self.pets.insert(position, Some(pet))
    }

    pub fn react(&mut self, queue: &mut TriggerQueue, event: &Event) -> Result<(), TeamError> {
        if event.team == self.team {
            Ok(match &event.event {
                Faint(idx) => {
                    println!("Pet Fainted!");
                    self.pets.get_mut(*idx).ok_or(TeamError::PetMissing)?.take();
                }
                PleaseSpawn(position, pet) => {
                    self.spawn(*position, pet.clone());
                    queue.add(Event {
                        team: event.team,
                        event: Spawn(*position, pet.clone()),
                    })
                }
                _ => (),
            })
        } else {
            Ok(())
        }
    }
}

impl<'a> std::iter::IntoIterator for &'a mut Team {
    type Item = &'a mut Option<Pet>;
    type IntoIter = std::slice::IterMut<'a, Option<Pet>>;
    fn into_iter(self) -> Self::IntoIter {
        self.pets.iter_mut()
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_pet = |pet: &Option<Pet>| match pet {
            Some(pet) => write!(f, "{}", pet),
            None => write!(f, " "),
        };

        if self.team == Position::Left {
            self.pets.iter().rev().map(display_pet).collect()
        } else {
            self.pets.iter().map(display_pet).collect()
        }
    }
}

#[derive(Debug)]
pub enum TeamError {
    PetMissing,
}

impl std::fmt::Display for TeamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            Self::PetMissing => "Could not find pet at specified index",
        };
        writeln!(f, "Team Error: {}", error)
    }
}
impl std::error::Error for TeamError {}
