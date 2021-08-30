impl From<SnapshotId> for String {
    fn from(snapshot_id: SnapshotId) -> Self {
        snapshot_id.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
