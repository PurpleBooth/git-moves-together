use std::convert::TryInto;
use std::path::PathBuf;

use git2::{Oid, Repository as LibGit2Repository, Sort, Tree};

use crate::model::change_delta::ChangeDelta;
use crate::model::changed_file_path::ChangedFilePath;
use crate::model::snapshot::Snapshot;
use crate::model::snapshot_id::SnapshotId;
use crate::model::snapshots::Snapshots;
use crate::repository::errors::Error;
use crate::repository::interface::Repository;

pub(crate) struct LibGit2 {
    repo: LibGit2Repository,
}

impl LibGit2 {
    pub(crate) fn new(path: PathBuf) -> Result<LibGit2, Error> {
        let repo = git2::Repository::open(path)?;

        Ok(LibGit2 { repo })
    }

    fn diff_with_parent(
        &self,
        snapshot_tree: &Tree,
        parent: SnapshotId,
    ) -> Result<Vec<ChangedFilePath>, Error> {
        let parent_id = parent.try_into()?;
        let parent_tree = self.repo.find_commit(parent_id)?.tree()?;
        let diff_to_parent =
            self.repo
                .diff_tree_to_tree(Some(&parent_tree), Some(snapshot_tree), None)?;
        Ok(diff_to_parent.deltas().map(|delta| delta.into()).collect())
    }

    fn to_snapshot_id(&self, commit_oid: Oid) -> Result<Snapshot, Error> {
        self.repo
            .find_commit(commit_oid)
            .map(Snapshot::from)
            .map_err(Error::from)
    }
}

impl Repository for LibGit2 {
    fn snapshots_in_current_branch(&self) -> Result<Snapshots, Error> {
        let mut walker = self.repo.revwalk()?;
        walker.set_sorting(Sort::TIME & Sort::TOPOLOGICAL)?;
        walker.push_head()?;

        walker
            .into_iter()
            .map(|commit_id_result| {
                commit_id_result
                    .map_err(Error::from)
                    .and_then(|commit_id_result| self.to_snapshot_id(commit_id_result))
            })
            .collect::<Result<Vec<Snapshot>, Error>>()
            .map(Snapshots::from)
    }

    fn compare_with_parent(&self, snapshot: &Snapshot) -> Result<ChangeDelta, Error> {
        let snapshot_tree = snapshot
            .id()
            .try_into()
            .and_then(|oid| self.repo.find_commit(oid))
            .and_then(|commit| commit.tree())?;

        let changes = snapshot
            .parents()
            .into_iter()
            .map(|parent| self.diff_with_parent(&snapshot_tree, parent))
            .reduce(flatten_or_first_err)
            .unwrap_or_else(|| Ok(vec![]));

        Ok(ChangeDelta::new(
            snapshot.id(),
            snapshot.timestamp(),
            changes?,
        ))
    }
}

fn flatten_or_first_err(
    acc: Result<Vec<ChangedFilePath>, Error>,
    item: Result<Vec<ChangedFilePath>, Error>,
) -> Result<Vec<ChangedFilePath>, Error> {
    match (acc, item) {
        (Ok(acc), Ok(item)) => Ok([acc, item].concat()),
        (Err(err), _) | (_, Err(err)) => Err(err),
    }
}
