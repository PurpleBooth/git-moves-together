use std::{convert::TryInto, path::PathBuf};

use git2::{Oid, Repository as LibGit2Repository, Sort, Tree};

use crate::{
    model::{
        changed_file::ChangedFile,
        commit::Commit,
        commits::Commits,
        delta::Delta,
        hash::Hash,
    },
    repository::{errors::Error, interface::Repository},
};

pub(crate) struct LibGit2 {
    repo: LibGit2Repository,
}

impl LibGit2 {
    pub(crate) fn new(path: PathBuf) -> Result<LibGit2, Error> {
        let repo = git2::Repository::open(path)?;

        Ok(LibGit2 { repo })
    }

    fn diff_with_parent(&self, tree: &Tree, parent: &Hash) -> Result<Vec<ChangedFile>, Error> {
        let tree1 = parent
            .try_into()
            .and_then(|oid| self.repo.find_commit(oid))
            .and_then(|commit| commit.tree())?;
        Ok(self
            .repo
            .diff_tree_to_tree(Some(&tree1), tree.into(), None)?
            .deltas()
            .map(std::convert::Into::into)
            .collect())
    }

    fn to_commit(&self, commit_oid: Oid) -> Result<Commit, Error> {
        self.repo
            .find_commit(commit_oid)
            .map(Commit::from)
            .map_err(Error::from)
    }
}

impl Repository for LibGit2 {
    fn commits_in_current_branch(&self) -> Result<Commits, Error> {
        let mut walker = self.repo.revwalk()?;
        walker.set_sorting(Sort::TIME & Sort::TOPOLOGICAL)?;
        walker.push_head()?;

        walker
            .into_iter()
            .map(|oid| self.to_commit(oid?))
            .collect::<Result<Vec<Commit>, Error>>()
            .map(Commits::from)
    }

    fn compare_with_parent(&self, commit: &Commit) -> Result<Delta, Error> {
        let tree = commit
            .hash()
            .try_into()
            .and_then(|oid| self.repo.find_commit(oid))
            .and_then(|commit| commit.tree())?;

        let changes = commit
            .parents()
            .iter()
            .map(|parent| self.diff_with_parent(&tree, parent))
            .reduce(flatten_or_first_err)
            .unwrap_or_else(|| Ok(vec![]));

        Ok(Delta::new(commit.hash(), commit.timestamp(), changes?))
    }
}

fn flatten_or_first_err(
    acc: Result<Vec<ChangedFile>, Error>,
    item: Result<Vec<ChangedFile>, Error>,
) -> Result<Vec<ChangedFile>, Error> {
    match (acc, item) {
        (Ok(acc), Ok(item)) => Ok([acc, item].concat()),
        (Err(err), _) | (_, Err(err)) => Err(err),
    }
}
