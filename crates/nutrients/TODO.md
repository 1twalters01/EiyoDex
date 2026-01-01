Consider if Add and Sub impl functions from nutrition amounts should be functions in the regular impl
    They can crash and I may want to handle errors
    I could have both?
Complete ascendant sum test in nutrient list test file

Changing NutrientAmount to something else as it currently uses a fake unitless Nutrient amount for none:
pub struct NutrientAmount {
    value: f64,
    nutrient: Option<Nutrient>,
    output_unit: NutrientUnit,
}
