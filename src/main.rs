use std::env;
use std::fmt::{Display, Formatter};

use eloget::error::{EloGetError};
use eloget::lichess;
use eloget::chesscom;
use eloget::fmt::{format};

#[derive(Debug)]
struct Options {
    user: String,
    fmt: String,
    src: String,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            user: String::from("fabmazz"),
            fmt: String::from("{user}: {blitz_elo} blitz"),
            src: String::from("chesscom"),
        }
    }
}

fn main() {
    let flags = env::args().collect::<Vec<_>>();

    let options = match parse_options(flags[1..].to_vec()) {
        Ok(options) => options,
        Err(err) => {
            eprintln!("{err}");
            return
        }
    };

    let data_result = match options.src.as_str() {
        "lichess" => lichess::get_data(&options.user),
        "chesscom" => chesscom::get_data(&options.user),
        _ => Err(EloGetError::InvalidSource)
    };

    let data = match data_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{err}");
            return
        }
    };

    match format(&mut options.fmt.chars(), data) {
        Ok(string) => println!("{string}"),
        Err(err) => eprintln!("{err}"),
    }
}

fn parse_options(flags: Vec<String>) -> Result<Options, FlagParseError> {
    let mut options = Options::default();
    for flag in flags {
        let (name, value) = parse_flag(flag.clone())?;
        match name.as_str() {
            "usr" => options.user = value,
            "fmt" => options.fmt= value,
            "src" => options.src = value,
            _ => return Err(FlagParseError::InvalidFlag(flag.clone()))
        }
    }

    return Ok(options);
}

fn parse_flag(flag: String) -> Result<(String, String), FlagParseError> {
    let char1 = flag.chars().nth(0).ok_or(FlagParseError::EmptyFlag)?;
    if char1 != '-' {
        return Err(FlagParseError::MissingMinus(flag));
    }
    let mut words = flag[1..].split("=");
    let name = words.next().ok_or(FlagParseError::TooFewWords(flag.clone()))?;
    let value = words.next().ok_or(FlagParseError::TooFewWords(flag.clone()))?;
    Ok((String::from(name), String::from(value)))
}

pub enum FlagParseError {
    EmptyFlag,
    MissingMinus(String),
    TooFewWords(String),
    InvalidFlag(String),
}

impl Display for FlagParseError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FlagParseError::EmptyFlag => write!(fmt, "The flag is empty."),
            FlagParseError::MissingMinus(flag) => write!(fmt, "The flag {flag} does not start with a minus."),
            FlagParseError::TooFewWords(flag) => write!(fmt, "Flag {flag} does not contain enough words."),
            FlagParseError::InvalidFlag(flag) => write!(fmt, "Flag {flag} is not recognized." ),
        }
    }
}
