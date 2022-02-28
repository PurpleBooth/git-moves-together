use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// A repository to analyse
    #[clap(env, default_values = &["."])]
    pub git_repo: Vec<String>,
    /// Ignore deltas older than the given days
    #[clap(short = 'd', long = "from-days", env = "MAX_DAYS_AGO")]
    pub max_days_ago: Option<i64>,
    /// Group commits by similar time window rather than by commit id
    #[clap(short = 't', long = "time-window-minutes", env = "TIME_WINDOW_MINUTES")]
    pub time_window_minutes: Option<i64>,
}
