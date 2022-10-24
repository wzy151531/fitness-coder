use crate::data_calculation::energy_intake::{
    calculate_carbon_cycle_energy_intake, CarbonCycleEnergyIntake,
};

use super::utils::{transform_answer_to_int, transform_answer_to_list_item};

pub fn get_carbon_cycle_energy_intake(
    daily_energy_expenditure: f32,
    weight: f32,
) -> CarbonCycleEnergyIntake {
    let body_fat_percentage_question = requestty::Question::select("body fat percentage")
        .message("Which one is your current body fat percentage?")
        .choices(vec![
            "Clearly see your abdominals(10%)",
            "Roughly see your abdominals with waist fat(15%)",
            "Could not see your abdominals but could touch it(20%)",
            "Lots of waist fat(25%)",
            "Lots of chest fat(30%)",
            "Lots of body fat(35%)",
            "Customize",
        ])
        .build();

    let body_fat_percentage_item: requestty::ListItem =
        transform_answer_to_list_item(requestty::prompt_one(body_fat_percentage_question));

    // Customize body fat percentage
    if body_fat_percentage_item.index == 6 {
        let customize_body_fat_percentage_question =
            requestty::Question::int("custom body fat percentage")
                .message("Input your current body fat percentage(%)")
                .validate(|num, _| {
                    if num > 0 && num < 100 {
                        Ok(())
                    } else {
                        Err("Please enter a valid body fat percentage".to_owned())
                    }
                });
        let customize_body_fat_percentage: f32 = transform_answer_to_int(requestty::prompt_one(
            customize_body_fat_percentage_question.build(),
        )) as f32
            / 100.0;

        return calculate_carbon_cycle_energy_intake(
            daily_energy_expenditure,
            weight,
            customize_body_fat_percentage,
        );
    }

    calculate_carbon_cycle_energy_intake(
        daily_energy_expenditure,
        weight,
        0.1 + (body_fat_percentage_item.index as f32) * 0.05,
    )
}
