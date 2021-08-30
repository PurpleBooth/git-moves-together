use partial_application::partial;
use std::ffi::OsStr;
use std::iter::FromIterator;
use std::path::PathBuf;

use crate::model::changed_file_path::ChangedFilePath;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ChangeDelta(Vec<ChangedFilePath>);

impl ChangeDelta {
    pub(crate) fn add_prefix(self, prefix: &str) -> ChangeDelta {
        ChangeDelta(
            self.0
                .iter()
                .map(partial!(ChangedFilePath::add_prefix => _, prefix))
                .collect(),
        )
    }

    pub(crate) fn add_prefix_from_filename(&self, path: &str) -> ChangeDelta {
        self.clone().add_prefix(
            PathBuf::from(path)
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap_or(path),
        )
    }
}

impl IntoIterator for &ChangeDelta {
    type Item = ChangedFilePath;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

impl From<Vec<String>> for ChangeDelta {
    fn from(paths: Vec<String>) -> Self {
        ChangeDelta(paths.into_iter().map(ChangedFilePath::from).collect())
    }
}

impl From<Vec<&str>> for ChangeDelta {
    fn from(paths: Vec<&str>) -> Self {
        ChangeDelta(paths.into_iter().map(ChangedFilePath::from).collect())
    }
}

impl From<Vec<ChangedFilePath>> for ChangeDelta {
    fn from(paths: Vec<ChangedFilePath>) -> Self {
        ChangeDelta(paths.into_iter().collect())
    }
}

impl ChangeDelta {
    pub(crate) fn contains(&self, item: &ChangedFilePath) -> bool {
        self.0.contains(item)
    }
}

impl FromIterator<ChangedFilePath> for ChangeDelta {
    fn from_iter<T: IntoIterator<Item = ChangedFilePath>>(iter: T) -> Self {
        let mut items = Vec::new();
        for item in iter {
            items.push(item);
        }

        ChangeDelta(items)
    }
}
