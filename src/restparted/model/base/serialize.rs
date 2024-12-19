use serde_json::Value;

pub trait Serializable {
	fn to_json(&self) -> Value;
	fn serialize(&self) -> Value {
		self.to_json()
	}
}

pub trait Deserializable: Sized {
	type Error;
	fn from_json(data: Value) -> Result<Self, Self::Error>;
	fn deserialize(data: Value) -> Result<Self, Self::Error> {
		Self::from_json(data)
	}
}
