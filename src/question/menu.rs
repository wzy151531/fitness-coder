use std::env;

use crate::{
    i18n::{load_resources, translate},
    question::{
        body_metabolism::get_daily_energy_expenditure,
        energy_intake::get_carbon_cycle_energy_intake,
    },
};

use super::utils::transform_answer_to_list_item;

pub fn init_before_select_menu() {
    // init env
    env::set_var("LANGUAGE", "en_US");
    load_resources();
}

pub fn select_menu() {
    let select_menu_question = requestty::Question::select("select menu")
        .message(translate("menu_title"))
        .choices(vec![
            translate("menu_body_analysis"),
            translate("menu_language_setting"),
        ])
        .build();

    let select_menu_item: requestty::ListItem =
        transform_answer_to_list_item(requestty::prompt_one(select_menu_question));

    // Body analysis logic
    // calculate BMR and give energy intake suggestion
    if select_menu_item.index == 0 {
        // TODO: disk cache
        let (daily_energy_expenditure, body_information) = get_daily_energy_expenditure(None);
        println!(
            "{}{}{}",
            translate("daily_energy_expenditure_result_text"),
            daily_energy_expenditure,
            translate("daily_energy_expenditure_unit")
        );
        let carbon_cycle_energy_intake =
            get_carbon_cycle_energy_intake(daily_energy_expenditure, body_information.get_weight());
        println!(
            "{}{}",
            translate("suggested_carbon_cycle_energy_intake_result_text"),
            carbon_cycle_energy_intake
        )
    }

    if select_menu_item.index == 1 {
        let select_language_question = requestty::Question::select("select language")
            .message("Select display language")
            .choices(vec!["English", "简体中文"])
            .build();
        let select_language_item: requestty::ListItem =
            transform_answer_to_list_item(requestty::prompt_one(select_language_question));

        if select_language_item.index == 0 {
            env::set_var("LANGUAGE", "en_US")
        } else {
            env::set_var("LANGUAGE", "zh_CN")
        }

        println!("-----------------------");

        select_menu();
    }
}
