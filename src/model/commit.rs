use git2::Commit as Git2Commit;
use time::OffsetDateTime;

use crate::model::hash::Hash;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Commit {
    hash: Hash,
    timestamp: OffsetDateTime,
    parents: Vec<Hash>,
}

impl Commit {
    pub(crate) fn new(hash: Hash, parents: Vec<Hash>, timestamp: OffsetDateTime) -> Self {
        Self {
            hash,
            timestamp,
            parents,
        }
    }

    pub(crate) fn hash(&self) -> Hash {
        self.hash.clone()
    }

    pub(crate) const fn timestamp(&self) -> OffsetDateTime {
        self.timestamp
    }

    pub(crate) fn parents(&self) -> Vec<Hash> {
        self.parents.clone()
    }
}

impl From<Git2Commit<'_>> for Commit {
    fn from(commit: Git2Commit<'_>) -> Self {
        Self::new(
            commit.id().into(),
            commit.parents().map(|parent| parent.id().into()).collect(),
            OffsetDateTime::from_unix_timestamp(commit.time().seconds())
                .expect("Timestamp would overflow integer"),
        )
    }
}
