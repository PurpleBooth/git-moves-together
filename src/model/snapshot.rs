use chrono::{DateTime, TimeZone, Utc};

use crate::model::snapshot_id::SnapshotId;
use git2::Commit;

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

impl From<Commit<'_>> for Snapshot {
    fn from(commit: Commit) -> Self {
        Snapshot::new(
            commit.id().into(),
            commit
                .parents()
                .map(|parent_commit| parent_commit.id().into())
                .collect(),
            Utc.timestamp(commit.time().seconds(), 0),
        )
    }
}
