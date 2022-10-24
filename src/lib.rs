mod data_calculation;
mod question;

use question::body_metabolism::get_daily_energy_expenditure;

use crate::question::energy_intake::get_carbon_cycle_energy_intake;

pub fn run() {
    // TODO: disk cache
    let (daily_energy_expenditure, body_information) = get_daily_energy_expenditure(None);
    println!(
        "Your daily energy expenditure is roughly: {}kcal",
        daily_energy_expenditure
    );
    let carbon_cycle_energy_intake =
        get_carbon_cycle_energy_intake(daily_energy_expenditure, body_information.get_weight());
    println!(
        "Suggested carbon cycle energy intake: {}",
        carbon_cycle_energy_intake
    )
}
