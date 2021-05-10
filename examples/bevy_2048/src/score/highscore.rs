//! This module contains the implementation of the highscore struct.

use bevy::prelude::*;
use savefile::prelude::*;
use std::{env, error::Error, fs};

use super::Score;

/// This struct manages the highscore and saves it into a binary file
/// `best.bin`.
#[derive(Savefile)]
pub struct HighScore(pub u32);

impl Default for HighScore {
    /// Trys to load the highscore from the file.
    /// If it fails it sets the highscore to 0.
    fn default() -> Self {
        match fulldir("best.bin", false) {
            Ok(filepath) => match load_file::<Self>(&filepath, 0) {
                Ok(result) => return result,
                Err(e) => print_error(e, ErrorType::Load),
            },
            Err(e) => print_error(e, ErrorType::Load),
        }
        Self(0)
    }
}

/// This system is checking if a new highscore exists.
/// If there is a new highscre then it saves it into the binary file `best.bin`.
pub fn update_highscore(mut highscore: ResMut<HighScore>, score: Res<Score>) {
    if score.0 > highscore.0 {
        highscore.0 = score.0;

        match fulldir("best.bin", true) {
            Ok(filepath) => {
                if let Err(e) = save_file(&filepath, 0, &*highscore) {
                    print_error(e, ErrorType::Save)
                }
            },
            Err(e) => print_error(e, ErrorType::Save),
        }
    }
}

/// Gets `filename`, returns the full path `{path_to_exe}/data/{filename}`.
/// If `create_dir` is true, then the directory `data` is being created.
fn fulldir(filename: &str, create_dir: bool) -> Result<String, Box<dyn Error>> {
    let mut path = env::current_exe()?
        .parent() // removes the exe name from the path.
        .ok_or("Couldn't get parent")?
        .to_path_buf();

    path.push("data");

    if create_dir {
        fs::create_dir_all(&path)?;
    }

    path.push(filename);

    // Transforms the path into String.
    Ok(path
        .to_str()
        .ok_or("Couldn't parse os_str to str")?
        .to_owned())
}

/// Prints an error message into the console.
/// If in debug mode then it prints with more verbose.
fn print_error<T: std::fmt::Display>(e: T, err_type: ErrorType) {
    if cfg!(debug_assertions) {
        eprintln!("Couldn't {} highscore: {}", err_type, e);
    } else {
        eprintln!("Couldn't {} highscore", err_type);
    }
}

enum ErrorType {
    Load,
    Save,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Load => write!(f, "load"),
            Self::Save => write!(f, "save"),
        }
    }
}
