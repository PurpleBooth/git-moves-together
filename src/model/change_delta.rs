use std::collections::BTreeSet;
use std::ffi::OsStr;

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use partial_application::partial;

use crate::model::changed_file_path::ChangedFilePath;
use crate::model::snapshot_id::SnapshotId;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ChangeDelta {
    changes: BTreeSet<ChangedFilePath>,
    timestamp: DateTime<Utc>,
    id: SnapshotId,
}

impl ChangeDelta {
    pub(crate) fn id(&self) -> SnapshotId {
        self.id.clone()
    }
    pub(crate) fn new(
        id: SnapshotId,
        timestamp: DateTime<Utc>,
        changes: Vec<ChangedFilePath>,
    ) -> ChangeDelta {
        ChangeDelta {
            changes: changes.into_iter().collect(),
            timestamp,
            id,
        }
    }

    pub(crate) fn contains(&self, item: &ChangedFilePath) -> bool {
        self.changes.contains(item)
    }

    pub(crate) fn add_prefix(self, prefix: &str) -> ChangeDelta {
        ChangeDelta {
            changes: self
                .changes
                .iter()
                .map(partial!(ChangedFilePath::add_prefix => _, prefix))
                .collect(),
            timestamp: self.timestamp,
            id: self.id,
        }
    }

    pub(crate) fn add_prefix_from_filename(&self, path: &str) -> ChangeDelta {
        self.clone().add_prefix(
            PathBuf::from(path)
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap_or(path),
        )
    }
}

impl IntoIterator for ChangeDelta {
    type Item = ChangedFilePath;
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.changes.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::model::change_delta::ChangeDelta;

    #[test]
    fn can_put_a_prefix_on_everything_in() {
        let actual: Vec<String> = ChangeDelta::new(
            "sample-id".into(),
            Utc::now(),
            vec!["item 1".into(), "item 2".into(), "item 3".into()],
        )
        .add_prefix("Something")
        .into_iter()
        .map(|x| x.into())
        .collect();

        assert_eq!(
            actual,
            vec!["Something@item 1", "Something@item 2", "Something@item 3"]
        );
    }
    #[test]
    fn can_tell_if_something_is_in_this_delta() {
        let actual = ChangeDelta::new(
            "sample-id".into(),
            Utc::now(),
            vec!["item 1".into(), "item 2".into(), "item 3".into()],
        );

        assert!(actual.contains(&"item 1".into()));
        assert!(!actual.contains(&"none existing".into()));
    }
}
