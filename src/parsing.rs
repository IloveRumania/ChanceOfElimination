use std::error::Error;
use std::fs;

use toml::Value;

use super::athlete::Athlete;
use super::simulation::SimulationInfo;

pub fn parse_athletes_toml_file(path: &str) -> Result<Value, Box<dyn Error>> {
    let file_contents = fs::read_to_string(&path)?;
    let athletes_toml = file_contents.parse::<Value>()?;

    Ok(athletes_toml)
}

pub fn get_simulation_info(toml: &Value) -> Option<SimulationInfo> {
    let simulation_count = toml["simulation_count"].as_integer()? as u32;
    let original_scores = toml["original_scores"]
        .as_array()?
        .iter()
        .map(|score| score.as_integer().unwrap() as u32)
        .collect::<Vec<_>>();

    let current_event = toml["current_event"].as_integer()? as u32;
    let completed_non_elimination_event_count =
        toml["completed_non_elimination_event_count"].as_integer()? as u32;

    let per_day_multiplier = toml["per_day_multiplier"].as_float()? as f32;

    Some(SimulationInfo {
        simulation_count,
        original_scores,
        current_event,
        completed_non_elimination_event_count,
        per_day_multiplier,
    })
}

pub fn get_athletes(toml: &Value) -> Option<Vec<Athlete>> {
    Some(
        toml["athletes"]
            .as_array()?
            .iter()
            .map(|athlete| Athlete {
                score: athlete["score"].as_integer().unwrap() as u32,
                name: athlete["name"].as_str().unwrap(),
            })
            .collect::<Vec<_>>(),
    )
}
