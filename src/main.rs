mod data_handling;
mod formatting;
mod util;
use data_handling::read_data::{read_leg_data, read_performance_data};
use data_handling::leg_configurations::{get_one_new_legs_configuration};
use data_handling::retrieve_athlete::{read_athlete_data};
use data_handling::filter_data::{filter_all_performances_data_by_year};
use formatting::output::{output_leg_configuration};
use util::utility_functions::{parse_str_to_int};

use clap::{Arg, App};

const LEG_DATA_FILE: &str = "./data/legs.json";
const PERFORMANCES_DATA_FILE: &str = "./data/performances.json";

fn main()
{
  let matches = App::new("Orienteering team data management program")
        .version("0.1.0")
        .author("Aarni Joensuu")
        .about("Handles orienteering team data")
        .arg(Arg::with_name("performances")
          .short('p')
          .long("performances")
          .takes_value(false)
          .help("Get all performances"))
        .arg(Arg::with_name("athlete")
          .short('a')
          .long("athlete")
          .takes_value(true)
          .help("Get performances of one athlete"))
        .arg(Arg::with_name("year")
          .short('y')
          .long("year")
          .takes_value(true)
          .help("Filter performances by year"))
        .arg(Arg::with_name("new_leg")
          .short('l')
          .long("new-leg")
          .takes_value(false)
          .help("Produce new leg configuration"))
        .get_matches();
  let get_performaces: bool = matches.is_present("performances");
  let athlete: &str = matches.value_of("athlete").unwrap_or("");
  let year_provided: bool = matches.is_present("year");
  let new_leg: bool = matches.is_present("new_leg");
  
  // get all performances of all athletes
  if get_performaces
  {
    let mut performances_data = read_performance_data(PERFORMANCES_DATA_FILE);
    // filter by year
    if year_provided
    {
      let year: u32 = parse_str_to_int(matches.value_of("year").unwrap());
      performances_data = filter_all_performances_data_by_year(performances_data, year);
    }
    println!("{:#?}", performances_data);
  }

  // get all performances of one athlete
  if !athlete.is_empty()
  {
    let mut performance_data = read_performance_data(PERFORMANCES_DATA_FILE);
    // filter by year
    if year_provided
    {
      let year: u32 = parse_str_to_int(matches.value_of("year").unwrap());
      performance_data = filter_all_performances_data_by_year(performance_data, year);
    }
    println!("{:#?}", read_athlete_data(athlete, &performance_data));
  }
  
  // produce new leg
  if new_leg
  {
    let leg_data = read_leg_data(LEG_DATA_FILE);
    let new_leg_configuration = get_one_new_legs_configuration(leg_data.competitions);
    output_leg_configuration(&new_leg_configuration);
  }
}
