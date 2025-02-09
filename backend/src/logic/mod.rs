use std::collections::HashMap;

pub mod simulation;

#[derive(Clone, Copy, serde::Deserialize, Debug)]
pub struct Settings {
    base_rate: f32,
    limited_rate: f32,
    limited_options: u32,
    guaranteed_after: u32,

    hard_pity: u32,
    soft_pity: u32,
    soft_pity_increment: f32,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Inputs {
    pub pulls: u32,
    pub simulations: u32,

    pub targets: Vec<SimulationData>,
    pub settings: Vec<Settings>,
}

#[derive(Clone, Copy, serde::Deserialize, Debug)]
pub struct SimulationData {
    copies: u32,
    pity: u32,
    is_guaranteed: bool,
    losses: u32,
}


#[derive(serde::Serialize)]
pub struct SimulationOutput {
    pub successfull_simulations: u32,
    pub pulls_map: HashMap<u32, u32>,
}

#[derive(serde::Serialize)]
pub struct CalculationOutput {
    pub odds: f32
}