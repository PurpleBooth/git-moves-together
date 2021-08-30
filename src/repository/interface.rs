use crate::model::change_delta::ChangeDelta;
use crate::model::snapshot::Snapshot;
use crate::model::snapshots::Snapshots;
use crate::repository::errors::Error;

pub(crate) trait Repository {
    fn snapshots_in_current_branch(&self) -> Result<Snapshots, Error>;
    fn compare_with_parent(&self, _: &Snapshot) -> Result<ChangeDelta, Error>;
}
