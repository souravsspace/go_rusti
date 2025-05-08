use clap::{Command, arg};
use std::process::exit;

use crate::{blockchain::Blockchain, errors::Result, transactions::Transaction};

pub struct Cli {}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("Blockchain rusty!")
            .version("0.0.1")
            .author("Sourav Ukil")
            .about("Simple blockchain in rust.")
            .subcommand(Command::new("print_chain").about("Print all the chain blocks."))
            .subcommand(Command::new("create_wallet").about("create a wallet"))
            .subcommand(Command::new("list_addresses").about("list all addresses"))
            .subcommand(Command::new("re_index").about("reindex UTXO"))
            .subcommand(
                Command::new("get_balance")
                    .about("get balance in the blochain")
                    .arg(arg!(<ADDRESS>"'The Address it get balance for'")),
            )
            .subcommand(
                Command::new("start_node")
                    .about("start the node server")
                    .arg(arg!(<PORT>"'the port server bind to locally'")),
            )
            .subcommand(
                Command::new("create")
                    .about("Create new blochain")
                    .arg(arg!(<ADDRESS>"'The address to send gensis block reqward to' ")),
            )
            .subcommand(
                Command::new("send")
                    .about("send  in the blockchain")
                    .arg(arg!(<FROM>" 'Source wallet address'"))
                    .arg(arg!(<TO>" 'Destination wallet address'"))
                    .arg(arg!(<AMOUNT>" 'Destination wallet address'"))
                    .arg(arg!(-m --mine " 'the from address mine immediately'")),
            )
            .subcommand(
                Command::new("start_miner")
                    .about("start the minner server")
                    .arg(arg!(<PORT>" 'the port server bind to locally'"))
                    .arg(arg!(<ADDRESS>" 'wallet address'")),
            )
            .get_matches();

        if matches.subcommand_matches("print_chain").is_some() {
            cmd_print_chain()?;
        }

        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                Blockchain::create_blockchain(address.clone())?;
                println!("Blockchain created!");
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("get_balance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = Blockchain::new()?;
                let utxos = bc.find_utxo(&address);
                let mut balance: i32 = 0;

                for out in utxos {
                    balance += out.value;
                }

                println!("Balance of: '{}'; {}", address, balance)
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let mut bc = Blockchain::new()?;
            let tx = Transaction::new_utxo(from, to, amount, &bc)?;
            bc.add_block(vec![tx])?;
            println!("Success!");
        }

        Ok(())
    }
}

fn cmd_print_chain() -> Result<()> {
    let bc = Blockchain::new()?;
    for b in bc.iter() {
        println!("{:#?}", b);
    }
    Ok(())
}
