use std::num::ParseIntError;

use thiserror::Error as ThisError;

use crate::repository::errors::Error as Repository;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("git repository problem")]
    Repository(#[from] Repository),
    #[error("failed to parse days")]
    DaysParse(#[from] ParseIntError),
}
