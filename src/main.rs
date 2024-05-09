use clap::{Args, Parser, Subcommand};
use std::io::Write;

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
    Exit,
}

#[derive(Debug, Args)]
struct Arguments {
    lhs: Input,
    rhs: Input,
}

fn main() {
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
                    Commands::Exit => {
                        println!("Exiting...");
                        break;
                    }
                }
                
                let answer = func(ops.lhs, ops.rhs);
                println!("Answer: {answer}");
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