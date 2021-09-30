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

pub(crate) struct InMemory {
    commits: Commits,
    changes: Vec<(Hash, ChangedFile)>,
}

impl InMemory {
    // Note, this is not actually dead, but rather proof that we can swap out our
    // git provider
    #[allow(dead_code)]
    pub(crate) fn new(commits: Commits, changes: Vec<(Hash, ChangedFile)>) -> InMemory {
        InMemory { commits, changes }
    }
}

impl Repository for InMemory {
    fn commits_in_current_branch(&self) -> Result<Commits, Error> {
        Ok(self.commits.clone())
    }

    fn compare_with_parent(&self, commit: &Commit) -> Result<Delta, Error> {
        Ok(Delta::new(
            commit.hash(),
            commit.timestamp(),
            self.changes
                .clone()
                .iter()
                .filter_map(|(hash, change)| {
                    if &commit.hash() == hash {
                        Some(change.clone())
                    } else {
                        None
                    }
                })
                .collect(),
        ))
    }
}
