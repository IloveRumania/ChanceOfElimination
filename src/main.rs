mod athlete;

use std::error::Error;
use std::fs;

use toml::Value;

use athlete::Athlete;

#[derive(Debug)]
struct SimulationInfo {
    simulation_count: u32,
    original_scores: Vec<u32>,
    current_event: u32,
    completed_non_elimination_event_count: u32,
    per_day_multiplier: f32,
}

fn main() {
    let athletes_toml = parse_athletes_toml_file("data/athletes.toml").unwrap();
    let simulation_info = get_simulation_info(&athletes_toml);

    println!("{:#?}", simulation_info);
}

fn parse_athletes_toml_file(path: &str) -> Result<Value, Box<dyn Error>> {
    let file_contents = fs::read_to_string(&path)?;
    let athletes_toml = file_contents.parse::<Value>()?;

    Ok(athletes_toml)
}

fn get_simulation_info(toml: &Value) -> Option<SimulationInfo> {
    let simulation_count = toml["simulation_count"].as_integer()? as u32;
    let original_scores = toml["original_scores"]
        .as_array()?
        .iter()
        .map(|score| score.as_integer().unwrap() as u32)
        .collect::<Vec<_>>();
    
    let current_event = toml["current_event"].as_integer()? as u32;
    let completed_non_elimination_event_count = toml["completed_non_elimination_event_count"].as_integer()? as u32;

    let per_day_multiplier = toml["per_day_multiplier"].as_float()? as f32;

    Some(SimulationInfo{
        simulation_count,
        original_scores,
        current_event,
        completed_non_elimination_event_count,
        per_day_multiplier,
    })
}
