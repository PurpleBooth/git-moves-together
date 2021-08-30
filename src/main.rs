use std::path::PathBuf;

use partial_application::partial;

use repository::interface::Repository;

use crate::errors::Error;
use crate::repository::libgit2::LibGit2;
use crate::statistics::{Statistics, Strategy};
use chrono::Duration;
use model::change_delta::ChangeDelta;

mod cli;
mod errors;
mod filters;
mod model;
mod repository;
mod statistics;

fn main() -> Result<(), crate::errors::Error> {
    let matches = cli::app().get_matches();
    let max_days = if let Some(value) = matches.value_of("max-days-ago") {
        Some(value.parse()?)
    } else {
        None
    };
    let strategy = if let Some(value) = matches.value_of("time-window-minutes") {
        Strategy::CommitTime(Duration::minutes(value.parse()?))
    } else {
        Strategy::Id
    };
    let deltas = matches
        .values_of("git-repo")
        .unwrap()
        .map(partial!(read_deltas => max_days, _))
        .collect::<Result<Vec<Vec<ChangeDelta>>, crate::errors::Error>>()?;

    let statistics = deltas
        .iter()
        .zip(matches.values_of("git-repo").unwrap())
        .flat_map(add_prefix)
        .fold(Statistics::default(), |acc, change_delta| {
            acc.add_delta(&change_delta, &strategy)
        });

    let coupling = statistics.coupling();
    if coupling.is_empty() {
        println!("0 files move together");
    } else {
        print!("{}", statistics);
    }

    Ok(())
}

fn add_prefix((delta, prefix): (&Vec<ChangeDelta>, &str)) -> Vec<ChangeDelta> {
    delta
        .iter()
        .map(partial!(ChangeDelta::add_prefix_from_filename=> _, prefix))
        .collect::<Vec<_>>()
}

fn read_deltas(max_days: Option<i64>, path_str: &str) -> Result<Vec<ChangeDelta>, Error> {
    let path = PathBuf::from(path_str);
    let repo = LibGit2::new(path)?;
    let delta = repo
        .snapshots_in_current_branch()?
        .iter()
        .filter(partial!(filters::within_time_limit => max_days, _))
        .map(partial!(Repository::compare_with_parent => &repo, _))
        .collect::<Result<Vec<_>, crate::repository::errors::Error>>()?;
    Ok(delta)
}
