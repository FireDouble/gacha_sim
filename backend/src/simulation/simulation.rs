use std::cmp::max;

use crate::simulation::SimulationTarget;

use super::{SimulationInput, SimulationSettings};
use rand::prelude::*;


enum PullResult {
    Limited,
    Standard,
    None,
}

#[derive(Clone, Copy)]
struct SimulationData {
    copies: u32,
    pity: u32,
    is_guaranteed: bool,
    losses: u32,
}

impl Default for SimulationData {
    fn default() -> Self {
        Self { copies: 0, pity: 0, is_guaranteed: false, losses: 0 }
    }
}

pub fn run_simulations(inputs: &mut SimulationInput, settings: &Vec<SimulationSettings>) -> (f32, u32) {
    let mut successfull_simulations: u32 = 0;
    let mut rng = rand::rng();
    let mut average_pulls_left = 0;

    for _ in 0..inputs.simulation_count {
        let (result, pulls_left) = run_simulation(settings, inputs.targets.clone(), inputs.pulls, &mut rng);

        if result { successfull_simulations += 1; }

        average_pulls_left += pulls_left;
    }

    return ((successfull_simulations as f32 / inputs.simulation_count as f32) * 100., average_pulls_left as u32  / inputs.simulation_count as u32);
}

fn run_simulation(settings: &Vec<SimulationSettings>, targets: Vec<SimulationTarget>, pulls: u32, rng: &mut ThreadRng) -> (bool, u32) {
    let mut pulls_left = pulls;
    let mut achieved_targets = 0;
    for index in 0..targets.len() {
        let target = targets.get(index as usize).expect("Simulation target missing");
        let settings = settings.get(index as usize).expect("Simulation srettings missing");
        let mut data = SimulationData { is_guaranteed: target.is_guaranteed, ..Default::default() };

        if target.copies == 0 { achieved_targets += 1; }
        
        while pulls_left > 0 && data.copies < target.copies {
            let succ = pull(settings, data, rng);

            match succ {
                PullResult::Limited => {
                    data.copies += 1;
                    data.pity = 0;
                    data.is_guaranteed = false;
                    data.losses = 0;

                    if data.copies >= target.copies {
                        achieved_targets += 1;
                        break;
                    }
                },
                PullResult::Standard => {
                    data.losses += 1;
                    data.is_guaranteed = data.losses >= settings.guaranteed_after;
                    data.pity = 0;
                },
                _ => {
                    data.pity += 1;
                },
            }

            pulls_left -= 1;
        }

        if achieved_targets < targets.len() { continue; }

        return (true, pulls_left);
    }

    return (false, 0);
}

fn pull(settings: &SimulationSettings, data: SimulationData, rng: &mut ThreadRng) -> PullResult {
    let rng1: f32 = rng.random();
    let rate = settings.base_rate + settings.soft_pity_increment * max(data.pity as i64 - settings.soft_pity as i64, 0) as f32;

    if rng1 <= rate || data.pity + 1 == settings.hard_pity {
        if data.is_guaranteed { return PullResult::Limited; }

        let rng2: f32 = rng.random();

        if rng2 <= settings.limited_rate / settings.limited_options as f32 { return PullResult::Limited }

        return PullResult::Standard
    }

    return PullResult::None;
}