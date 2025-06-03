use std::{collections::HashMap, fs, path::Path};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use super::Generator;


pub struct NameGen{
	config: NameConfig
}

impl NameGen{
	pub fn new() -> crate::Result<Self>{
		let config = parse_config()?;
		Ok(Self{config})
	}
}

impl Generator for NameGen{
	type Output = String;

	fn generate(&self) -> Self::Output {
		let mut rng = rand::rng();
		let country = self.config.countries.get(&Country::Japan).unwrap();
		let first_name = country.first_names
			.choose(&mut rng)
			.unwrap_or(&String::from("John")).clone();
		let surname = country.surnames
			.choose(&mut rng)
			.unwrap_or(&String::from("Smith")).clone();
		
		format!("{first_name} {surname}")
	}

	fn write_json<P>(&self,count: u32,path: P) -> crate::Result<()>
	where P: AsRef<Path> 
	{
		Ok(())
	}
}

#[derive(Debug,Serialize,Deserialize,PartialEq, Eq,Hash,PartialOrd, Ord)]
#[serde(rename_all="lowercase")]
enum Country{
	Usa,
	Japan
}

#[derive(Debug,Deserialize)]
struct NameConfig{
	#[serde(flatten)]
	countries: HashMap<Country,CountryNames>
}


#[derive(Debug,Deserialize)]
struct CountryNames{
	first_names: Vec<String>,
	surnames: Vec<String>
}

fn parse_config() -> crate::Result<NameConfig>{
	let data = fs::read_to_string("data/names.toml")?;
	let config: NameConfig = toml::from_str(&data)?;
	Ok(config)
}
