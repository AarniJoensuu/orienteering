use crate::data_handling::models::{ManyYearsLegs};
use std::fs::{read_to_string};

pub fn read_leg_data(path: &str) -> ManyYearsLegs
{
  let data = read_to_string(path).expect("Unable to read file");
  let json: ManyYearsLegs = serde_json::from_str(&data)
    .expect("incorrect format");
  return json;
}
