use std::fmt::{Display, Formatter, Error};

pub struct KnownData {
    pub user: String,
    pub title: String,
    pub bullet_elo: u32,
    pub blitz_elo: u32,
    pub rapid_elo: u32,
    pub classical_elo: u32,
}

pub enum FormatError {
    UnexpectedEmpty,
    UnexpectedChar {
        found: char,
        expected: char,
    },
    UnrecognizedFormat(String),
}

impl Display for FormatError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            FormatError::UnexpectedEmpty => write!(f, "Unexpected end of string."),
            FormatError::UnexpectedChar { found, expected } => write!(f, "Epected char '{expected}', found '{found}'."),
            FormatError::UnrecognizedFormat(param) => write!(f, "Value {param} is not recognized."),
        }
    }
}

pub fn format<T>(fmt: &mut T, data: KnownData) -> Result<String, FormatError> where T: Iterator<Item = char> {
    match fmt.next() {
        None => Ok(String::new()),
        Some('{') => {
            let word = handle_word(parse_word(fmt)?, &data)?;
            let rest = format(fmt, data)?;
            Ok([word, rest].concat())
        }
        Some(c) => {
            let mut result = String::new();
            result.push(c);
            let rest = format(fmt, data)?;
            Ok([result, rest].concat())
        }
    }
}

fn parse_word<T>(fmt: &mut T) -> Result<String, FormatError> where T: Iterator<Item = char> {
    match fmt.next() {
        None => Err(FormatError::UnexpectedEmpty),
        Some('}') => Ok(String::new()),
        Some(c) => {
            let mut result = String::new();
            result.push(c);
            let rest = parse_word(fmt)?;
            Ok([result, rest].concat())
        }
    }
}

fn handle_word(word: String, data: &KnownData) -> Result<String, FormatError> {
    match word.as_str() {
        "user" => Ok(data.user.to_string()),
        "title" => Ok(data.title.to_string()),
        "bullet_elo" => Ok(data.bullet_elo.to_string()),
        "blitz_elo" => Ok(data.blitz_elo.to_string()),
        "rapid_elo" => Ok(data.rapid_elo.to_string()),
        "classical_elo" => Ok(data.classical_elo.to_string()),
        _ => Err(FormatError::UnrecognizedFormat(word)),
    }
}
