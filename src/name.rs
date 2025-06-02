use std::{collections::HashMap, fs};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize,Deserialize,PartialEq, Eq,Hash,PartialOrd, Ord)]
#[serde(rename_all="lowercase")]
pub enum Country{
	Usa,
	Japan
}

#[derive(Debug,Deserialize)]
pub struct NameConfig{
	#[serde(flatten)]
	pub countries: HashMap<Country,CountryNames>
}

#[derive(Debug,Deserialize)]
pub struct CountryNames{
	pub first_names: Vec<String>,
	pub surnames: Vec<String>
}

pub fn parse_config() -> crate::Result<NameConfig>{
	let data = fs::read_to_string("data/names.toml")?;
	let config: NameConfig = toml::from_str(&data)?;
	Ok(config)
}

pub fn generate_name(config: &NameConfig) -> String{
	let mut rng = rand::rng();
	let country = config.countries.get(&Country::Japan).unwrap();
	let first_name = country.first_names
		.choose(&mut rng)
		.unwrap_or(&String::from("John")).clone();
	let surname = country.surnames
		.choose(&mut rng)
		.unwrap_or(&String::from("Smith")).clone();
	
	format!("{first_name} {surname}")
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn parse(){
		assert!(parse_config().is_ok());
	}
}