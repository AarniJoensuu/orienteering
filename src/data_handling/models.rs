use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OneYearLegs
{
  pub year: i32,
  pub legs: [String; 7]
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManyYearsLegs
{
  pub competitions: Vec<OneYearLegs>
}
