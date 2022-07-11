#[allow(dead_code)] // function is used by cli application
pub fn output_leg_configuration(leg_configuration: &Vec<String>)
{
  for i in 0..leg_configuration.len()
  {
    let leg_number: usize = i + 1;
    let athlete: String = leg_configuration[i].clone();
    println!("{leg_number}. {athlete}");
  }
}
