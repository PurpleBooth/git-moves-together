use std::sync::{MutexGuard, PoisonError};

use git2::Repository;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read git repository")]
    LibGit2(#[from] git2::Error),
    #[error("failed to lock mutex")]
    MutexPoison(#[from] PoisonError<MutexGuard<'static, Repository>>),
}
