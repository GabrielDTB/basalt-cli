#![feature(let_chains)]

mod app;
mod compile;
mod init;
mod magic;
mod query;

use crate::app::{App, Command::*};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = App::parse();
    let command = args.command;

    match command {
        Init => init::command(),
        Compile(args) => compile::command(&args.output, args.open.as_deref()),
        Query(args) => query::command(args.args),
        _ => unimplemented!(),
    }
}
