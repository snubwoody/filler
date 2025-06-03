//! Library for generating dummy data.
mod error;
pub mod generator;
use clap::{Parser, Subcommand};
pub use error::{Error,Result};
use crate::generator::{DateGen, Generator, NameGen, UuidGen};


/// Generate realistic dummy data
#[derive(Parser)]
#[command(version, about)]
struct Cli{
	#[command(subcommand)]
	command: CliCommand
}

#[derive(Subcommand)]
enum CliCommand{
	Gen{
		#[command(subcommand)]
		command: GenCommand	
	}
}

#[derive(Subcommand)]
enum GenCommand{
	Uuids{
		/// The number of uuids to generate
		#[arg(short,long,default_value_t=1000)]
		count: u32
	},
	Names{
		/// The number of names to generate
		#[arg(short,long,default_value_t=1000)]
		count: u32
	},
	Dates{
		/// The number of dates to generate
		#[arg(short,long,default_value_t=1000)]
		count: u32,
	}
}

pub fn cli_main(){
	let cli = Cli::parse();

	match &cli.command {
		CliCommand::Gen { command } => {
			gen_main(command);
		}
	}
}

fn gen_main(command: &GenCommand){
	match command {
		GenCommand::Uuids { count } => {
			let uuid_gen = UuidGen::new();
			let ids = uuid_gen.generate_many(*count);
			println!("{:?}",ids)
		},
		GenCommand::Names { count } => {
			let name_gen = NameGen::new().unwrap();
			let names = name_gen.generate_many(*count);
			println!("{:?}",names);
		}
		GenCommand::Dates { count } => {
			let date_gen = DateGen::new();
			let dates = date_gen.generate_many(*count);
			println!("{:?}",dates);
		}
	}
}