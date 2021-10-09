use crate::{
    model::{commit::Commit, commits::Commits, delta::Delta},
    repository::errors::Error,
};

pub trait Repository {
    fn commits_in_current_branch(&self) -> Result<Commits, Error>;
    fn compare_with_parent(&self, _: &Commit) -> Result<Delta, Error>;
}
