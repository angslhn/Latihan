use std::fs;
use std::path::Path;
use std::error::Error;
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Scheduling {
  pub event: String,
  pub start_time: SchedulingTime,
  pub end_time: SchedulingTime
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchedulingTime {
  pub hour: u8,
  pub minute: u8,
}

const JSON_PATH: &str = "./data/schedulings.json";

pub fn read_json() -> Result<Vec<Scheduling>, Box<dyn Error>> {
  let file_path: &Path = Path::new(JSON_PATH);

  let json_content: String = fs::read_to_string(file_path)?;

  let schedulings: Vec<Scheduling> = serde_json::from_str(&json_content)?;

  Ok(schedulings)
}

pub fn write_json(schedulings: &Vec<Scheduling>) -> Result<(), Box<dyn Error>> {
  let file_path: &Path = Path::new(JSON_PATH);

  let json_content: String = serde_json::to_string_pretty(&schedulings)?;

  fs::write(file_path, json_content)?;

  Ok(())
}