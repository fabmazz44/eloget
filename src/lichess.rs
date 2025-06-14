use serde::{Deserialize};
use ureq::{get};

use crate::error::{EloGetError};
use crate::fmt::{KnownData};

#[derive(Deserialize)]
struct UserData {
    id: String,
    username: String,
    #[serde(default = "default_title")]
    title: String,
    perfs: Perfs,
}

#[derive(Deserialize)]
struct Perfs {
    bullet: Mode,
    blitz: Mode,
    rapid: Mode,
    classical: Mode,
    correspondence: Mode,
}

#[derive(Deserialize)]
struct Mode {
    games: u32,
    rating: u32,
    rd: u32,
    prog: i32,
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            games: 0,
            rating: 1500,
            rd: 500,
            prog: 0,
        }
    }
}

impl From<UserData> for KnownData {
    fn from(udata: UserData) -> KnownData {
        KnownData {
            user: udata.username,
            title: udata.title,
            bullet_elo: udata.perfs.bullet.rating,
            blitz_elo: udata.perfs.blitz.rating,
            rapid_elo: udata.perfs.rapid.rating,
            classical_elo: udata.perfs.classical.rating,
        }
    }
}

pub fn get_data(user: &str) -> Result<KnownData, EloGetError> {
    let url = ["https://lichess.org/api/user/", user].concat();
    println!("{url}");
    let data = get(url)
        .call().map_err(|_| EloGetError::HttpError)?
        .body_mut()
        .read_json::<UserData>().map_err(|_| EloGetError::JsonError)?;
    Ok(data.into())
}

fn default_title() -> String {
    String::from("None")
}
