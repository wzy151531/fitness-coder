use crate::data_calculation::energy_intake::{
    calculate_carbon_cycle_energy_intake, CarbonCycleEnergyIntake,
};

use super::utils::transform_answer_to_list_item;

pub fn get_carbon_cycle_energy_intake(
    daily_energy_expenditure: f32,
    weight: f32,
) -> CarbonCycleEnergyIntake {
    let body_fat_percentage_question = requestty::Question::select("body fat percentage")
        .message("Which one is your curent body fat percentage?")
        .choices(vec![
            "(10%)",
            "(15%)",
            "(20%)",
            "(25%)",
            "(30%)",
            "(35%)",
        ])
        .build();

    let body_fat_percentage_item: requestty::ListItem =
        transform_answer_to_list_item(requestty::prompt_one(body_fat_percentage_question));

    calculate_carbon_cycle_energy_intake(daily_energy_expenditure, weight, 0.1 + (body_fat_percentage_item.index as f32) * 0.05)
}
