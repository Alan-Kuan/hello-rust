# hello_world
## Features
It prints `hello, world!`.

## Knowledge Points
### rustup
`rustup` is the Rust toolchain installer.
It can be downloaded at https://rustup.rs/.
After `rustup` is installed, `rustc` and `cargo` is available.

### rustc
`rustc` is the compiler for Rust.
We can compile a Rust code by simply running `rustc main.rs`.

### cargo
`cargo` is the package manager for Rust.
It is helpful to create and manage a Rust project.

Some common usages:
- `cargo new <path>`: create a new Rust package under the given path
    - It uses the deepest directory's name as the package's name by default.
      However, a package's name cannot begin with a number.
      Therefore, if a directory's name begins with a number,
      one can specify the package's name with the option `--name`.
- `cargo init`: make current directory into a Rust package
- `cargo build`: build the package under current directory
- `cargo run`: build and run the package under current directory
- `cargo add <dep>`: add a dependency to the package

### macro
A function name ends with a `!` is actually a *macro*, e.g., `println!`.
