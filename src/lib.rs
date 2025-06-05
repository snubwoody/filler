//! Library for generating dummy data.
mod error;
pub mod generator;
use std::{fs, path::PathBuf, process::exit};
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
				// TODO writing files without extensions is valid though
						
				// Can't use `is_file` because the file might
				// not exist yet
				if file.extension().is_none(){
					return Err(Error::InvalidPath)
				}
			}
			// Check for file extension
			let format = format.unwrap_or_default();

			match format {
				OutputFormat::Json => {
					let json = gen_json(command,*count)?;
					match path {
						Some(file) => {
							let file = fs::OpenOptions::new()
								.write(true)
								.create(true)
								.open(file)?;

							serde_json::to_writer_pretty(file, &json)?;
						},
						None => {
							let text = serde_json::to_string(&json)?;
							println!("{}",text);
						}
					}
				},
				_ => {}
			}
		}
	}
	Ok(())
}


fn gen_json(command: &GenCommand,count: u32) -> crate::Result<serde_json::Value>{
	let json = match command {
		GenCommand::Uuids => {
			let uuid_gen = UuidGen::new();
			let ids = uuid_gen.generate_many(count);
			uuid_gen.json(ids)
		},
		GenCommand::Names => {
			let name_gen = NameGen::new()?;
			let names = name_gen.generate_many(count);
			name_gen.json(names)
		}
		GenCommand::Dates => {
			let date_gen = DateGen::new();
			let dates = date_gen.generate_many(count);
			date_gen.json(dates)
		}
	};

	Ok(json)
}

// TODO
//	- Test that json ext auto infers format
#[cfg(test)]
mod tests{
	use std::fs::{self, File};
	use serde_json::Value;
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
		let path = format!("temp/test-{}.json",Uuid::new_v4());
		let path = PathBuf::from(&path);
		
		let command = CliCommand::Gen { 
			count: 100, 
			path: Some(path.clone()), 
			format: Some(OutputFormat::Json), 
			command: GenCommand::Uuids 
		}; 
		
		handle_command(&command)?;

		let file = File::open(&path)?;
		let data = serde_json::from_reader::<_,Value>(file)?
			.get_mut("data")
			.unwrap()
			.take();

		let ids: Vec<Uuid> = serde_json::from_value(data)?;
		assert_eq!(ids.len(),100);
		fs::remove_file(path)?;
		Ok(())
	}
}