use std::ops::Sub;

use time::{Duration, OffsetDateTime};

use crate::model::commit::Commit;

pub fn within_time_limit(max_days: Option<i64>, commit: &Commit) -> bool {
    max_days.is_none_or(|max_days| {
        Duration::days(max_days).gt(&OffsetDateTime::now_utc().sub(commit.timestamp()))
    })
}
