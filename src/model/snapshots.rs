use crate::model::snapshot::Snapshot;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Snapshots(Vec<Snapshot>);

impl From<Vec<Snapshot>> for Snapshots {
    fn from(source: Vec<Snapshot>) -> Self {
        Snapshots(source.into_iter().collect())
    }
}

impl IntoIterator for Snapshots {
    type Item = Snapshot;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Snapshots {
    pub(crate) fn iter(&self) -> std::slice::Iter<Snapshot> {
        self.0.iter()
    }
}
