use requestty::ListItem;

use crate::data_calculation::body_metabolism::{
    calculate_bmr, calculate_daily_energy_expenditure, BodyInformation, SportInformation,
};

use super::utils::{
    transform_answer_to_float, transform_answer_to_int, transform_answer_to_list_item,
};

pub fn get_bmr(default_body_information: Option<BodyInformation>) -> f32 {
    let height_question = requestty::Question::float("height")
        .message("Input your height(cm)")
        .validate(|num, _| {
            if num.is_finite() && num > 0.0 && num < 300.0 {
                Ok(())
            } else {
                Err("Please enter a valid height".to_owned())
            }
        });

    let weight_question = requestty::Question::float("weight")
        .message("Input your weight(kg)")
        .validate(|num, _| {
            if num.is_finite() && num > 0.0 && num < 300.0 {
                Ok(())
            } else {
                Err("Please enter a valid weight".to_owned())
            }
        });

    let age_question = requestty::Question::int("age")
        .message("Input your age(year)")
        .validate(|num, _| {
            if num > 0 && num < 200 {
                Ok(())
            } else {
                Err("Please enter a valid age".to_owned())
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
    let bmr = calculate_bmr(body_information);

    bmr
}

pub fn get_daily_energy_expenditure(default_body_information: Option<BodyInformation>) -> f32 {
    let bmr = get_bmr(default_body_information);

    let sport_ratio_question = requestty::Question::select("sport_ratio")
        .message("Which one is your daily sport strength?")
        .choices(vec![
            "Sedentray", "Very light", "Light", "Moderate", "High", "Extreme",
        ])
        .build();

    let sport_ratio_item: ListItem =
        transform_answer_to_list_item(requestty::prompt_one(sport_ratio_question));

    let sport_information = SportInformation::new(1.0 + (sport_ratio_item.index as f32) * 0.2);
    let daily_energy_expenditure = calculate_daily_energy_expenditure(bmr, sport_information);

    daily_energy_expenditure
}
