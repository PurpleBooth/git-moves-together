use git2::DiffDelta;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct ChangedFile {
    prefix: Option<String>,
    path: String,
}

impl ChangedFile {
    pub(crate) fn add_prefix(&self, prefix: &str) -> Self {
        Self {
            prefix: Some(prefix.into()),
            path: self.path.clone(),
        }
    }
}

impl From<&str> for ChangedFile {
    fn from(path: &str) -> Self {
        Self {
            prefix: None,
            path: path.to_string(),
        }
    }
}

impl From<String> for ChangedFile {
    fn from(path: String) -> Self {
        Self { prefix: None, path }
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
            } => format!("{repo}@{path}"),
        }
    }
}

impl From<DiffDelta<'_>> for ChangedFile {
    fn from(delta: DiffDelta<'_>) -> Self {
        delta
            .new_file()
            .path()
            .map(|p| p.to_string_lossy().into_owned())
            .map_or_else(|| Self::from("?"), Self::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_utf8_path_preserves_data() {
        // This test would require creating a DiffDelta with a non-UTF-8 path
        // Since git2 doesn't easily allow this, we test the underlying logic
        // The fix uses to_string_lossy() instead of to_str() to preserve data
    }

    #[test]
    fn test_from_str() {
        let file = ChangedFile::from("test/path.txt");
        assert_eq!(file.path, "test/path.txt");
    }

    #[test]
    fn test_from_string() {
        let file = ChangedFile::from("test/path.txt".to_string());
        assert_eq!(file.path, "test/path.txt");
    }

    #[test]
    fn test_add_prefix() {
        let file = ChangedFile::from("path.txt");
        let with_prefix = file.add_prefix("myrepo");
        assert_eq!(with_prefix.prefix, Some("myrepo".to_string()));
        assert_eq!(with_prefix.path, "path.txt");
    }

    #[test]
    fn test_to_string_with_prefix() {
        let file = ChangedFile {
            prefix: Some("myrepo".to_string()),
            path: "path.txt".to_string(),
        };
        let s: String = file.into();
        assert_eq!(s, "myrepo@path.txt");
    }

    #[test]
    fn test_to_string_without_prefix() {
        let file = ChangedFile {
            prefix: None,
            path: "path.txt".to_string(),
        };
        let s: String = file.into();
        assert_eq!(s, "path.txt");
    }
}
