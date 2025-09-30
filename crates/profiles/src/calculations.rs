use crate::profile::{Gender, Profile};

impl Profile {
    pub fn body_mass_index(&self) -> f64 {
        let height_m = self.get_height().to_m().get_value();
        let weight_kg = self.get_weight().to_kg().get_value();

        weight_kg / height_m.powf(2f64)
    }

    pub fn body_roundness_index(&self) -> f64 {
        let waist_to_height_ratio =
            self.get_waist_circumference().get_value() / self.get_height().get_value();
        let eccentricity = (1f64 - (waist_to_height_ratio / std::f64::consts::PI)).sqrt();
        let body_roundness_index = 364.2 - 365.5 * eccentricity;
        return body_roundness_index;
    }

    pub fn basal_metabolic_rate(&self) -> f64 {
        let gender = self.get_gender();
        let weight_kg = self.get_weight().to_kg().get_value();
        let height_cm = self.get_height().to_cm().get_value();
        let age = self.get_age();

        match gender {
            Gender::Male => (10f64 * weight_kg) + (6.25 * height_cm) - (5f64 * age as f64) + 5f64,
            Gender::Female => {
                (10f64 * weight_kg) + (6.25 * height_cm) - (5f64 * age as f64) - 161f64
            }
        }
    }

    pub fn macronutrient_ratios() {}

    pub fn macronutrient_percentages() {}
}
