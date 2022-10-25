use crate::{
    data_calculation::body_metabolism::{
        calculate_bmr, calculate_daily_energy_expenditure, BodyInformation, SportInformation,
    },
    i18n::translate,
};

use super::utils::{
    transform_answer_to_float, transform_answer_to_int, transform_answer_to_list_item,
};

pub fn get_bmr(default_body_information: Option<BodyInformation>) -> (f32, BodyInformation) {
    let height_question = requestty::Question::float("height")
        .message(translate("body_metabolism_height_question"))
        .validate(|num, _| {
            if num.is_finite() && num > 0.0 && num < 300.0 {
                Ok(())
            } else {
                Err(translate("body_metabolism_height_question_error_message").to_owned())
            }
        });

    let weight_question = requestty::Question::float("weight")
        .message(translate("body_metabolism_weight_question"))
        .validate(|num, _| {
            if num.is_finite() && num > 0.0 && num < 300.0 {
                Ok(())
            } else {
                Err(translate("body_metabolism_weight_question_error_message").to_owned())
            }
        });

    let age_question = requestty::Question::int("age")
        .message(translate("body_metabolism_age_question"))
        .validate(|num, _| {
            if num > 0 && num < 200 {
                Ok(())
            } else {
                Err(translate("body_metabolism_age_question_error_message").to_owned())
            }
        });

    let (height_question, weight_question, age_question) = match default_body_information {
        None => (height_question, weight_question, age_question),
        Some(info) => (
            height_question.default(info.get_height() as f64),
            weight_question.default(info.get_weight() as f64),
            age_question.default(info.get_age() as i64),
        ),
    };

    let height: f32 = transform_answer_to_float(requestty::prompt_one(height_question.build()));
    let weight: f32 = transform_answer_to_float(requestty::prompt_one(weight_question.build()));
    let age: u8 = transform_answer_to_int(requestty::prompt_one(age_question.build()));

    let body_information = BodyInformation::new(height, weight, age);
    let bmr = calculate_bmr(&body_information);

    (bmr, body_information)
}

pub fn get_daily_energy_expenditure(
    default_body_information: Option<BodyInformation>,
) -> (f32, BodyInformation) {
    let (bmr, body_information) = get_bmr(default_body_information);

    let sport_ratio_question = requestty::Question::select("sport_ratio")
        .message(translate("body_metabolism_sport_strength_question"))
        .choices(vec![
            translate("body_metabolism_sport_strength_1"),
            translate("body_metabolism_sport_strength_2"),
            translate("body_metabolism_sport_strength_3"),
            translate("body_metabolism_sport_strength_4"),
            translate("body_metabolism_sport_strength_5"),
            translate("body_metabolism_sport_strength_6"),
        ])
        .build();

    let sport_ratio_item: requestty::ListItem =
        transform_answer_to_list_item(requestty::prompt_one(sport_ratio_question));

    let sport_information = SportInformation::new(1.0 + (sport_ratio_item.index as f32) * 0.2);
    let daily_energy_expenditure = calculate_daily_energy_expenditure(bmr, sport_information);

    (daily_energy_expenditure, body_information)
}
