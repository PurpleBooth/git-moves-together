use chrono::Utc;

use crate::model::snapshot::Snapshot;

pub(crate) fn within_time_limit(max_days: Option<i64>, snapshot: &Snapshot) -> bool {
    match max_days {
        None => true,
        Some(max_days) => chrono::Duration::days(max_days)
            .gt(&Utc::now().signed_duration_since(snapshot.timestamp())),
    }
}
