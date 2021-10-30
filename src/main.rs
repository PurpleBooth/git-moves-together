//! Find files that commonly appear in the same time slice or commit

#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

mod cli;
mod errors;
mod filters;
mod model;
mod repository;
mod statistics;

use std::path::PathBuf;

use futures::{future, stream, StreamExt, TryStreamExt};
use model::delta::Delta;
use repository::interface::Repository;
use time::Duration;

use crate::{
    errors::Error,
    repository::libgit2::LibGit2,
    statistics::{Statistics, Strategy},
};

#[tokio::main]
async fn main() -> Result<(), crate::errors::Error> {
    miette::set_panic_hook();
    let matches = cli::app().get_matches();

    let max_days_arg = matches.value_of("max-days-ago");
    let time_window_arg = matches.value_of("time-window-minutes");
    let git_repo_args = matches.values_of("git-repo").unwrap();

    let max_days = if let Some(value) = max_days_arg {
        Some(value.parse()?)
    } else {
        None
    };
    let strategy = if let Some(value) = time_window_arg {
        Strategy::CommitTime(Duration::minutes(value.parse()?))
    } else {
        Strategy::Hash
    };

    let deltas: Vec<Vec<Delta>> = stream::iter(git_repo_args.clone())
        .then(|path_str| read_deltas(max_days, path_str))
        .try_collect()
        .await?;

    let statistics = stream::iter(deltas)
        .zip(stream::iter(git_repo_args))
        .flat_map(|(delta, prefix)| stream::iter(add_prefix((&delta, prefix))))
        .fold(Statistics::default(), |statistics, change_delta| {
            statistics.add_delta(change_delta, &strategy)
        })
        .await;

    let coupling = statistics.coupling();
    if coupling.is_empty() {
        println!("0 files move together");
    } else {
        print!("{}", coupling);
    }

    Ok(())
}

fn add_prefix((delta, prefix): (&Vec<Delta>, &str)) -> Vec<Delta> {
    delta
        .iter()
        .map(|delta| delta.add_prefix(prefix))
        .collect::<Vec<_>>()
}

async fn read_deltas(max_days: Option<i64>, path_str: &str) -> Result<Vec<Delta>, Error> {
    let path = PathBuf::from(path_str);
    let commits = LibGit2::new(path.clone())?.commits_in_current_branch()?;
    stream::iter(commits.iter())
        .filter(|commit| future::ready(filters::within_time_limit(max_days, commit)))
        .map(|commit| LibGit2::new(path.clone())?.compare_with_parent(commit))
        .try_collect()
        .await
        .map_err(Error::from)
}
