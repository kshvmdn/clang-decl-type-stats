## vardecl

> Determine if a C/C++ codebase prefers commas or semicolons for variable declarations.

vardecl statically analyzes your C/C++ source files to determine the preferred method of declaring new variables.

### Demo

[![asciicast](https://asciinema.org/a/ZWkn83g3BcyWhTWuUOVTiZGCP.png)](https://asciinema.org/a/ZWkn83g3BcyWhTWuUOVTiZGCP)

### Install

#### Prerequisites

To build vardecl, you'll need [Rust](https://www.rust-lang.org/en-US/install.html) and [Clang](https://releases.llvm.org/download.html) (for [`libclang`](https://clang.llvm.org/doxygen/group__CINDEX.html)).

##### via Cargo

```sh
$ cargo install --git https://github.com/kshvmdn/vardecl.git
```

##### Build manually

```sh
$ git clone https://github.com/kshvmdn/vardecl.git
$ cd vardecl
$ cargo build --release
$ mv ./target/release/vardecl ~/.cargo/bin
```

### Usage

vardecl expects one or more files as input.

```sh
$ vardecl /path/to/file ...
```

#### Examples

```sh
$ cat main.c
void f() {
  int a, b = 2;
  int c;
  int d;
  int e; int f = 1;
  char *g,
        h,
        i = 'a';
}
$ vardecl main.c
comma-delimited decls     : 3
semicolon-delimited decls : 3
```

The example should be self-explanatory: there are three pairs of comma-delimited declarations (`int a, b;`, `char *g, h;`, `char h, i;`) and three pairs of semicolon-delimited declarations (`int c; int d;`, `int d; int e;`, `int e; int f;`).

### Contribute

This project is completely open source, feel free to [open an issue](https://github.com/kshvmdn/vardecl/issues) or [submit a pull request](https://github.com/kshvmdn/vardecl/pulls).

_I'm also quite new to Rust, so I'd love any feedback._
