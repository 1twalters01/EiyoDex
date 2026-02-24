Changing NutrientAmount to something else as it currently uses a fake unitless Nutrient amount for none:
pub struct NutrientAmount {
    value: f64,
    nutrient: Option<Nutrient>,
    output_unit: NutrientUnit,
}
In fact, it probably shouldn't use ord at all
