mod data_handling;
mod formatting;
mod util;
use data_handling::read_data::{read_leg_data, read_performance_data};
use data_handling::leg_configurations::{get_one_new_legs_configuration};
use data_handling::filter_data::{filter_performances_by_year, filter_performances_by_athlete_name};
use formatting::output::{output_leg_configuration};
use util::utility_functions::{parse_str_to_int};

use clap::{Arg, App, SubCommand};

const LEG_DATA_FILE: &str = "./data/legs.json";
const PERFORMANCES_DATA_FILE: &str = "./data/performances.json";

fn main()
{
  let matches = App::new("Orienteering team data management program")
        .version("0.1.0")
        .author("Aarni Joensuu")
        .about("Handles orienteering team data")
        .subcommand(SubCommand::with_name("performances")
          .about("Get performance data")
          .arg(Arg::with_name("athlete")
            .short('t')
            .long("athlete")
            .takes_value(true)
            .help("Get performances of one athlete"))
          .arg(Arg::with_name("year")
            .short('y')
            .long("year")
            .takes_value(true)
            .help("Filter performances by year")))
        .subcommand(SubCommand::with_name("new_leg")
          .about("Produce new unused leg configurations")
          .arg(Arg::with_name("amount")
            .short('a')
            .long("amount")
            .takes_value(true)
            .help("Amount of new leg configurations")))
        .get_matches();

  match matches.subcommand()
  {
    Some(("performances", _)) =>
    {
      let mut athlete = "";
      let mut year = "";
      if let Some(sub_matches) = matches.subcommand_matches("performances")
      {
        if sub_matches.is_present("athlete")
        {
          athlete = sub_matches.value_of("athlete").unwrap();
        }

        if sub_matches.is_present("year")
        {
          year = sub_matches.value_of("year").unwrap();
        }
      }
      
      let mut performance_data = read_performance_data(PERFORMANCES_DATA_FILE);

      // filter by athlete name
      if !athlete.is_empty()
      {
        performance_data = filter_performances_by_athlete_name(performance_data, athlete);
      }

      // filter by year
      if !year.is_empty()
      {
        let year: u32 = parse_str_to_int(year);
        performance_data = filter_performances_by_year(performance_data, year);
      }

      println!("{:#?}", performance_data);
    }

    Some(("new_leg", _)) =>
    {
      let mut amount = "1";
      if let Some(sub_matches) = matches.subcommand_matches("new_leg")
      {
        if sub_matches.is_present("amount")
        {
          amount = sub_matches.value_of("amount").unwrap();
        }
      }

      let leg_data = read_leg_data(LEG_DATA_FILE);
      for i in 0..parse_str_to_int(amount)
      {
        let new_leg_configuration = get_one_new_legs_configuration(leg_data.competitions.clone());
        output_leg_configuration(&new_leg_configuration);
        if i < parse_str_to_int(amount) - 1
        {
          println!();
        }
      }
    }

    _ =>
    {
      println!("Subcommands 'performances' and 'new_leg' are supported");
    }
  }
}
