mod data_handling;
mod formatting;
mod util;
use actix_web::{get, web, App as WebApp, HttpRequest, HttpServer, Responder};
use qstring::QString;
use data_handling::read_data::{read_leg_data, read_performance_data};
use data_handling::leg_configurations::{get_one_new_legs_configuration};
use data_handling::filter_data::{filter_performances_by_year, filter_performances_by_athlete_name};
use util::utility_functions::{parse_str_to_int};

const LEG_DATA_FILE: &str = "./data/legs.json";
const PERFORMANCES_DATA_FILE: &str = "./data/performances.json";

#[get("/performances")]
async fn get_performances(req: HttpRequest) -> impl Responder
{
  let query_str = QString::from(req.query_string());
  let year: u32 = parse_str_to_int(query_str.get("year").unwrap_or_else(|| "0"));
  let athlete: &str = query_str.get("athlete").unwrap_or_else(|| "");
  let mut performance_data = read_performance_data(PERFORMANCES_DATA_FILE);
  
  // filter by year
  if year != 0
  {
    performance_data = filter_performances_by_year(performance_data, year);
  }

  // filter by athlete
  if athlete != ""
  {
    performance_data = filter_performances_by_athlete_name(performance_data, athlete);
  }

  return web::Json(performance_data);
}

#[get("new_leg")]
async fn get_new_legs(req: HttpRequest) -> impl Responder
{
  let query_str = QString::from(req.query_string());
  let count: u32 = parse_str_to_int(query_str.get("amount").unwrap_or_else(|| "1"));
  let leg_data = read_leg_data(LEG_DATA_FILE);
  let mut new_leg_configurations: Vec<Vec<String>> = Vec::new();
  for _ in 0..count
  {
    new_leg_configurations.push(get_one_new_legs_configuration(leg_data.competitions.clone()));
  }
  return web::Json(new_leg_configurations);
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
  HttpServer::new(||
  {
    WebApp::new()
      .service(get_performances)
      .service(get_new_legs)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
