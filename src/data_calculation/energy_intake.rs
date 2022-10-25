use std::fmt;

use crate::i18n::translate;

pub struct EnergyIntake {
    carbohydrate: f32,
    protein: f32,
    fat: f32,
}

impl EnergyIntake {
    fn new(carbohydrate: f32, protein: f32, fat: f32) -> Self {
        EnergyIntake {
            carbohydrate,
            protein,
            fat,
        }
    }
}

impl fmt::Display for EnergyIntake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}{}, {}: {}{}, {}: {}{}",
            translate("carbohydrate"),
            self.carbohydrate,
            translate("energy_intake_unit"),
            translate("protein"),
            self.protein,
            translate("energy_intake_unit"),
            translate("fat"),
            self.fat,
            translate("energy_intake_unit"),
        )
    }
}

pub struct CarbonCycleEnergyIntake {
    low: EnergyIntake,
    mid: EnergyIntake,
    high: EnergyIntake,
}

impl CarbonCycleEnergyIntake {
    pub fn new(low: EnergyIntake, mid: EnergyIntake, high: EnergyIntake) -> Self {
        CarbonCycleEnergyIntake { low, mid, high }
    }
}

impl fmt::Display for CarbonCycleEnergyIntake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n==============================\n{}: {}\n==============================\n{}: {}\n==============================\n{}: {}\n==============================\n",
            translate("low_carbon_day"), self.low, translate("mid_carbon_day"), self.mid, translate("high_carbon_day"), self.high
        )
    }
}

const KG_TO_LBS_RATIO: f32 = 2.20462262;
const MID_CARBOHYDRATE_RATIO: f32 = 1.25;
const LOW_CARBOHYDRATE_RATIO: f32 = 0.25;
const HIGH_CARBOHYDRATE_RATIO: f32 = 1.75;
const HIGH_FAT_RATIO: f32 = 0.5;
const LOW_FAT_RATIO: f32 = 1.5;
const PROTEIN_RATIO: f32 = 1.2;

pub fn calculate_mid_carbon_day_energy_intake(
    daily_energy_expenditure: f32,
    weight: f32,
    body_fat_percentage: f32,
) -> EnergyIntake {
    let carbohydrate: f32 = MID_CARBOHYDRATE_RATIO * weight * KG_TO_LBS_RATIO;
    let protein: f32 = PROTEIN_RATIO * weight * (1.0 - body_fat_percentage);
    let fat: f32 = (daily_energy_expenditure - (carbohydrate + protein) * 4.0) / 9.0;

    EnergyIntake::new(carbohydrate, protein, fat)
}

pub fn calculate_high_carbon_day_energy_intake(
    daily_energy_expenditure: f32,
    weight: f32,
    body_fat_percentage: f32,
) -> EnergyIntake {
    let EnergyIntake {
        carbohydrate,
        protein,
        fat,
    } = calculate_mid_carbon_day_energy_intake(
        daily_energy_expenditure,
        weight,
        body_fat_percentage,
    );
    EnergyIntake::new(
        carbohydrate * HIGH_CARBOHYDRATE_RATIO,
        protein,
        fat * HIGH_FAT_RATIO,
    )
}

pub fn calculate_low_carbon_day_energy_intake(
    daily_energy_expenditure: f32,
    weight: f32,
    body_fat_percentage: f32,
) -> EnergyIntake {
    let EnergyIntake {
        carbohydrate,
        protein,
        fat,
    } = calculate_mid_carbon_day_energy_intake(
        daily_energy_expenditure,
        weight,
        body_fat_percentage,
    );
    EnergyIntake::new(
        carbohydrate * LOW_CARBOHYDRATE_RATIO,
        protein,
        fat * LOW_FAT_RATIO,
    )
}

pub fn calculate_carbon_cycle_energy_intake(
    daily_energy_expenditure: f32,
    weight: f32,
    body_fat_percentage: f32,
) -> CarbonCycleEnergyIntake {
    CarbonCycleEnergyIntake::new(
        calculate_low_carbon_day_energy_intake(
            daily_energy_expenditure,
            weight,
            body_fat_percentage,
        ),
        calculate_mid_carbon_day_energy_intake(
            daily_energy_expenditure,
            weight,
            body_fat_percentage,
        ),
        calculate_high_carbon_day_energy_intake(
            daily_energy_expenditure,
            weight,
            body_fat_percentage,
        ),
    )
}
