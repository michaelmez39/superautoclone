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

pub fn border(text: String, header: Option<String>) -> String {
    let lines = text.split("\n");
    let width = lines.map(|l| l.chars().count()).max().unwrap_or(0);

    let central_text = text
        .split("\n")
        .map(|line| format!("│{}│", line))
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
