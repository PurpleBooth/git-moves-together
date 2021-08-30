use partial_application::partial;

use crate::model::change_delta::ChangeDelta;
use crate::model::changed_file_path::ChangedFilePath;
use crate::model::snapshot::Snapshot;
use crate::model::snapshot_id::SnapshotId;
use crate::model::snapshots::Snapshots;
use crate::repository::errors::Error;
use crate::repository::interface::Repository;

pub(crate) struct InMemory {
    snapshots: Snapshots,
    changed_filenames: Vec<(SnapshotId, ChangedFilePath)>,
}

impl InMemory {
    // Note, this is not actually dead, but rather proof that we can swap out our git provider
    #[allow(dead_code)]
    pub(crate) fn new(
        snapshots: Snapshots,
        changed_filenames: Vec<(SnapshotId, ChangedFilePath)>,
    ) -> InMemory {
        InMemory {
            snapshots,
            changed_filenames,
        }
    }

    fn take_file_if_id_matches(
        snapshot: &Snapshot,
        (id, file): (SnapshotId, ChangedFilePath),
    ) -> Option<ChangedFilePath> {
        if snapshot.has_id(&id) {
            Some(file)
        } else {
            None
        }
    }
}

impl Repository for InMemory {
    fn snapshots_in_current_branch(&self) -> Result<Snapshots, Error> {
        Ok(self.snapshots.clone())
    }

    fn compare_with_parent(&self, snapshot: &Snapshot) -> Result<ChangeDelta, Error> {
        Ok(self
            .changed_filenames
            .clone()
            .into_iter()
            .filter_map(partial!(InMemory::take_file_if_id_matches => snapshot, _))
            .collect())
    }
}
