use ::minigrep::{Cli, Parser};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    args.run()?;

    Ok(())
}
