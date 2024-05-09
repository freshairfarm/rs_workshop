use clap::{Args, Parser, Subcommand};

type Input = f32;
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

fn add(x: Input, y: Input) -> Output {
    x + y
}

fn sub(x: Input, y: Input) -> Output {
    x - y
}