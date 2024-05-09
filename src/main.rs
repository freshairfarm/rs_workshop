use clap::{Args, Parser, Subcommand};
use std::io::Write;
use std::cell::RefCell;

type Input = f64;
type Output = Input;

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add(Arguments),
    Sub(Arguments),
    Acc,
    Exit,
}

#[derive(Debug, Args)]
struct Arguments {
    lhs: Input,
    rhs: Option<Input>,
}

fn main() {
    let acc: RefCell<Output> = RefCell::new(Default::default());

    loop {
        let args = read();
        let args = args.trim().split(' ');
        let parsed = Cli::try_parse_from(args);

        match parsed {
            Ok(parsed) => {
                let ops;
                let func: fn(Input, Input) -> Output;
                match parsed.command {
                    Commands::Add(a) => {
                        ops = a;
                        func = add;
                    }
                    Commands::Sub(a) => {
                        ops = a;
                        func = sub;
                    }
                    Commands::Acc => {
                        println!("Accumulator: {}", acc.borrow());
                        continue;
                    }
                    Commands::Exit => {
                        println!("Exiting...");
                        break;
                    }
                }
                
                let mut acc = acc.borrow_mut();
                
                *acc = match ops.rhs {
                    Some(val) => {
                        func(ops.lhs, val)
                    }
                    None => {
                        func(*acc, ops.lhs)
                    }
                };

                println!("Answer: {acc}");
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
}

fn add<T,R>(x: T, y: T) -> R
    where T: std::ops::Add<Output = R> {
    x + y
}

fn sub<T,R>(x: T, y: T) -> R
    where T: std::ops::Sub<Output = R> {
    x - y
}

fn read() -> String {
    let mut buffer = String::new();
    print!("$ ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut buffer)
        .unwrap();
    buffer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_adds() {
        const EXPECTED: Output = 0.;

        let x: Input= 25.;
        let y: Input = 25.;
        
        let actual = add(x, y);

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_subs() {
        const EXPECTED: Output = -50.;

        let x: Input = -25.;
        let y: Input = 25.;

        let actual = sub(x, y);

        assert_eq!(EXPECTED, actual);
    }
}