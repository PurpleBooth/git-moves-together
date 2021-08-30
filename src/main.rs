use std::path::PathBuf;

use partial_application::partial;

use repository::interface::Repository;

use crate::errors::Error;
use crate::repository::interface::ChangeDelta;
use crate::repository::libgit2::LibGit2;
use crate::statistics::Statistics;

mod cli;
mod errors;
mod repository;
mod statistics;

fn main() -> Result<(), crate::errors::Error> {
    let matches = cli::app().get_matches();
    let deltas = matches
        .values_of("git-repo")
        .unwrap()
        .map(read_deltas)
        .collect::<Result<Vec<Vec<ChangeDelta>>, crate::errors::Error>>()?;

    let statistics = deltas
        .iter()
        .flatten()
        .fold(Statistics::default(), Statistics::add_delta);

    let coupling = statistics.coupling();
    if coupling.is_empty() {
        println!("0 files move together");
    } else {
        println!("{}", statistics);
        println!();
        println!("{} files move together", coupling.len());
    }

    Ok(())
}

fn read_deltas(path_str: &str) -> Result<Vec<ChangeDelta>, Error> {
    let path = PathBuf::from(path_str);
    let repo = LibGit2::new(path)?;
    let delta = repo
        .snapshots_in_current_branch()?
        .iter()
        .map(partial!(Repository::compare_with_parent => &repo, _))
        .collect::<Result<Vec<_>, crate::repository::errors::Error>>()?;
    Ok(delta)
}
