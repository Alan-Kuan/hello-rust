# guessing_game
## Knowledge Points
### Range
`..=` is a range operator expressing a range including begin and end.
For example, `beg..=end` expresses $[beg, end]$.
There is also a range operator expressing a range including begin only, i.e., `..`.
For example, `beg..end` expresses $[beg, end)$.

### Loop
`loop {}` creates a infinite loop.

### Flush
`print!()` outputs the given string to `stdout` without a newline character.
However, since `stdout` is line buffered, a flush is required to show the string as early as possible.
A flush can be done by using `std::io::Write` and calling `io::stdout().flush()`.

### Mutability
By default, a variable is not mutable.
To make it mutable, we have to add `mut` during a variable's declaration.

### Variable Shadowing
We can reuse the same variable name by *shadowing* it, i.e.,
declaring a new variable with the same name as a previous variable.

### String-to-Integer Conversion
To convert a string into an integer, simply call its `parse()` and assign it to a new variable declared with an integer type.

### Error Handling
A function call that may fail usually returns a value of the type `Result`.
To handle the error, there are three methods.

1. We can call `unwrap()` of the `Result`.
    By doing so, we simply take the `Ok` case's value.
    If the function call fails, the program panics, i.e., terminates abnormally.
    We call this function when we are confident that the call always successes.
2. We can call `expect()` of the `Result` with a message as its argument so that it prints the message and terminates the program when the call fails.
3. We can also handle the `Result` with `match` expression to prevent the program from exiting.
There are two cases in the expression, `Ok` and `Err`.
If the function call does not fail, the return value is passed as the argument of `Ok`;
on the other hand, if it fails, the error reason is passed as the argument of `Err`.
