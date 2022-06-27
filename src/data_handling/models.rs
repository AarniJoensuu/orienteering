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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Performance
{
  pub year: u32,
  pub leg: u8,
  pub leg_distance: f32,
  pub h: u8,
  pub m: u8,
  pub s: u8,
  pub placement_in_own_leg: u32,
  pub placement_change: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AthletePerformances
{
  athlete: String,
  performances: Vec<Performance>
}
