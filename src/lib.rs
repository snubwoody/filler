//! Library for generating dummy data.
mod error;
pub mod generator;
use std::{path::PathBuf, process::exit};
use clap::{Parser, Subcommand, ValueEnum};
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
		/// The number of items to generate
		#[arg(short,long,default_value_t=1000)]
		count: u32,
		#[arg(short='o',long="out")]
		path: Option<PathBuf>,
		#[arg(short,long,value_enum)]
		format: Option<OutputFormat>,
		
		#[command(subcommand)]
		command: GenCommand	
	}
}

#[derive(Subcommand)]
enum GenCommand{
	Uuids{
		/// The number of uuids to generate
		#[arg(short,long,default_value_t=1000)]
		count: u32,
		#[arg(short='o',long="out")]
		path: PathBuf
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

#[derive(Debug,Clone,Copy,PartialEq,ValueEnum)]
pub enum OutputFormat{
	Json,
	Yaml,
	Text,
	Toml
}

pub fn main(){
	let cli = Cli::parse();
	let result = handle_command(&cli.command);
	if let Err(error) = result{
		println!("{:#?}",error);
		exit(1)
	}
}

fn handle_command(command: &CliCommand) -> crate::Result<()>{
	match command {
		CliCommand::Gen { command, count, path,format } => {
			if let Some(file) = path{
				if !file.is_file(){
					return Err(Error::InvalidPath)
				}
			}
			configure_generator(command)?
		}
	}
	Ok(())
}

/// Configure a generator to use
fn configure_generator(command: &GenCommand) -> crate::Result<()>{
	match command {
		GenCommand::Uuids { count,path} => {
			let uuid_gen = UuidGen::new();
			let ids = uuid_gen.generate_many(*count);
			uuid_gen.write_json(ids, path).unwrap();
			// println!("{:?}",ids)
		},
		GenCommand::Names { count } => {
			let name_gen = NameGen::new()?;
			let names = name_gen.generate_many(*count);
			println!("{:?}",names);
		}
		GenCommand::Dates { count } => {
			let date_gen = DateGen::new();
			let dates = date_gen.generate_many(*count);
			println!("{:?}",dates);
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn empty_path(){
		let command = CliCommand::Gen { 
			count: 10, 
			path: Some(PathBuf::new()), 
			format: Some(OutputFormat::Json), 
			command: GenCommand::Dates { count: 10 } 
		}; 

		let err = handle_command(&command).err().unwrap();
		assert!(matches!(err,Error::InvalidPath));
	}

	#[test]
	fn folder_as_path_fails(){
		let command = CliCommand::Gen { 
			count: 10, 
			path: Some(PathBuf::new()), 
			format: Some(OutputFormat::Json), 
			command: GenCommand::Dates { count: 10 } 
		}; 

		let err = handle_command(&command);
		dbg!(err);
	}
}