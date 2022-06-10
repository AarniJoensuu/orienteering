use crate::data_handling::models::{OneYearLegs};
use rand::seq::SliceRandom;

pub fn get_one_new_legs_configuration(legs: Vec<OneYearLegs>) -> Vec<String>
{
  for _ in 0..1_000
  {
    let allocated_athletes = allocate_athletes_for_legs(&legs);
    if allocated_athletes.len() == 7
    {
      return allocated_athletes;
    }
  }
  return Vec::new();
}

fn allocate_athletes_for_legs(legs: &Vec<OneYearLegs>) -> Vec<String>
{
  let mut allocated_athletes: Vec<String> = Vec::new();
  for i in 0..7
  {
    let mut athletes_already_on_specific_leg: Vec<String> = Vec::new();
    for j in 0..legs.len()
    {
      athletes_already_on_specific_leg.push(legs[j].legs[i].clone());
    }
    let allocated_athlete = allocate_athlete_for_one_leg(
      athletes_already_on_specific_leg,
      &allocated_athletes,
      legs[0].legs.clone());
    if !allocated_athlete.is_empty()
    {
      allocated_athletes.push(allocated_athlete);
    }
    else
    {
      return Vec::new();
    }
  }
  return allocated_athletes;
}

fn allocate_athlete_for_one_leg(
  athletes_already_on_current_leg: Vec<String> ,
  athletes_already_allocated: &Vec<String> ,
  all_possible_athletes: [String; 7]) -> String
{
  let mut possible_athletes_for_leg: Vec<String> = Vec::new();
  for i in 0..7
  {
    if
      // athlete has not yet attended current leg
      !athletes_already_on_current_leg.contains(&all_possible_athletes[i])
      // athlete not yet allocated to other legs
      && !athletes_already_allocated.contains(&all_possible_athletes[i])
    {
      possible_athletes_for_leg.push(all_possible_athletes[i].clone());
    }
    else {
      continue;
    }
  }
  if possible_athletes_for_leg.is_empty()
  {
    return String::new();
  }
  return possible_athletes_for_leg.choose(&mut rand::thread_rng()).unwrap().to_string();
}
