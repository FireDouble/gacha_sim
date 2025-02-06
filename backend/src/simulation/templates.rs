use std::collections::HashMap;

use super::SimulationSettings;



pub fn get_templates() -> HashMap<String, Vec<SimulationSettings>> {
    let mut templates= HashMap::new();

    templates.insert(
        "hsr".to_string(),
        vec![
            SimulationSettings {
                base_rate: 0.006,
                limited_rate: 0.5,
                limited_options: 1,
                guaranteed_after: 1,

                hard_pity: 90,
                soft_pity: 74,
                soft_pity_increment: 0.06
            },
            SimulationSettings {
                base_rate: 0.006,
                limited_rate: 0.75,
                limited_options: 1,
                guaranteed_after: 1,

                hard_pity: 80,
                soft_pity: 64,
                soft_pity_increment: 0.06
            }
        ]
    );


    templates
}