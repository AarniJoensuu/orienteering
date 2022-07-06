use crate::data_handling::models::{AthletePerformances};

pub fn filter_all_performances_data_by_year(
  all_performances: Vec<AthletePerformances>,
  year: u32) -> Vec<AthletePerformances>
  {
    let mut performances_of_year: Vec<AthletePerformances> = Vec::new();
    let all_performances_iter = all_performances.into_iter();
    for athlete_performances in all_performances_iter
    {
      let athlete: String = athlete_performances.athlete;
      let athlete_performance = athlete_performances
        .performances
        .iter()
        .position(|&x| x.year == year)
        .unwrap();
      let athlete_performance_of_one_year: AthletePerformances = AthletePerformances {
        athlete: athlete,
        performances: vec![athlete_performances.performances[athlete_performance]]
      };
      performances_of_year.push(athlete_performance_of_one_year);
    }
    return performances_of_year;
  }
