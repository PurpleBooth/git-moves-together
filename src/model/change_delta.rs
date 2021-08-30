use std::ffi::OsStr;
use std::iter::FromIterator;
use std::path::PathBuf;

use partial_application::partial;

use crate::model::changed_file_path::ChangedFilePath;
use std::collections::BTreeSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ChangeDelta(BTreeSet<ChangedFilePath>);

impl ChangeDelta {
    pub(crate) fn contains(&self, item: &ChangedFilePath) -> bool {
        self.0.contains(item)
    }

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
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

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

impl FromIterator<ChangedFilePath> for ChangeDelta {
    fn from_iter<T: IntoIterator<Item = ChangedFilePath>>(iter: T) -> Self {
        let mut items = BTreeSet::new();
        for item in iter {
            items.insert(item);
        }

        ChangeDelta(items)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::change_delta::ChangeDelta;

    #[test]
    fn can_put_a_prefix_on_everything_in() {
        let actual: Vec<String> = ChangeDelta::from(vec!["item 1", "item 2", "item 3"])
            .add_prefix("Something")
            .into_iter()
            .map(|x| x.into())
            .collect();

        assert_eq!(
            actual,
            vec!["Something@item 1", "Something@item 2", "Something@item 3"]
        );
    }
    #[test]
    fn can_tell_if_something_is_in_this_delta() {
        let actual = ChangeDelta::from(vec!["item 1", "item 2", "item 3"]);

        assert!(actual.contains(&"item 1".into()));
        assert!(!actual.contains(&"none existing".into()));
    }
}
