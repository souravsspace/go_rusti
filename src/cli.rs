use clap::{Command, arg};

use crate::{blockchain::Blockchain, errors::Result};

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::create_blockchain()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("Blockchain rusty!")
            .version("0.0.1")
            .author("Sourav Ukil")
            .subcommand(Command::new("printchain").about("Print all the chain blocks."))
            .subcommand(
                Command::new("addblock")
                    .about("Add a block in the blockchain.")
                    .arg(arg!(<DATA>"'The blockchain data'")),
            )
            .get_matches();

        if matches.subcommand_matches("printchain").is_some() {
            self.print_chain();
        } else if let Some(add_matches) = matches.subcommand_matches("addblock") {
            if let Some(data) = add_matches.get_one::<String>("DATA") {
                self.addblock(data.to_string())?;
            }
        }

        Ok(())
    }

    fn addblock(&mut self, data: String) -> Result<()> {
        self.bc.add_block(data)
    }

    fn print_chain(&mut self) {
        for b in &mut self.bc.iter() {
            println!("Block: {:#?}", b);
        }
    }
}
