use crate::compile;
use crate::init::check_vault;
use crate::magic::COMPILE_SCRATCH_PATH;
use anyhow::{anyhow, bail, Context, Result};
use std::process::Command;

pub fn command(args: Vec<String>) -> Result<()> {
    check_vault().context("Not in a valid vault.")?;

    println!("{:#?}", args);

    compile::gather().context("Failed to gather all vault files.")?;

    let command_output = Command::new("typst")
        .args(["query", "--root", ".", COMPILE_SCRATCH_PATH])
        .args(args)
        .output()
        .context("Failed to run typst compile.")?;
    if !command_output.stderr.is_empty() {
        return Err(anyhow!(
            String::from_utf8_lossy(&command_output.stderr).to_string()
        ))
        .context(format!("Typst query failed for {COMPILE_SCRATCH_PATH:?}"));
    }
    println!(
        "{}",
        String::from_utf8(command_output.stdout)
            .context("Failed to create string to print from Typst query output.")?
    );

    Ok(())
}
