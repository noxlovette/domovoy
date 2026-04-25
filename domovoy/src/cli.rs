use std::io::{self, Write};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Auth,
    Reset,
}

pub fn prompt(msg: &str) -> Result<String, io::Error> {
    println!("{msg}");
    io::stdout().flush()?;

    let mut i = String::new();
    io::stdin().read_line(&mut i)?;

    Ok(i.trim().to_string())
}
