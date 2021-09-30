use chrono::{DateTime, TimeZone, Utc};
use git2::Commit as Git2Commit;

use crate::model::hash::Hash;

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) struct Commit {
    hash: Hash,
    timestamp: DateTime<Utc>,
    parents: Vec<Hash>,
}

impl Commit {
    pub(crate) fn new(hash: Hash, parents: Vec<Hash>, timestamp: DateTime<Utc>) -> Commit {
        Commit {
            hash,
            timestamp,
            parents,
        }
    }

    pub(crate) fn hash(&self) -> Hash {
        self.hash.clone()
    }

    pub(crate) fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub(crate) fn parents(&self) -> Vec<Hash> {
        self.parents.clone()
    }
}

impl From<Git2Commit<'_>> for Commit {
    fn from(commit: Git2Commit) -> Self {
        Commit::new(
            commit.id().into(),
            commit.parents().map(|parent| parent.id().into()).collect(),
            Utc.timestamp(commit.time().seconds(), 0),
        )
    }
}
