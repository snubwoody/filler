mod error;
use clap::{Parser, Subcommand};
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
	Uuid{
		#[arg(short,long,default_value_t=1000)]
		count: u32
	}
}

pub fn cli_main(){
	let cli = Cli::parse();

	match &cli.command {
		CliCommand::Gen { command } => {
			match command {
				GenCommand::Uuid { count } => {
					// TODO allow selecting uuid version
					let ids: Vec<Uuid> = (0..*count).map(|_|Uuid::new_v4()).collect();
					println!("{:?}",ids)
				}
			}
		}
	}
}