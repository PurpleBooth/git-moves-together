use std::path::PathBuf;

use partial_application::partial;

use repository::interface::Repository;

use crate::repository::errors::Error as RepositoryError;
use crate::repository::libgit2::LibGit2;
use crate::statistics::Statistics;

mod cli;
mod errors;
mod repository;
mod statistics;

fn main() -> Result<(), crate::errors::Error> {
    let matches = cli::app().get_matches();
    let path = PathBuf::from(matches.value_of("git-repo").unwrap());

    let repo = LibGit2::new(path)?;
    let statistics = repo
        .snapshots_in_current_branch()?
        .iter()
        .map(partial!(Repository::compare_with_parent => &repo, _))
        .collect::<Result<Vec<_>, RepositoryError>>()?
        .iter()
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
