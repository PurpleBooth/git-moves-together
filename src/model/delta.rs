use std::{collections::BTreeSet, ffi::OsStr, path::PathBuf};

use time::OffsetDateTime;

use crate::model::{changed_file::ChangedFile, hash::Hash};

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub struct Delta {
    changes: BTreeSet<ChangedFile>,
    timestamp: OffsetDateTime,
    hash: Hash,
}

impl Delta {
    pub(crate) fn merge(&self, other: &Self) -> Self {
        Self {
            hash: self.hash(),
            timestamp: self.timestamp(),
            changes: self
                .changes
                .union(&other.changes)
                .cloned()
                .collect::<BTreeSet<_>>(),
        }
    }

    pub(crate) fn hash(&self) -> Hash {
        self.hash.clone()
    }

    pub(crate) const fn timestamp(&self) -> OffsetDateTime {
        self.timestamp
    }

    pub(crate) fn new(hash: Hash, timestamp: OffsetDateTime, changes: Vec<ChangedFile>) -> Self {
        Self {
            changes: changes.into_iter().collect(),
            timestamp,
            hash,
        }
    }

    pub(crate) fn add_str_prefix(&self, prefix: &str) -> Self {
        Self {
            changes: self
                .changes
                .iter()
                .map(|path| path.add_prefix(prefix))
                .collect(),
            timestamp: self.timestamp,
            hash: self.hash.clone(),
        }
    }

    pub(crate) fn add_prefix(&self, path: &str) -> Self {
        self.clone().add_str_prefix(
            PathBuf::from(path)
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap_or(path),
        )
    }
}

impl IntoIterator for Delta {
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;
    type Item = ChangedFile;

    fn into_iter(self) -> Self::IntoIter {
        self.changes.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;

    use crate::model::delta::Delta;

    #[test]
    fn can_put_a_prefix_on_everything_in() {
        let actual: Vec<String> = Delta::new(
            "sample-id".into(),
            OffsetDateTime::now_utc(),
            vec!["item 1".into(), "item 2".into(), "item 3".into()],
        )
        .add_str_prefix("Something")
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

        assert_eq!(
            actual,
            vec!["Something@item 1", "Something@item 2", "Something@item 3"]
        );
    }
}
