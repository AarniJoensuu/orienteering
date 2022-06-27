use crate::data_handling::models::{AthletePerformances};
use crate::data_handling::read_data::{read_athlete_data};

pub fn read_athlete_data(String athlete_name, &str filename) -> 
{
  all_performances: Vec<AthletePerformances> = read_athlete_data(athlete_name, filename);
  println!("{:?}", all_performances);
}