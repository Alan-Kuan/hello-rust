# shell
## Features
- executes commands
- supports single quotes and double quotes
- supports multiple IO redirections via `>` and `<`
- supports pipelines via `|`

### Built-in Commands
- `cd [DIR]`
- `echo [STR]...`
- `exit`
- `pwd`

## Knowledge Points
### Crate
A *crate* is the unit the compiler considers at a time.
Crates contain *modules*, and the modules may be defined in other files that get compiled with the crate.

The crate comes in two forms:
1. binary crate
    - executable program
    - usually begins with the file `src/main.rs`
2. library crate
    - does not have a `main` function
    - defines functionality to be shared with multiple projects
    - usually begins with the file `src/<library name>.rs`

### Module
A *module* can be defined with the `mod` keyword.
For example, a module named `garden` and be defined with `mod garden;`.
The compiler will look for its code in the following places:
1. Inline, within curly brackets that replace the semicolon following `mod garden`
2. In the file `src/garden.rs`
3. In the file `src/garden/mod.rs`

Submodules can be defined by adding `mod` statement in its parent module's code.
For example, add `mod vegetables;` in `src/garden.rs`.
The compiler will look for its code in the following places:
1. Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
2. In the file `src/garden/vegetables.rs`
3. In the file `src/garden/vegetables/mod.rs`

#### Paths to Code
A type `Asparagus` defined in garden vegetables module can be found at `crate::garden::vegetables::Asparagus`.

#### Visibility
Code within a module is private from its parent modules by default.
To make it visible to its parent, add `pub` before the `mod` statement, e.g., `pub mod garden;`.
To make items within a public module visible too, add `pub` before their declarations too.

#### `use` Keyword
Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths.
After writing `use crate::garden::vegetables::Asparagus;`, we can use `Asparagus` directly from then on.

### Package
A *package* is a bundle of one or more crates that provides a set of functionality.
A package contains a `Cargo.toml` file that describes how to build those crates.

### Documentation Comment
A documentation comment starts with `///` and supports Markdown notation for formatting the text.
It can be used to generate HTML documentation,
which can be built and opened by running `cargo doc --open`.

### Array vs. Slice vs. Vector
- An *array* is a fixed-sized ordered collection of continuous values of the same type.
    For example, `[i32; 5]` denotes an integer array with 5 elements.
- A *slice* is a reference to a dynamic-sized ordered collection of continuous values of the same type.
    For example, `&[i32]` denotes an integer slice.
- A *vector* is a dynamic-sized ordered collection of continuous values of the same type.
    For example, `Vec<i32>` denotes an integer vector.
    - It can be initialized with either `let v = vec![]` or `let v = Vec::new()`.

#### String Slice
`str` is string *slice*, which is a reference to part of a `String`.

### String Split
A string can be split by some character into a `Split` by calling its `split()`.
The `Split` is an iterator, which lazily returns each segment via `next()`.
