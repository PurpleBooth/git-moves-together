use std::convert::TryFrom;

use chrono::{DateTime, Utc};
use git2::Oid;

impl From<Hash> for String {
    fn from(hash: Hash) -> Self {
        hash.hash
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Hash {
    hash: String,
}

impl From<&str> for Hash {
    fn from(hash: &str) -> Self {
        Hash {
            hash: String::from(hash),
        }
    }
}

impl From<String> for Hash {
    fn from(hash: String) -> Self {
        Hash { hash }
    }
}

impl From<Oid> for Hash {
    fn from(oid: Oid) -> Self {
        Hash {
            hash: oid.to_string(),
        }
    }
}

impl From<DateTime<Utc>> for Hash {
    fn from(datetime: DateTime<Utc>) -> Self {
        Hash {
            hash: datetime.to_string(),
        }
    }
}

impl TryFrom<Hash> for Oid {
    type Error = git2::Error;

    fn try_from(hash: Hash) -> Result<Self, Self::Error> {
        Oid::from_str(&String::from(hash))
    }
}

impl TryFrom<&Hash> for Oid {
    type Error = git2::Error;

    fn try_from(hash: &Hash) -> Result<Self, Self::Error> {
        Oid::from_str(hash.hash.as_str())
    }
}
