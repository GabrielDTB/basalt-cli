use crate::magic::DEFAULT_OUTPUT_PATH;
use clap::{Args, Parser, Subcommand};

/// Basalt is a set of utilities for knowledge management in the terminal.
#[derive(Parser, Debug)]
#[command(version)]
pub struct App {
    /// The command you want to run.
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize a vault in the current directory.
    ///
    /// Sets up the necessary structure to track
    /// config, build documents, store state, etc.
    Init,

    /// Create a new note.
    New(NewArgs),

    /// Compile a vault.
    Compile(CompileArgs),

}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// Name for the new note.
    #[clap(short, long)]
    pub name: Option<String>,
    /// Program to automatically open note with.
    #[clap(short, long)]
    pub open: Option<String>,
}

#[derive(Args, Debug)]
pub struct CompileArgs {
    // /// Path of vault to compile.
    // #[clap(long, default_value = ".")]
    // pub path: String,
    /// Path to write result.
    #[clap(long, default_value = DEFAULT_OUTPUT_PATH)]
    pub output: String,
    /// Program to open result with.
    #[clap(long)]
    pub open: Option<String>,
}
