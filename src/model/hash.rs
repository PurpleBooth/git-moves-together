use std::convert::TryFrom;

use git2::Oid;
use time::OffsetDateTime;

impl From<Hash> for String {
    fn from(hash: Hash) -> Self {
        hash.hash
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hash {
    hash: String,
}

impl From<&str> for Hash {
    fn from(hash: &str) -> Self {
        Self {
            hash: String::from(hash),
        }
    }
}

impl From<String> for Hash {
    fn from(hash: String) -> Self {
        Self { hash }
    }
}

impl From<Oid> for Hash {
    fn from(oid: Oid) -> Self {
        Self {
            hash: oid.to_string(),
        }
    }
}

impl From<OffsetDateTime> for Hash {
    fn from(datetime: OffsetDateTime) -> Self {
        Self {
            hash: datetime.to_string(),
        }
    }
}

impl TryFrom<Hash> for Oid {
    type Error = git2::Error;

    fn try_from(hash: Hash) -> Result<Self, Self::Error> {
        Self::from_str(&String::from(hash))
    }
}

impl TryFrom<&Hash> for Oid {
    type Error = git2::Error;

    fn try_from(hash: &Hash) -> Result<Self, Self::Error> {
        Self::from_str(hash.hash.as_str())
    }
}
