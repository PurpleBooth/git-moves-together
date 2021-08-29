use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read git repository")]
    LibGit2(#[from] git2::Error),
}
