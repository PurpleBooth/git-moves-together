use std::path::PathBuf;

use git2::{Commit, DiffDelta, DiffOptions, Oid, Repository as LibGit2Repository, Sort};

use crate::repository::errors::Error;
use crate::repository::interface::{
    ChangeDelta, ChangedFilePath, Repository, Snapshot, SnapshotId, Snapshots,
};

pub(crate) struct LibGit2 {
    repo: LibGit2Repository,
}

impl LibGit2 {
    pub(crate) fn new(path: PathBuf) -> Result<LibGit2, Error> {
        let repo = git2::Repository::open(path)?;

        Ok(LibGit2 { repo })
    }

    fn path_of_changed_file(x: &DiffDelta) -> Option<ChangedFilePath> {
        DiffDelta::new_file(x)
            .path()
            .and_then(std::path::Path::to_str)
            .map(ChangedFilePath::from)
    }

    fn commit_to_snapshot_id(parent: &Commit) -> SnapshotId {
        parent.id().to_string().into()
    }

    fn oid_from_snapshot(snapshot: &Snapshot) -> Result<Oid, Error> {
        Oid::from_str(&String::from(snapshot.id())).map_err(Error::from)
    }
    fn oid_from_snapshot_id(snapshot: SnapshotId) -> Result<Oid, Error> {
        Oid::from_str(&String::from(snapshot)).map_err(Error::from)
    }
}

impl Repository for LibGit2 {
    fn snapshots_in_current_branch(&self) -> Result<Snapshots, Error> {
        let mut walker = self.repo.revwalk()?;
        walker.set_sorting(Sort::TIME & Sort::TOPOLOGICAL)?;
        walker.push_head()?;
        let mut snapshots = vec![];
        for reference in walker {
            let oid = reference?;
            let patents = self
                .repo
                .find_commit(oid)?
                .parents()
                .map(|commit| LibGit2::commit_to_snapshot_id(&commit))
                .collect();

            snapshots.push(Snapshot::new(oid.to_string().into(), patents));
        }

        Ok(snapshots.into())
    }

    fn compare_with_parent(&self, snapshot: &Snapshot) -> Result<ChangeDelta, Error> {
        let mut diffs: Vec<Vec<ChangedFilePath>> = vec![];
        let commit_id = LibGit2::oid_from_snapshot(snapshot)?;
        let snapshot_tree = self.repo.find_commit(commit_id)?.tree()?;

        for parent in snapshot.parents() {
            let parent_id = LibGit2::oid_from_snapshot_id(parent)?;
            let parent_tree = self.repo.find_commit(parent_id)?.tree()?;
            let diff_to_parent = self.repo.diff_tree_to_tree(
                Some(&parent_tree),
                Some(&snapshot_tree),
                Some(&mut DiffOptions::new()),
            )?;
            let deltas = diff_to_parent.deltas();
            diffs.push(
                deltas
                    .filter_map(|delta| LibGit2::path_of_changed_file(&delta))
                    .collect(),
            );
        }

        Ok(diffs.into_iter().flatten().collect::<Vec<_>>().into())
    }
}
