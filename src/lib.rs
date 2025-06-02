//! Library for generating dummy data.
mod error;
pub mod date;
pub mod generator;
pub mod name;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use date::gen_date;
use name::generate_name;
use uuid::Uuid;
pub use error::{Error,Result};


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
			// TODO allow selecting uuid version
			let ids: Vec<Uuid> = (0..*count).map(|_|Uuid::new_v4()).collect();
			println!("{:?}",ids)
		},
		GenCommand::Names { count } => {
			let config = name::parse_config().unwrap();
			let names: Vec<String> = (0..*count).map(|_|generate_name(&config)).collect();
			println!("{:?}",names);
		}
		GenCommand::Dates { count } => {
			let dates: Vec<NaiveDate> = (0..*count).map(|_|gen_date()).collect();
			println!("{:?}",dates);
		}
	}
}