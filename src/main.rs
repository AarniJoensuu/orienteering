mod data_handling;
mod formatting;
mod util;
use data_handling::read_data::{read_leg_data, read_performance_data};
use data_handling::leg_configurations::{get_one_new_legs_configuration};
use formatting::output::{output_leg_configuration};

use clap::{Arg, App};

// #[derive(Parser, Debug)]
// #[clap(author, version, about)]
// struct Args {
//   #[clap(short, long, value_parser, default_value = "")]
//   athlete: String,
// }

const LEG_DATA_FILE: &str = "./data/legs.json";
const PERFORMANCES_DATA_FILE: &str = "./data/performances.json";

fn main()
{
  let matches = App::new("Orienteering team data management program")
        .version("0.1.0")
        .author("Aarni Joensuu")
        .about("Handles orienteering team data")
        .arg(Arg::with_name("athlete")
                 .short('a')
                 .long("athlete")
                 .takes_value(true)
                 .help("Athlete name"))
        .get_matches();
  let athlete = matches.value_of("athlete").unwrap_or("");
  println!("athlete: {}", athlete);
  if !athlete.is_empty()
  {
    let data = read_performance_data(PERFORMANCES_DATA_FILE);
    println!("{:?}", data);
  }
  else
  {
    let leg_data = read_leg_data(LEG_DATA_FILE);
    let new_leg_configuration = get_one_new_legs_configuration(leg_data.competitions);
    output_leg_configuration(&new_leg_configuration);
  }
}
