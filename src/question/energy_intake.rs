use crate::{
    data_calculation::energy_intake::{
        calculate_carbon_cycle_energy_intake, CarbonCycleEnergyIntake,
    },
    i18n::translate,
};

use super::utils::{transform_answer_to_int, transform_answer_to_list_item};

pub fn get_carbon_cycle_energy_intake(
    daily_energy_expenditure: f32,
    weight: f32,
) -> CarbonCycleEnergyIntake {
    let body_fat_percentage_question = requestty::Question::select("body fat percentage")
        .message(translate(
            "energy_intake_body_fat_percentage_select_question",
        ))
        .choices(vec![
            translate("energy_intake_body_fat_percentage_1"),
            translate("energy_intake_body_fat_percentage_2"),
            translate("energy_intake_body_fat_percentage_3"),
            translate("energy_intake_body_fat_percentage_4"),
            translate("energy_intake_body_fat_percentage_5"),
            translate("energy_intake_body_fat_percentage_6"),
            translate("energy_intake_body_fat_percentage_7"),
        ])
        .build();

    let body_fat_percentage_item: requestty::ListItem =
        transform_answer_to_list_item(requestty::prompt_one(body_fat_percentage_question));

    // Customize body fat percentage
    if body_fat_percentage_item.index == 6 {
        let customize_body_fat_percentage_question =
            requestty::Question::int("custom body fat percentage")
                .message(translate(
                    "energy_intake_body_fat_percentage_customize_question",
                ))
                .validate(|num, _| {
                    if num > 0 && num < 100 {
                        Ok(())
                    } else {
                        Err(translate(
                            "energy_intake_body_fat_percentage_customize_question_error_message",
                        )
                        .to_owned())
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
