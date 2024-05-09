use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add(Arguments),
    Sub(Arguments),
}

#[derive(Debug, Args)]
struct Arguments {
    lhs: i32,
    rhs: i32,
}

fn main() {
    let cli = Cli::parse();

    let answer;
    match cli.command {
        Commands::Add(Arguments { lhs, rhs }) => {
            answer = lhs + rhs;
        }
        Commands::Sub(Arguments { lhs, rhs }) => {
            answer = lhs - rhs;
        }
    }
    println!("Answer: {answer}");
}
