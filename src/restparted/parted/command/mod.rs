use std::error::Error;

use serde_json::Value;

use crate::restparted::{model::base::Deserializable, parted::models::request::Request};

pub fn run_command(input: Value) -> Result<Value, Box<dyn Error>> {
  let full_command= Request::from_json(input)?;
  todo!()
}
