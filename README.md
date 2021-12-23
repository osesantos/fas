# fas


fas stands for Find all stuff and it's a rust app that simplify the find command and allow you to easily search everything you need.

Note: currently being developed with go and rust for experience porpuses.

<br>

![Screenshot from 2021-12-19 01-38-59](https://user-images.githubusercontent.com/72045872/146660634-c23031d7-cf50-44fc-86ab-b3375db74e32.png)

<br>

```sh
fas 0.0.1
m4jrt0m
fas stands for find all stuff. With this app you can easily look for a string in your files names
and inside your files.

USAGE:
    fas_rust [OPTIONS]

OPTIONS:
    -f, --file <file>        file to look into
    -h, --help               Print help information
    -p, --path <path>        Path to look into
    -s, --search <search>    search term to look for
    -V, --version            Print version information

```

<br>

## How to run fas?

#### To build

```sh
$ cargo build --relase
```

#### To run

```sh
$ cargo run -- -p <path> -s <search term>
```
