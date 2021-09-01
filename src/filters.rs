use chrono::Utc;

use crate::model::commit::Commit;

pub(crate) fn within_time_limit(max_days: Option<i64>, commit: &Commit) -> bool {
    match max_days {
        None => true,
        Some(max_days) => chrono::Duration::days(max_days)
            .gt(&Utc::now().signed_duration_since(commit.timestamp())),
    }
}
