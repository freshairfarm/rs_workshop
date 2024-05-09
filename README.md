Step by Step:

- [ ] Create a new project: `cargo new rs_workshop`
- [ ] Add a new Crate dependency: `cargo add clap --features derive`

## Commit 1: Start of Work
- [ ] Add Clap using the Derive API
	- [ ] Add the CLI - The parser for the arguments sent to the application and core component
	- [ ] Add a `Subcommand` enum, the different command, or program, values which our application can expect
	- [ ] Add an `Args` struct, a collection of arguments for each individual subcommand that our application can expect
- [ ] Add a command for addition and subtraction
- [ ] Show naive solution, show solution with less code-duplication

## Commit 2: Closures
- [ ] Break args out
- [ ] Create methods for add and sub
- [ ] Create a variable, `func`, which is a closure that is set to either add or sub

## Commit 3: Unit Tests
Now that closures have been added, functions are outside the scope of main
- [ ] Unit test the add and sub functions
- [ ] Explain the nuance of unit tests:
	- In Rust, unit tests are in the same source file as our implementation
	- Use `super::*`
	- They are in a separate module, decorated with \#\[cfg(test)]
		- This means they will not be compiled into our executable binary if we are not compiling with the test flag enabled
	- Tests inside the module are decorated with \#\[test]
		- This is used by the test runner to determine what code is meant to be tested.

## Commit 4: Generics
- [ ] Take existing functions, add generic parameters for T and R
	- T stands for Type, R stands for Return
- [ ] Add generic type constraints for `std::ops::Add<Output = R>` and `std::ops::Sub<Output = R>`

## Commit 5: REPL
Running our application repeatedly is hard, and makes using the calculator unintuitive.
- [ ] Add a function for reading from stdin
- [ ] Loop in the main method
- [ ] Add `#[command(multicall = true)]` to Cli
- [ ] Convert `Cli::parse` to `Cli::try_parse_from()`
- [ ] Add a command for exiting
- [ ] Add error handling (print the error)

## Commit 6: Accumulator
- [ ] Add an accumulator which is a `std::cell::RefCell`
- [ ] Instead of answer, use acc
- [ ] Mutate the interior value of acc between loops
- [ ] Make `y` value an `option`