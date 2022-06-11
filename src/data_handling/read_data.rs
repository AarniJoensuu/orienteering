use crate::data_handling::models::{ManyYearsLegs};
use std::fs::{read_to_string};

pub fn read_leg_data(path: &str) -> ManyYearsLegs
{
  let data = read_to_string(path).expect("Unable to read file");
  let json: ManyYearsLegs = serde_json::from_str(&data)
    .expect("incorrect format");
  return json;
}

#[cfg(test)]
mod data_reading_tests
{
  use crate::data_handling::models::{ManyYearsLegs};
  use crate::data_handling::read_data::{read_leg_data};
  
  #[test]
  fn data_can_be_read()
  {
    let data: ManyYearsLegs = read_leg_data("./data/legs.json");
    assert!(data.competitions.len() > 0);
  }

  #[test]
  fn correct_amount_of_legs()
  {
    let data: ManyYearsLegs = read_leg_data("./data/legs.json");
    for competition in data.competitions
    {
      assert_eq!(competition.legs.len(), 7);
    }
  }
}
