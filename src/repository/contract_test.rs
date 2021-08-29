use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tempfile::tempdir;

use crate::repository::in_memory::InMemory;
use crate::repository::interface::{ChangeDelta, Repository, Snapshot, Snapshots};
use crate::repository::libgit2::LibGit2;

fn in_memory_repository() -> InMemory {
    InMemory::new(
        Snapshots::from(vec![
            Snapshot::new("3".into(), vec!["2".into()]),
            Snapshot::new("2".into(), vec!["1".into()]),
            Snapshot::new("1".into(), vec![]),
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
        let actual = repo.snapshots_in_current_branch().unwrap();
        let mut iter = actual.iter();
        let head_id = iter.next().unwrap();
        let mid_id = iter.next().unwrap();
        let root_id = iter.next().unwrap();

        assert!(iter.next().is_none());
        assert_eq!(
            Snapshots::from(vec![
                Snapshot::new(head_id.id(), vec![mid_id.id()]),
                Snapshot::new(mid_id.id(), vec![root_id.id()]),
                Snapshot::new(root_id.id(), vec![]),
            ]),
            actual
        );
    }
}

#[test]
fn given_a_snapshot_i_can_find_out_what_files_changed_in_it() {
    let tempdir = tempdir().unwrap();
    let path = tempdir.path();
    let repos: Vec<Box<dyn Repository>> = vec![
        Box::from(in_memory_repository()),
        Box::from(libgit2_repository(path.to_path_buf())),
    ];
    for repo in &repos {
        let actual = repo.snapshots_in_current_branch().unwrap();
        let mut iter = actual.iter();
        let head_id = iter.next().unwrap().id();
        let mid_id = iter.next().unwrap().id();
        iter.next().unwrap();

        assert!(iter.next().is_none());
        let expected: ChangeDelta = vec![String::from("file2"), String::from("file3")].into();
        assert_eq!(
            expected,
            repo.compare_with_parent(&Snapshot::new(head_id, vec![mid_id]))
                .unwrap()
        );
    }

    tempdir.close().unwrap();
}
