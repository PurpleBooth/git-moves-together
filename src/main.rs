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

use clap::Parser;
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
    cli::Args,
    errors::Error,
    repository::libgit2::LibGit2,
    statistics::{Statistics, Strategy},
};

#[tokio::main]
async fn main() -> Result<(), crate::errors::Error> {
    miette::set_panic_hook();
    let args = Args::parse();

    let strategy = args.time_window_minutes.map_or(Strategy::Hash, |value| {
        Strategy::CommitTime(Duration::minutes(value))
    });

    let deltas: Vec<Vec<Delta>> = stream::iter(args.git_repo.iter())
        .then(|path_str| read_deltas(args.max_days_ago, path_str))
        .try_collect()
        .await?;

    let statistics = deltas
        .into_iter()
        .zip(args.git_repo)
        .flat_map(|(delta, prefix)| add_prefix((&delta, &prefix)))
        .fold(Statistics::default(), |statistics, change_delta| {
            statistics.add_delta(&change_delta, &strategy)
        });

    let coupling = statistics.coupling();
    if coupling.is_empty() {
        println!("0 files move together");
    } else {
        print!("{coupling}");
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
