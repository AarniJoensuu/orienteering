mod data_handling;
mod formatting;
use data_handling::read_data::{read_leg_data};
use data_handling::leg_configurations::{get_one_new_legs_configuration};
use formatting::output::{output_leg_configuration};

fn main()
{
  let leg_data = read_leg_data("./data/legs.json");
  let new_leg_configuration = get_one_new_legs_configuration(leg_data.competitions);
  output_leg_configuration(&new_leg_configuration);
}
