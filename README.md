# git moves-together

This tells you when files in the repository frequently move together.
This lets you identify where the coupling is in the system. Coupling is
negative and to an extent unavoidable, this tool aims to make it
visible.

## Getting Started

If every time I commit no file moves at the same time, that's 0 coupling

``` shell,script(name="no-coupling-setup",expected_exit_code=0)
echo "no-coupling-setup - file_1" > file_1
git add .
git commit --message "demo: no-coupling-setup"
echo "no-coupling-setup - file_2" > file_2
git add .
git commit --message "demo: no-coupling-setup"
```

When we run git-moves-together we can see that these files have no
direct commit based coupling

``` shell,script(name="no-coupling",expected_exit_code=0)
git-moves-together
```

``` text,verify(script_name="no-coupling",stream=stdout)
0 files move together
```

If we then make a change to both files in the same commit

``` shell,script(name="coupling-setup",expected_exit_code=0)
rm file_1 file_2
echo "coupling-setup - file_1" > file_1
echo "coupling-setup - file_2" > file_2
echo "coupling-setup - file_3" > file_3
git add .
git commit --message "demo: coupling-setup"
```

When we run git-moves-together we can see that these files have no
direct commit based coupling

``` shell,script(name="coupling",expected_exit_code=0)
git-moves-together $PWD
```

``` text,verify(script_name="coupling",stream=stdout)
+--------+--------+----------------+
| File A | File B | Moves Together |
+==================================+
| file_2 | file_3 | 100.00%        |
|--------+--------+----------------|
| file_1 | file_3 | 100.00%        |
|--------+--------+----------------|
| file_1 | file_2 | 50.00%         |
+--------+--------+----------------+


3 files move together
```

## Usage

``` shell,script(name="help",expected_exit_code=0)
git-moves-together -h
```

``` text,verify(script_name="help",stream=stdout)
git-moves-together 0.1.3

Billie Thompson <billie@billiecodes.com>

Find files that move at the same time in a git repository to identify coupling

USAGE:
    git-moves-together [git-repo]

ARGS:
    <git-repo>    A repository to analyse [env: GIT_REPO=] [default: .]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Installing

See the [releases
page](https://github.com/PurpleBooth/ellipsis/releases/latest) we build
for linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/git-moves-together
```
