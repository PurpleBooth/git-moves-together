use std::{
    fs::File,
    io::Write,
    os::unix::ffi::OsStringExt,
    path::{Path, PathBuf},
    process::Command,
};

use rand::prelude::*;
use tempfile::tempdir;

use crate::{
    model::{commit::Commit, commits::Commits, delta::Delta},
    repository::{in_memory::InMemory, interface::Repository, libgit2::LibGit2},
};

fn in_memory_repository() -> InMemory {
    InMemory::new(
        Commits::from(vec![
            Commit::new(
                "3".into(),
                vec!["2".into()],
                time::OffsetDateTime::now_utc(),
            ),
            Commit::new(
                "2".into(),
                vec!["1".into()],
                time::OffsetDateTime::now_utc(),
            ),
            Commit::new("1".into(), vec![], time::OffsetDateTime::now_utc()),
        ]),
        vec![
            ("1".into(), "file1".into()),
            ("2".into(), "file2".into()),
            ("3".into(), "file2".into()),
            ("3".into(), "file3".into()),
        ],
    )
}

fn libgit2_repository(dir: PathBuf) -> LibGit2 {
    git_init(&dir);
    git_add_file(&dir, "file1");
    git_commit(&dir);
    git_add_file(&dir, "file2");
    git_commit(&dir);
    git_add_file(&dir, "file2");
    git_add_file(&dir, "file3");
    git_commit(&dir);
    LibGit2::new(dir).unwrap()
}

fn git_init(dir: &Path) {
    Command::new("git")
        .arg("init")
        .arg(dir.to_string_lossy().as_ref())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn git_add_file(dir: &Path, file_name: &str) {
    let mut file = File::create(dir.join(file_name)).unwrap();
    let random_junk: String = rand::rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    file.write_all(random_junk.as_bytes()).unwrap();
    file.flush().unwrap();
    Command::new("git")
        .arg("add")
        .arg(dir.join(file_name))
        .current_dir(dir)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn git_commit(dir: &Path) {
    Command::new("git")
        .env("GIT_COMMITTER_NAME", "John Doe")
        .env("GIT_COMMITTER_EMAIL", "john@doe.org")
        .env("GIT_AUTHOR_NAME", "John Doe")
        .env("GIT_AUTHOR_EMAIL", "john@doe.org")
        .arg("commit")
        .arg("--no-verify")
        .arg("--no-gpg-sign")
        .arg("-m")
        .arg("Commit message")
        .current_dir(dir)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[test]
fn i_can_get_a_list_of_all_current_commits() {
    let dir = tempdir().unwrap();
    let repos: Vec<Box<dyn Repository>> = vec![
        Box::from(libgit2_repository(dir.keep())),
        Box::from(in_memory_repository()),
    ];
    for repo in &repos {
        let actual = repo.commits_in_current_branch().unwrap();
        let mut iter = actual.iter();
        let head = iter.next().unwrap();
        let mid = iter.next().unwrap();
        let root = iter.next().unwrap();

        assert!(iter.next().is_none());
        assert_eq!(
            Commits::from(vec![
                Commit::new(
                    head.hash().clone(),
                    vec![mid.hash().clone()],
                    head.timestamp()
                ),
                Commit::new(
                    mid.hash().clone(),
                    vec![root.hash().clone()],
                    mid.timestamp()
                ),
                Commit::new(root.hash().clone(), vec![], root.timestamp()),
            ]),
            actual
        );
    }
}

#[test]
fn given_a_commit_i_can_find_out_what_files_changed_in_it() {
    let tempdir = tempdir().unwrap();
    let path = tempdir.path();
    let repos: Vec<Box<dyn Repository>> = vec![
        Box::from(in_memory_repository()),
        Box::from(libgit2_repository(path.to_path_buf())),
    ];
    for repo in &repos {
        let actual = repo.commits_in_current_branch().unwrap();
        let mut iter = actual.iter();
        let head = iter.next().unwrap();
        let mid = iter.next().unwrap();
        iter.next().unwrap();

        assert!(iter.next().is_none());
        let expected: Delta = Delta::new(
            head.hash().clone(),
            head.timestamp(),
            vec!["file2".into(), "file3".into()],
        );
        assert_eq!(
            expected,
            repo.compare_with_parent(&Commit::new(
                head.hash().clone(),
                vec![mid.hash().clone()],
                head.timestamp(),
            ))
            .unwrap()
        );
    }

    tempdir.close().unwrap();
}

#[test]
fn root_commit_includes_its_added_files() {
    let tempdir = tempdir().unwrap();
    let path = tempdir.path();
    let repos: Vec<Box<dyn Repository>> = vec![
        Box::from(in_memory_repository()),
        Box::from(libgit2_repository(path.to_path_buf())),
    ];
    for repo in &repos {
        let actual = repo.commits_in_current_branch().unwrap();
        let mut iter = actual.iter();
        iter.next().unwrap();
        iter.next().unwrap();
        let root = iter.next().unwrap();

        assert!(iter.next().is_none());

        let delta = repo.compare_with_parent(root).unwrap();
        let changed: Vec<String> = delta.into_iter().map(String::from).collect();
        assert!(
            changed.contains(&"file1".to_string()),
            "Root commit should include file1, got {changed:?}"
        );
    }

    tempdir.close().unwrap();
}

#[test]
fn git_init_handles_non_utf8_paths() {
    // Create a directory with a non-UTF-8 character
    let tempdir = tempdir().unwrap();
    let non_utf8_name = std::ffi::OsString::from_vec(vec![0xC0, 0x80]); // Invalid UTF-8
    let non_utf8_path = tempdir.path().join(&non_utf8_name);

    // Create the directory with the non-UTF-8 name
    std::fs::create_dir(&non_utf8_path).unwrap();

    // This should not panic even with non-UTF-8 paths
    git_init(&non_utf8_path);
}
