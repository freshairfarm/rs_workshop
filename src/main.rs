use clap::{Args, Parser, Subcommand};

type Input = i8;
type Output = Input;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add(Arguments),
    Sub(Arguments),
}

#[derive(Debug, Args)]
struct Arguments {
    lhs: Input,
    rhs: Input,
}

fn main() {
    let cli = Cli::parse();

    let args;
    let func: fn(Input, Input) -> Output;
    match cli.command {
        Commands::Add(a) => {
            args = a;
            func = add;
        }
        Commands::Sub(a) => {
            args = a;
            func = sub;
        }
    }

    let answer = func(args.lhs, args.rhs);
    println!("Answer: {answer}");
}

fn add<T,R>(x: T, y: T) -> R
    where T: std::ops::Add<Output = R> {
    x + y
}

fn sub<T,R>(x: T, y: T) -> R
    where T: std::ops::Sub<Output = R> {
    x - y
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_adds() {
        const EXPECTED: Output = 0;

        let x: Input= 25;
        let y: Input = 25;
        
        let actual = add(x, y);

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_subs() {
        const EXPECTED: Output = -50;

        let x: Input = -25;
        let y: Input = 25;

        let actual = sub(x, y);

        assert_eq!(EXPECTED, actual);
    }
}