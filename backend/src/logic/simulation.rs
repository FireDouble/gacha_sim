use std::{cmp::max, collections::HashMap};

use super::{SimulationData, Inputs, SimulationOutput, Settings};
use rand::prelude::*;

enum PullResult {
    Limited,
    Standard,
    None,
}

pub fn simulate(inputs: &mut Inputs) -> SimulationOutput {
    let mut successfull_simulations: u32 = 0;
    let mut rng = rand::rng();
    let mut pulls_map = HashMap::new();

    for _ in 0..inputs.simulations {
        let (result, pulls_left) = simulation(inputs, inputs.pulls, &mut rng);

        if result {
            successfull_simulations += 1;
            pulls_map.insert(pulls_left, pulls_map.get(&pulls_left).unwrap_or(&0) + 1);
        }
    }

    SimulationOutput {
        successfull_simulations,
        pulls_map,
    }
}

fn simulation(inputs: &mut Inputs, pulls: u32, rng: &mut ThreadRng) -> (bool, u32) {
    let mut pulls_left = pulls;
    let mut achieved_targets = 0;
    for index in 0..inputs.targets.len() {
        let target = inputs
            .targets
            .get(index)
            .expect("Simulation target missing");
        let settings = inputs
            .settings
            .get(index)
            .expect("Simulation settings missing");
        let mut data = SimulationData {
            pity: target.pity,
            copies: 0,
            is_guaranteed: target.is_guaranteed,
            losses: 0,
        };


        if target.copies == 0 {
            achieved_targets += 1;
            continue;
        }

        while pulls_left > 0 && data.copies < target.copies {
            if settings.guaranteed_after == 0 {
                data.is_guaranteed = true;
            }
                
            let succ = pull(&settings, &data, rng);

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
                }
                PullResult::Standard => {
                    data.losses += 1;
                    data.is_guaranteed = data.losses >= settings.guaranteed_after;
                    data.pity = 0;
                }
                _ => {
                    data.pity += 1;
                }
            }

            pulls_left -= 1;
        }
    }

    if achieved_targets < inputs.targets.len() {
        return (false, 0);
    }

    return (true, pulls_left);
}

fn pull(settings: &Settings, data: &SimulationData, rng: &mut ThreadRng) -> PullResult {
    let rng1: f32 = rng.random();
    let rate = settings.base_rate
        + settings.soft_pity_increment
            * max(data.pity as i64 - settings.soft_pity as i64, 0) as f32;

    if rng1 <= rate || data.pity + 1 == settings.hard_pity {
        if data.is_guaranteed {
            return PullResult::Limited;
        }

        let rng2: f32 = rng.random();

        if rng2 <= settings.limited_rate / settings.limited_options as f32 {
            return PullResult::Limited;
        }

        return PullResult::Standard;
    }

    PullResult::None
}
