use uuid::Uuid;

/// A trait for generating random/dummy data
pub trait Generator{
	type Output;

	fn generate(&self) -> Self::Output;
	fn generate_many(&self,count: u32) -> Vec<Self::Output>{
		(0..count).map(|_|self.generate()).collect()
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
