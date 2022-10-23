pub struct BodyInformation {
    /**
     * Current height.
     * unit: cm
     */
    height: f32,
    /**
     * Current weight.
     * unit: kg
     */
    weight: f32,
    /**
     * Current age.
     * unit: year
     */
    age: u8,
}

impl BodyInformation {
    pub fn new(height: f32, weight: f32, age: u8) -> Self {
        BodyInformation {
            height,
            weight,
            age,
        }
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}

pub struct SportInformation {
    /**
     * The daily sport ratio.
     */
    sport_ratio: f32,
}

impl SportInformation {
    pub fn new(sport_ratio: f32) -> Self {
        SportInformation { sport_ratio }
    }
}

const BASE_BMR: f32 = 66.0;
const WEIGHT_BMR_RATIO: f32 = 13.7;
const HEIGHT_BMR_RATIO: f32 = 5.0;
const AGE_BMR_RATIO: f32 = 6.8;

/**
 * Calculate basalmetabolismrate according to body and sport information.
 */
pub fn calculate_bmr(body_information: BodyInformation) -> f32 {
    let body_age: f32 = body_information.age.into();

    BASE_BMR
        + WEIGHT_BMR_RATIO * body_information.weight
        + HEIGHT_BMR_RATIO * body_information.height
        - AGE_BMR_RATIO * body_age
}

/**
 * Calculate daily energy expenditure according to BMR and sport information.
 */
pub fn calculate_daily_energy_expenditure(bmr: f32, sport_information: SportInformation) -> f32 {
    bmr * sport_information.sport_ratio
}
