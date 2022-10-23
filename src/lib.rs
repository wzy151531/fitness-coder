mod data_calculation;
mod question;

use question::body_metabolism::get_daily_energy_expenditure;

pub fn run() {
    // TODO: disk cache
    let daily_energy_expenditure = get_daily_energy_expenditure(None);
    println!("Your daily energy expenditure is: {}", daily_energy_expenditure);
}
