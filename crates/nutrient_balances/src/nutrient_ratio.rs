use nutrients::nutrient::NutrientAmount;
use utils::base_types::Percentage;

pub struct NutrientRatio {
    parts: Vec<NutrientAmount>,
}

impl NutrientRatio {
    pub fn new(parts: Vec<NutrientAmount>) -> Result<Self, String> {
        if parts.is_empty() {
            return Err(String::from("Ratio must have at least one part"));
        }
        if parts
            .iter()
            .all(|nutrient_amount| nutrient_amount.get_value() != 0 as f64)
        {
            return Err(String::from("All parts must be non-zero"));
        }

        Ok(NutrientRatio { parts })
    }

    pub fn get_parts(&self) -> &[NutrientAmount] {
        &self.parts
    }

    pub fn get_normalization(&self) -> Vec<NutrientAmount> {
        let sum: f64 = self
            .parts
            .iter()
            .map(|nutrient_amount| nutrient_amount.get_value())
            .sum();
        self.parts
            .iter()
            .map(|nutrient_amount| nutrient_amount.clone() / sum)
            .collect()
    }

    pub fn get_percentage(&self) -> Result<Percentage, String> {
        if self.parts.len() != 2 {
            return Err(String::from("Can only get a percentage from two parts"));
        }

        let value_float = self.parts[0].get_value() / self.parts[1].get_value();
        Ok(Percentage::new(value_float))
    }

    pub fn approximate_to_n_dp(&self, precision: u8) -> Vec<NutrientAmount> {
        let scale = 10_f64.powi(precision as i32);
        let scaled_nutrient_amounts: Vec<NutrientAmount> = self
            .parts
            .iter()
            .map(|nutrient_amount| (nutrient_amount.clone() * scale).round())
            .collect();

        let highest_common_factor: i64 =
            scaled_nutrient_amounts
                .iter()
                .fold(0, |accumulator, nutrient_amount| {
                    hcf(accumulator, nutrient_amount.get_value().round() as i64)
                });
        scaled_nutrient_amounts
            .iter()
            .map(|nutrient_amount| nutrient_amount.clone() / highest_common_factor as f64)
            .collect()
    }
}

fn hcf(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a.abs()
}
