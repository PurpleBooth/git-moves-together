# git moves-together

This tells you when files in the repository frequently move together.
This lets you identify where the coupling is in the system. Coupling is
negative and to an extent unavoidable, this tool aims to make it
visible.

## Getting Started

If every time I commit no file moves at the same time, that's 0 coupling

```shell,script(name="no-coupling-setup", expected_exit_code=0)
git init --template "$(mktemp -d)" .
touch file_1
git add file_1
git commit --message "demo"
touch file_2
git add file_2
git commit --message "demo"
```

When we run git-moves-together 
we can see that these files have no direct commit based coupling

```shell,script(name="no-coupling", expected_exit_code=0)
git-moves-together .
```


```text,verify(script_name="no-coupling", stream=stdout)
0 files move together
```

If we then make a change to both files in the same commit

```shell,script(name="coupling-setup", expected_exit_code=0)
echo "change 1" > file_1
cp file_1 file_2
cp file_1 file_3
git add file_1 file_2 file_3
git commit --message "demo"
```

When we run git-moves-together
we can see that these files have no direct commit based coupling

```shell,script(name="coupling", expected_exit_code=0)
git-moves-together .
```

```text,verify(script_name="coupling", stream=stdout)
file_1, file_2 move together 33%
```