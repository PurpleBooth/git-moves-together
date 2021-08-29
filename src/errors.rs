use crate::repository::errors::Error as Repository;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("git repository problem")]
    Repository(#[from] Repository),
}
