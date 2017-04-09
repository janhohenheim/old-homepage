extern crate diesel;

use self::diesel::result::Error as DatabaseError;

use std::error;
use std::fmt;
use std::convert::From;

#[derive(Debug)]
pub enum QuizError {
    DatabaseError(DatabaseError),
    JokerUnavailable,
    GameAlreadyFinished,
    NoGameInProgress,
    GameStillInProgress,
    StateError,
    OutOfResources,
}

impl fmt::Display for QuizError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QuizError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
            QuizError::JokerUnavailable => write!(f, "Joker error: Tried to use unavailable Joker"),
            QuizError::GameAlreadyFinished => {
                write!(f,
                       "Game already finished error: Tried to interact with a game that has already been finished")
            }
            QuizError::NoGameInProgress => {
                write!(f,
                       "No game in progress error: Tried to play without starting a game first")
            }
            QuizError::GameStillInProgress => {
                write!(f,
                       "Game still in progress error: Tried to start game while old one was not finished yet")
            }
            QuizError::StateError => {
                write!(f,
                       "State error: Found game in a corrupt state, e.g. no available categories")
            }
            QuizError::OutOfResources => {
                write!(f, "Out of resources error: Answered all possible questions")
            }
        }
    }
}

impl error::Error for QuizError {
    fn description(&self) -> &str {
        match *self {
            QuizError::DatabaseError(ref err) => err.description(),
            QuizError::JokerUnavailable => "Joker unavailable error",
            QuizError::GameAlreadyFinished => "Game already finished error",
            QuizError::GameStillInProgress => "Game still in progress error",
            QuizError::NoGameInProgress => "No game in progress error",
            QuizError::StateError => "State error",
            QuizError::OutOfResources => "Out of resources error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            QuizError::DatabaseError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<DatabaseError> for QuizError {
    fn from(err: DatabaseError) -> Self {
        QuizError::DatabaseError(err)
    }
}
