pub mod simulation;
pub mod templates;

#[derive(Clone, Copy)]
pub struct SimulationSettings {
    base_rate: f32,
    limited_rate: f32,
    limited_options: u32,
    guaranteed_after: u32,

    hard_pity: u32,
    soft_pity: u32,
    soft_pity_increment: f32,
}

#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct SimulationTarget {
    pub pity: u32,
    pub copies: u32,
    pub is_guaranteed: bool,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct SimulationInput {
    pub pulls: u32,
    pub simulation_count: u32,

    pub targets: Vec<SimulationTarget>,
}