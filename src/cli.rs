use clap::{crate_authors, crate_version, App, Arg};
use std::env;

pub fn app() -> App<'static> {
    App::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("git-repo")
                .takes_value(true)
                .multiple_values(true)
                .default_values(&["."])
                .about("A repository to analyse")
                .env("GIT_REPO"),
        )
        .arg(
            Arg::new("max-days-ago")
                .short('d')
                .long("from-days")
                .takes_value(true)
                .about("Ignore deltas older than the given days")
                .env("MAX_DAYS_AGO"),
        )
        .arg(
            Arg::new("time-window-minutes")
                .short('t')
                .long("time-window-minutes")
                .takes_value(true)
                .about("Group commits by similar time window rather than by commit id")
                .env("TIME_WINDOW_MINUTES"),
        )
}
