use crate::model::commit::Commit;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Commits {
    commits: Vec<Commit>,
}

impl From<Vec<Commit>> for Commits {
    fn from(commits: Vec<Commit>) -> Self {
        Commits {
            commits: commits.into_iter().collect(),
        }
    }
}

impl IntoIterator for Commits {
    type Item = Commit;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.commits.into_iter()
    }
}

impl Commits {
    pub(crate) fn iter(&self) -> std::slice::Iter<Commit> {
        self.commits.iter()
    }
}
