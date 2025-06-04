mod name;
mod date;
use std::{fs, path::{Path, PathBuf}};

use serde::Serialize;
use serde_json::json;
use uuid::Uuid;
pub use name::NameGen;
pub use date::DateGen;

/// A trait for generating random/dummy data
/// 
/// In general `Generators` are cheap and it is not neccessary
/// to store and reuse them, there's little internal state.
/// 
/// The current generators are:
/// 
/// - [`DateGen`]
/// - [`UuidGen`]
/// - [`NameGen`]
/// 
pub trait Generator{
	type Output;

	fn generate(&self) -> Self::Output;
	fn generate_many(&self,count: u32) -> Vec<Self::Output>{
		(0..count).map(|_|self.generate()).collect()
	}

	/// Write the generated output to a json file
	fn write_json<P>(&self,data: Vec<Self::Output>,path: P) -> crate::Result<()>
	where 
		P: AsRef<Path>,
		Self::Output: Serialize
	{
		let body = json!({"data":data});
		let file = fs::OpenOptions::new()
			.write(true)
			.create(true)
			.open(path)?;

		serde_json::to_writer_pretty(file, &body)?;
		Ok(())
	}
}

pub struct UuidGen;

impl UuidGen{
	pub fn new() -> Self{
		Self
	}
}

impl Generator for UuidGen{
	type Output = Uuid;

	fn generate(&self) -> Self::Output {
		Uuid::new_v4()
	}
}

// TODO check that it doesnt overwrite the entire file

#[cfg(test)]
mod tests{
	use std::fs::File;
	use serde_json::Value;
	use super::*;

	#[test]
	fn write_uuid_to_json() -> crate::Result<()>{
		let num: u32 = rand::random();
		let path = format!("./temp/test-{num}.json");
		let uuid_gen = UuidGen::new();
		let ids = uuid_gen.generate_many(100);
		uuid_gen.write_json(ids, &path)?;
		
		let file = File::open(&path)?;
		let data: Value = serde_json::from_reader(file)?;
		let ids = data.get("data").unwrap().as_array().unwrap();
		assert_eq!(ids.len(),100);
		fs::remove_file(&path)?;

		Ok(())
	}
}