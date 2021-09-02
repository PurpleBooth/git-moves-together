use git2::DiffDelta;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub(crate) struct ChangedFile {
    prefix: Option<String>,
    path: String,
}

impl ChangedFile {
    pub(crate) fn add_prefix(&self, prefix: &str) -> ChangedFile {
        ChangedFile {
            prefix: Some(prefix.into()),
            path: self.path.clone(),
        }
    }
}

impl From<&str> for ChangedFile {
    fn from(path: &str) -> Self {
        ChangedFile {
            prefix: None,
            path: path.to_string(),
        }
    }
}

impl From<String> for ChangedFile {
    fn from(path: String) -> Self {
        ChangedFile { prefix: None, path }
    }
}

impl From<ChangedFile> for String {
    fn from(change: ChangedFile) -> Self {
        match change {
            ChangedFile {
                prefix: None, path, ..
            } => path,
            ChangedFile {
                prefix: Some(repo),
                path,
                ..
            } => format!("{}@{}", repo, path),
        }
    }
}

impl From<DiffDelta<'_>> for ChangedFile {
    fn from(delta: DiffDelta) -> Self {
        delta
            .new_file()
            .path()
            .and_then(std::path::Path::to_str)
            .map_or_else(|| ChangedFile::from("?"), ChangedFile::from)
    }
}
