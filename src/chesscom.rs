use serde::{Deserialize};
use ureq::{get};

use crate::error::{EloGetError};
use crate::fmt::{KnownData};

#[derive(Deserialize)]
struct UserData {
    player_id: u64,
    username: String,
    #[serde(default = "default_title")]
    title: String,
}

#[derive(Deserialize)]
struct Stats {
    #[serde(default)]
    chess_bullet: ModeStats,
    #[serde(default)]
    chess_blitz: ModeStats,
    #[serde(default)]
    chess_rapid: ModeStats,
    #[serde(default)]
    chess_classical: ModeStats,
}

#[derive(Deserialize, Default)]
struct ModeStats {
    last: LastStats,
}

#[derive(Deserialize)]
struct LastStats {
    date: u64,
    rating: u32,
    rd: u32,
}

impl Default for LastStats {
    fn default() -> Self {
        LastStats {
            date: 0,
            rating: 1500,
            rd: 500,
        }
    }
}

impl From<(UserData, Stats)> for KnownData {
    fn from((data, stats): (UserData, Stats)) -> KnownData {
        KnownData {
            user: data.username,
            title: data.title,
            bullet_elo: stats.chess_bullet.last.rating,
            blitz_elo: stats.chess_blitz.last.rating,
            rapid_elo: stats.chess_rapid.last.rating,
            classical_elo: stats.chess_classical.last.rating,
        }
    }
}

pub fn get_data(user: &str) -> Result<KnownData, EloGetError> {
    let user_url = ["https://api.chess.com/pub/player/", user].concat();
    let stats_url = [&user_url, "/stats"].concat();
    let data = get(user_url)
        .call().map_err(|err| EloGetError::from(err))?
        .body_mut()
        .read_json::<UserData>().map_err(|err| EloGetError::from(err))?;
    let stats = get(stats_url)
        .call().map_err(|err| EloGetError::from(err))?
        .body_mut()
        .read_json::<Stats>().map_err(|err| EloGetError::from(err))?;
    Ok((data, stats).into())
}

fn default_title() -> String {
    String::from("None")
}
