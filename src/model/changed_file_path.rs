use git2::DiffDelta;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub(crate) struct ChangedFilePath(Option<String>, String);

impl ChangedFilePath {
    pub(crate) fn add_prefix(&self, prefix: &str) -> ChangedFilePath {
        ChangedFilePath(Some(prefix.into()), self.1.clone())
    }
}

impl From<&str> for ChangedFilePath {
    fn from(path: &str) -> Self {
        ChangedFilePath(None, String::from(path))
    }
}

impl From<String> for ChangedFilePath {
    fn from(item: String) -> Self {
        ChangedFilePath(None, item)
    }
}

impl From<ChangedFilePath> for String {
    fn from(path: ChangedFilePath) -> Self {
        match path {
            ChangedFilePath(None, file) => file,
            ChangedFilePath(Some(repo), file) => format!("{}@{}", repo, file),
        }
    }
}

impl From<DiffDelta<'_>> for ChangedFilePath {
    fn from(delta: DiffDelta) -> Self {
        delta
            .new_file()
            .path()
            .and_then(std::path::Path::to_str)
            .map_or_else(|| ChangedFilePath::from("?"), ChangedFilePath::from)
    }
}
