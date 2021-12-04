#[derive(Debug)]
pub struct SimulationInfo {
    pub simulation_count: u32,
    pub original_scores: Vec<u32>,
    pub current_event: u32,
    pub completed_non_elimination_event_count: u32,
    pub per_day_multiplier: f32,
}
