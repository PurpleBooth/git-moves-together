use crate::repository::errors::Error;
use chrono::{DateTime, Utc};
use partial_application::partial;
use std::iter::FromIterator;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct SnapshotId(String);

impl From<&str> for SnapshotId {
    fn from(snapshot_id: &str) -> Self {
        SnapshotId(String::from(snapshot_id))
    }
}

impl From<SnapshotId> for String {
    fn from(snapshot_id: SnapshotId) -> Self {
        snapshot_id.0
    }
}

impl From<String> for SnapshotId {
    fn from(snapshot_id: String) -> Self {
        SnapshotId(snapshot_id)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) struct Snapshot {
    id: SnapshotId,
    timestamp: DateTime<Utc>,
    parents: Vec<SnapshotId>,
}

impl Snapshot {
    pub(crate) fn new(
        id: SnapshotId,
        parents: Vec<SnapshotId>,
        timestamp: DateTime<Utc>,
    ) -> Snapshot {
        Snapshot {
            id,
            timestamp,
            parents,
        }
    }

    pub(crate) fn id(&self) -> SnapshotId {
        self.id.clone()
    }

    pub(crate) fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub(crate) fn parents(&self) -> Vec<SnapshotId> {
        self.parents.clone()
    }

    pub(crate) fn has_id(&self, id: &SnapshotId) -> bool {
        &(self.id) == id
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Snapshots(Vec<Snapshot>);

impl From<Vec<Snapshot>> for Snapshots {
    fn from(source: Vec<Snapshot>) -> Self {
        Snapshots(source.into_iter().collect())
    }
}

impl IntoIterator for Snapshots {
    type Item = Snapshot;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Snapshots {
    pub(crate) fn iter(&self) -> std::slice::Iter<Snapshot> {
        self.0.iter()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub(crate) struct ChangedFilePath(Option<String>, String);

impl ChangedFilePath {
    fn add_prefix(&self, prefix: &str) -> ChangedFilePath {
        ChangedFilePath(Some(prefix.into()), self.1.clone())
    }
}

impl From<ChangedFilePath> for String {
    fn from(path: ChangedFilePath) -> Self {
        match path {
            ChangedFilePath(None, file) => file,
            ChangedFilePath(Some(repo), file) => format!("{}@{}", repo, file),
        }
    }
}

impl From<&str> for ChangedFilePath {
    fn from(path: &str) -> Self {
        ChangedFilePath(None, String::from(path))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ChangeDelta(Vec<ChangedFilePath>);

impl ChangeDelta {
    pub(crate) fn add_prefix(self, prefix: &str) -> ChangeDelta {
        ChangeDelta(
            self.0
                .iter()
                .map(partial!(ChangedFilePath::add_prefix => _, prefix))
                .collect(),
        )
    }
}

impl IntoIterator for &ChangeDelta {
    type Item = ChangedFilePath;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

impl From<Vec<String>> for ChangeDelta {
    fn from(paths: Vec<String>) -> Self {
        ChangeDelta(paths.into_iter().map(ChangedFilePath::from).collect())
    }
}

impl From<Vec<&str>> for ChangeDelta {
    fn from(paths: Vec<&str>) -> Self {
        ChangeDelta(paths.into_iter().map(ChangedFilePath::from).collect())
    }
}

impl From<Vec<ChangedFilePath>> for ChangeDelta {
    fn from(paths: Vec<ChangedFilePath>) -> Self {
        ChangeDelta(paths.into_iter().collect())
    }
}

impl ChangeDelta {
    pub(crate) fn contains(&self, item: &ChangedFilePath) -> bool {
        self.0.contains(item)
    }
}

impl FromIterator<ChangedFilePath> for ChangeDelta {
    fn from_iter<T: IntoIterator<Item = ChangedFilePath>>(iter: T) -> Self {
        let mut items = Vec::new();
        for item in iter {
            items.push(item);
        }

        ChangeDelta(items)
    }
}

impl From<String> for ChangedFilePath {
    fn from(item: String) -> Self {
        ChangedFilePath(None, item)
    }
}

pub(crate) trait Repository {
    fn snapshots_in_current_branch(&self) -> Result<Snapshots, Error>;
    fn compare_with_parent(&self, _: &Snapshot) -> Result<ChangeDelta, Error>;
}
