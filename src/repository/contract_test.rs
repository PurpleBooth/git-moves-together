use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};
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
        .arg(dir.to_str().unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn git_add_file(dir: &Path, file_name: &str) {
    let mut file = File::create(dir.join(file_name)).unwrap();
    let random_junk: String = thread_rng()
        .sample_iter(&Alphanumeric)
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
        Box::from(libgit2_repository(dir.into_path())),
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
                Commit::new(head.hash(), vec![mid.hash()], head.timestamp()),
                Commit::new(mid.hash(), vec![root.hash()], mid.timestamp()),
                Commit::new(root.hash(), vec![], root.timestamp()),
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
            head.hash(),
            head.timestamp(),
            vec!["file2".into(), "file3".into()],
        );
        assert_eq!(
            expected,
            repo.compare_with_parent(&Commit::new(
                head.hash(),
                vec![mid.hash()],
                head.timestamp(),
            ))
            .unwrap()
        );
    }

    tempdir.close().unwrap();
}
