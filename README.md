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
+------------------+------------------+------------+----------+---------+
| File A           | File B           | Together % | Together | Commits |
+=======================================================================+
| some-repo@file_1 | some-repo@file_2 | 50.00%     | 1        | 2       |
|------------------+------------------+------------+----------+---------|
| some-repo@file_1 | some-repo@file_3 | 100.00%    | 1        | 1       |
|------------------+------------------+------------+----------+---------|
| some-repo@file_2 | some-repo@file_3 | 50.00%     | 1        | 2       |
+------------------+------------------+------------+----------+---------+
```

You can also reduce the commits you're including, by limiting the
changes to a specific time period

``` shell,script(name="day-limit-setup",expected_exit_code=0)
echo "day-limit-setup - file_1" > file_1
git add .
GIT_COMMITTER_DATE="2005-04-07T22:13:13" git commit --message "demo: day-limit-setup"
```

``` shell,script(name="day-limit",expected_exit_code=0)
git-moves-together -d 30 $PWD
```

``` text,verify(script_name="day-limit",stream=stdout)
+------------------+------------------+------------+----------+---------+
| File A           | File B           | Together % | Together | Commits |
+=======================================================================+
| some-repo@file_1 | some-repo@file_2 | 50.00%     | 1        | 2       |
|------------------+------------------+------------+----------+---------|
| some-repo@file_1 | some-repo@file_3 | 100.00%    | 1        | 1       |
|------------------+------------------+------------+----------+---------|
| some-repo@file_2 | some-repo@file_3 | 50.00%     | 1        | 2       |
+------------------+------------------+------------+----------+---------+
```

You can also set a window of time to group by rather than the commit id,
which is useful when you're looking for coupling over multiple
repositories

Let's make another git repository

``` shell,script(name="time-windo-setup",expected_exit_code=0)
echo "time-window-setup - file_1" > "../other-repo/file_1"
echo "time-window-setup - file_2" > "../other-repo/file_2"
echo "time-window-setup - file_3" > "../other-repo/file_3"
git -C "../other-repo" add .
git -C "../other-repo" commit --message "demo: time-window-setup"
echo "time-window-setup - file_1 update" > "../other-repo/file_1"
echo "time-window-setup - file_2 update" > "../other-repo/file_2"
echo "time-window-setup - file_3 update" > "../other-repo/file_3"
git -C "../other-repo" add .
git -C "../other-repo" commit --message "demo: time-window-setup"
```

Now we can look at the coupling across two repositories

``` shell,script(name="time-window",expected_exit_code=0)
git-moves-together -t 30 "$PWD" "$PWD/../other-repo"
```

``` text,verify(script_name="time-window",stream=stdout)
+-------------------+-------------------+------------+----------+---------+
| File A            | File B            | Together % | Together | Commits |
+=========================================================================+
| other-repo@file_1 | other-repo@file_2 | 100.00%    | 2        | 2       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_1 | other-repo@file_3 | 100.00%    | 2        | 2       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_1 | some-repo@file_1  | 33.33%     | 2        | 6       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_1 | some-repo@file_2  | 40.00%     | 2        | 5       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_1 | some-repo@file_3  | 40.00%     | 2        | 5       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_2 | other-repo@file_3 | 100.00%    | 2        | 2       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_2 | some-repo@file_1  | 33.33%     | 2        | 6       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_2 | some-repo@file_2  | 40.00%     | 2        | 5       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_2 | some-repo@file_3  | 40.00%     | 2        | 5       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_3 | some-repo@file_1  | 33.33%     | 2        | 6       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_3 | some-repo@file_2  | 40.00%     | 2        | 5       |
|-------------------+-------------------+------------+----------+---------|
| other-repo@file_3 | some-repo@file_3  | 40.00%     | 2        | 5       |
|-------------------+-------------------+------------+----------+---------|
| some-repo@file_1  | some-repo@file_2  | 83.33%     | 5        | 6       |
|-------------------+-------------------+------------+----------+---------|
| some-repo@file_1  | some-repo@file_3  | 83.33%     | 5        | 6       |
|-------------------+-------------------+------------+----------+---------|
| some-repo@file_2  | some-repo@file_3  | 100.00%    | 5        | 5       |
+-------------------+-------------------+------------+----------+---------+
```

Which is why you see the coupling as shown above

## Usage

``` shell,script(name="help",expected_exit_code=0)
git-moves-together -h
```

``` text,verify(script_name="help",stream=stdout)
git-moves-together 2.3.2

Billie Thompson <billie@billiecodes.com>

Find files that move at the same time in a git repository to identify coupling

USAGE:
    git-moves-together [OPTIONS] [git-repo]...

ARGS:
    <git-repo>...    A repository to analyse [env: GIT_REPO=] [default: .]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -d, --from-days <max-days-ago>
            Ignore deltas older than the given days [env: MAX_DAYS_AGO=]

    -t, --time-window-minutes <time-window-minutes>
            Group commits by similar time window rather than by commit id [env:
            TIME_WINDOW_MINUTES=]
```

## Installing

See the [releases
page](https://github.com/PurpleBooth/ellipsis/releases/latest) we build
for linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/git-moves-together
```
