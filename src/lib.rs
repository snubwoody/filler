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
	Uuids,
	Names,
	Dates
}

#[derive(Debug,Clone,Copy,PartialEq,ValueEnum,Default)]
pub enum OutputFormat{
	#[default]
	Text,
	Json,
	Yaml,
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
			// Check for file extension
			let format = format.unwrap_or_default();

			match format {
				OutputFormat::Json => {
					gen_json(command,*count)?

				},
				_ => {}
			}
		}
	}
	Ok(())
}


fn gen_json(command: &GenCommand,count: u32) -> crate::Result<()>{
	match command {
		GenCommand::Uuids => {
			let uuid_gen = UuidGen::new();
			let ids = uuid_gen.generate_many(count);
			uuid_gen.json(ids);
		},
		GenCommand::Names => {
			let name_gen = NameGen::new()?;
			let names = name_gen.generate_many(count);
			println!("{:?}",names);
		}
		GenCommand::Dates => {
			let date_gen = DateGen::new();
			let dates = date_gen.generate_many(count);
			println!("{:?}",dates);
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests{
	use std::fs;
	use uuid::Uuid;
	use super::*;

	#[test]
	fn empty_path(){
		let command = CliCommand::Gen { 
			count: 10, 
			path: Some(PathBuf::new()), 
			format: Some(OutputFormat::Json), 
			command: GenCommand::Dates 
		}; 

		let err = handle_command(&command).err().unwrap();
		assert!(matches!(err,Error::InvalidPath));
	}
	
	#[test]
	fn folder_as_path_fails() -> crate::Result<()>{
		let path = format!("temp/test-{}",Uuid::new_v4());
		fs::create_dir(&path)?;
		let path = PathBuf::from(&path);
		
		let command = CliCommand::Gen { 
			count: 10, 
			path: Some(path.clone()), 
			format: Some(OutputFormat::Json), 
			command: GenCommand::Dates 
		}; 
		
		let err = handle_command(&command).err().unwrap();
		fs::remove_dir(path)?;
		assert!(matches!(err,Error::InvalidPath));
		Ok(())
	}

	#[test]
	fn gen_uuids() -> crate::Result<()>{
		let path = format!("temp/test-{}",Uuid::new_v4());
		let path = PathBuf::from(&path);
		
		let command = CliCommand::Gen { 
			count: 100, 
			path: Some(path.clone()), 
			format: Some(OutputFormat::Json), 
			command: GenCommand::Uuids 
		}; 
		
		let err = handle_command(&command).err().unwrap();
		fs::remove_dir(path)?;
		assert!(matches!(err,Error::InvalidPath));
		Ok(())
	}
}