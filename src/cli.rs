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
    #[clap(short = 't', long = "time-window-minutes", env = "TIME_WINDOW_MINUTES", value_parser = clap::value_parser!(i64).range(1..))]
    pub time_window_minutes: Option<i64>,
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::Args;

    #[test]
    fn rejects_zero_time_window() {
        let result = Args::try_parse_from(["git-moves-together", "--time-window-minutes", "0"]);
        assert!(
            result.is_err(),
            "A time window of zero minutes causes a division-by-zero panic and must be rejected"
        );
    }

    #[test]
    fn rejects_negative_time_window() {
        let result = Args::try_parse_from(["git-moves-together", "--time-window-minutes", "-5"]);
        assert!(
            result.is_err(),
            "A negative time window is nonsensical and must be rejected"
        );
    }
}
