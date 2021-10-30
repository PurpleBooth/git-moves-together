use crate::model::commit::Commit;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Commits {
    commits: Vec<Commit>,
}

impl From<Vec<Commit>> for Commits {
    fn from(commits: Vec<Commit>) -> Self {
        Self { commits }
    }
}

impl IntoIterator for Commits {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Commit;

    fn into_iter(self) -> Self::IntoIter {
        self.commits.into_iter()
    }
}

impl Commits {
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, Commit> {
        self.commits.iter()
    }
}
