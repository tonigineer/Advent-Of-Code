use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]

pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Solve a puzzles for a day.")]
    Solve {
        year: u32,
        day: u32,
    },
    #[command(about = "List results for a year.")]
    List { 
        year: u32
     }
}