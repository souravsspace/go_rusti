mod block;
mod blockchain;
mod cli;
mod errors;

use cli::Cli;
use errors::Result;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;

    cli.run()?;

    Ok(())
}
