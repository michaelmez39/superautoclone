use crate::pet::Pet;
use crate::team::Team;
use unicode_width::UnicodeWidthStr;

const EMPTY_SHELF: &'static str = "🪑         ";
pub trait Emojify {
    fn icon(&self) -> char;
}

pub struct Emoji<'a, T>(pub &'a T);

impl<'a, E> std::fmt::Display for Emoji<'a, E>
where
    E: Emojify,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.icon())
    }
}

pub trait DisplayShort {
    fn display_shortened(&self) -> String;
}
pub struct Short<'a, T>(pub &'a T);

impl<'a, S> std::fmt::Display for Short<'a, S>
where
    S: DisplayShort,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display_shortened())
    }
}

impl DisplayShort for Option<&Pet> {
    fn display_shortened(&self) -> String {
        self.as_ref().map_or(EMPTY_SHELF.to_string(), |pet| {
            format!("{} 🗡️ {} ❤️ {}", pet.icon(), pet.attack(), pet.health())
        })
    }
}

pub fn team_shop(team: &Team) -> String {
    border(
        team.pets.iter().fold(String::new(), |acc, spot| {
            format!("{} {}", acc, Short(&spot.as_ref()))
        }),
        Some("Current Team".to_string()),
    )
}

pub fn border(text: String, header: Option<String>) -> String {
    let lines = text.split("\n");
    let width = lines
        .map(|l| l.width())
        .chain(header.as_ref().map(|h| h.width()))
        .max()
        .unwrap_or(0);

    let central_text = text
        .split("\n")
        .map(|line| format!("│{}{}│", line, " ".repeat(width - line.width())))
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "┌{:─^width$}┐\n{}\n└{:─^width$}┘",
        header.unwrap_or("".to_string()),
        central_text,
        "",
        width = width
    )
}
