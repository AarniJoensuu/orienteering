use crate::data_handling::models::{AthletePerformances, Performance};

pub fn filter_performances_by_year(
  performances: Vec<AthletePerformances>,
  year: u32) -> Vec<AthletePerformances>
{
  let mut performances_of_year: Vec<AthletePerformances> = Vec::new();
  for athlete_performances in performances
  {
    let athlete = athlete_performances.athlete;
    let mut performance_of_year: Vec<Performance> = Vec::new();
    for performance in athlete_performances.performances
    {
      if performance.year == year
      {
        performance_of_year.push(performance);
        performances_of_year.push(
          AthletePerformances {
            athlete: athlete,
            performances: performance_of_year
          }
        );
        break;
      }
    }
  }
  return performances_of_year;
}

pub fn filter_performances_by_athlete_name(
  performances: Vec<AthletePerformances>,
  athlete_name: &str) -> Vec<AthletePerformances>
{
  let mut performances_of_athletes: Vec<AthletePerformances> = Vec::new();
  for performance in performances
  {
    if performance
      .athlete
      .to_lowercase()
      .contains(&String::from(athlete_name).to_lowercase())
    {
      performances_of_athletes.push(performance);
    }
  }
  return performances_of_athletes;
}
