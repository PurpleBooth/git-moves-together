use std::collections::BTreeSet;
use std::ffi::OsStr;

use std::path::PathBuf;

use chrono::{DateTime, Utc};

use crate::model::changed_file::ChangedFile;
use crate::model::hash::Hash;

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub(crate) struct Delta {
    changes: BTreeSet<ChangedFile>,
    timestamp: DateTime<Utc>,
    hash: Hash,
}

impl Delta {
    pub(crate) fn merge(&self, other: &Delta) -> Delta {
        Delta {
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
    pub(crate) fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    pub(crate) fn new(hash: Hash, timestamp: DateTime<Utc>, changes: Vec<ChangedFile>) -> Delta {
        Delta {
            changes: changes.into_iter().collect(),
            timestamp,
            hash,
        }
    }

    pub(crate) fn add_str_prefix(&self, prefix: &str) -> Delta {
        Delta {
            changes: self
                .changes
                .iter()
                .map(|path| path.add_prefix(prefix))
                .collect(),
            timestamp: self.timestamp,
            hash: self.hash.clone(),
        }
    }

    pub(crate) fn add_prefix(&self, path: &str) -> Delta {
        self.clone().add_str_prefix(
            PathBuf::from(path)
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap_or(path),
        )
    }
}

impl IntoIterator for Delta {
    type Item = ChangedFile;
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.changes.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::model::delta::Delta;

    #[test]
    fn can_put_a_prefix_on_everything_in() {
        let actual: Vec<String> = Delta::new(
            "sample-id".into(),
            Utc::now(),
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
