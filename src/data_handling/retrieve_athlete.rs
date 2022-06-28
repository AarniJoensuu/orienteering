use crate::data_handling::models::{AthletePerformances};

pub fn read_athlete_data(athlete_name: &str, all_performances: &[AthletePerformances]) -> AthletePerformances
{
  let mut iter = all_performances.into_iter();
  iter
    .find(|&x| x.athlete.to_lowercase().contains(&athlete_name.to_lowercase()))
    .unwrap()
    .clone()
}
