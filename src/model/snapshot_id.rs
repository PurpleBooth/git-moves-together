use git2::Oid;
use std::convert::TryFrom;

impl From<SnapshotId> for String {
    fn from(snapshot_id: SnapshotId) -> Self {
        snapshot_id.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct SnapshotId(String);

impl From<&str> for SnapshotId {
    fn from(snapshot_id: &str) -> Self {
        SnapshotId(String::from(snapshot_id))
    }
}

impl From<String> for SnapshotId {
    fn from(snapshot_id: String) -> Self {
        SnapshotId(snapshot_id)
    }
}

impl From<Oid> for SnapshotId {
    fn from(oid: Oid) -> Self {
        SnapshotId(oid.to_string())
    }
}

impl TryFrom<SnapshotId> for Oid {
    type Error = git2::Error;

    fn try_from(value: SnapshotId) -> Result<Self, Self::Error> {
        Oid::from_str(&String::from(value))
    }
}
