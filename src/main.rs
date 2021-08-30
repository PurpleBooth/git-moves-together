use std::path::PathBuf;

use partial_application::partial;

use repository::interface::Repository;

use crate::errors::Error;
use crate::repository::interface::{ChangeDelta, Snapshot};
use crate::repository::libgit2::LibGit2;
use crate::statistics::Statistics;
use chrono::Utc;
use std::ffi::OsStr;

mod cli;
mod errors;
mod repository;
mod statistics;

fn main() -> Result<(), crate::errors::Error> {
    let matches = cli::app().get_matches();
    let max_days = if let Some(value) = matches.value_of("max-days-ago") {
        Some(value.parse()?)
    } else {
        None
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

fn add_prefix((delta, prefix): (&Vec<ChangeDelta>, &str)) -> Vec<ChangeDelta> {
    delta
        .iter()
        .map(|x| {
            x.clone().add_prefix(
                PathBuf::from(prefix)
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap_or(prefix),
            )
        })
        .collect::<Vec<_>>()
}

fn read_deltas(max_days: Option<i64>, path_str: &str) -> Result<Vec<ChangeDelta>, Error> {
    let path = PathBuf::from(path_str);
    let repo = LibGit2::new(path)?;
    let delta = repo
        .snapshots_in_current_branch()?
        .iter()
        .filter(partial!(within_time_limit => max_days, _))
        .map(partial!(Repository::compare_with_parent => &repo, _))
        .collect::<Result<Vec<_>, crate::repository::errors::Error>>()?;
    Ok(delta)
}

fn within_time_limit(max_days: Option<i64>, snapshot: &Snapshot) -> bool {
    match max_days {
        None => true,
        Some(max_days) => chrono::Duration::days(max_days)
            .gt(&Utc::now().signed_duration_since(snapshot.timestamp())),
    }
}
