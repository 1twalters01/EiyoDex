use std::{collections::BTreeSet, str::FromStr};
use units::{
    density::{measurement_system::DensityMeasurementSystem, unit::DensityUnit},
    mass::unit::MassUnit,
    measurement_system::MeasurementSystem,
    volume::unit::VolumeUnit,
};

#[test]
fn test_from_variants() {
    let gram = MassUnit::Gram;
    let kilogram = MassUnit::Kilogram;
    let milligram = MassUnit::Milligram;
    let microgram = MassUnit::Microgram;
    let ounce = MassUnit::Ounce;

    let liter = VolumeUnit::Liter;
    let milliliter = VolumeUnit::Milliliter;
    let pint = VolumeUnit::Pint;
    let gallon = VolumeUnit::Gallon;
    let fluid_ounce = VolumeUnit::FluidOunce;
    let tablespoon = VolumeUnit::Tablespoon;
    let teaspoon = VolumeUnit::Teaspoon;

    assert_eq!(
        DensityUnit::from_variants(gram, liter),
        DensityUnit::GramPerLiter
    );
    assert_eq!(
        DensityUnit::from_variants(gram, milliliter),
        DensityUnit::GramPerMilliliter
    );
    assert_eq!(
        DensityUnit::from_variants(gram, pint),
        DensityUnit::GramPerPint
    );
    assert_eq!(
        DensityUnit::from_variants(gram, gallon),
        DensityUnit::GramPerGallon
    );
    assert_eq!(
        DensityUnit::from_variants(gram, fluid_ounce),
        DensityUnit::GramPerFluidOunce
    );
    assert_eq!(
        DensityUnit::from_variants(gram, tablespoon),
        DensityUnit::GramPerTablespoon
    );
    assert_eq!(
        DensityUnit::from_variants(gram, teaspoon),
        DensityUnit::GramPerTeaspoon
    );

    assert_eq!(
        DensityUnit::from_variants(kilogram, liter),
        DensityUnit::KilogramPerLiter
    );
    assert_eq!(
        DensityUnit::from_variants(kilogram, milliliter),
        DensityUnit::KilogramPerMilliliter
    );
    assert_eq!(
        DensityUnit::from_variants(kilogram, pint),
        DensityUnit::KilogramPerPint
    );
    assert_eq!(
        DensityUnit::from_variants(kilogram, gallon),
        DensityUnit::KilogramPerGallon
    );
    assert_eq!(
        DensityUnit::from_variants(kilogram, fluid_ounce),
        DensityUnit::KilogramPerFluidOunce
    );
    assert_eq!(
        DensityUnit::from_variants(kilogram, tablespoon),
        DensityUnit::KilogramPerTablespoon
    );
    assert_eq!(
        DensityUnit::from_variants(kilogram, teaspoon),
        DensityUnit::KilogramPerTeaspoon
    );

    assert_eq!(
        DensityUnit::from_variants(milligram, liter),
        DensityUnit::MilligramPerLiter
    );
    assert_eq!(
        DensityUnit::from_variants(milligram, milliliter),
        DensityUnit::MilligramPerMilliliter
    );
    assert_eq!(
        DensityUnit::from_variants(milligram, pint),
        DensityUnit::MilligramPerPint
    );
    assert_eq!(
        DensityUnit::from_variants(milligram, gallon),
        DensityUnit::MilligramPerGallon
    );
    assert_eq!(
        DensityUnit::from_variants(milligram, fluid_ounce),
        DensityUnit::MilligramPerFluidOunce
    );
    assert_eq!(
        DensityUnit::from_variants(milligram, tablespoon),
        DensityUnit::MilligramPerTablespoon
    );
    assert_eq!(
        DensityUnit::from_variants(milligram, teaspoon),
        DensityUnit::MilligramPerTeaspoon
    );

    assert_eq!(
        DensityUnit::from_variants(microgram, liter),
        DensityUnit::MicrogramPerLiter
    );
    assert_eq!(
        DensityUnit::from_variants(microgram, milliliter),
        DensityUnit::MicrogramPerMilliliter
    );
    assert_eq!(
        DensityUnit::from_variants(microgram, pint),
        DensityUnit::MicrogramPerPint
    );
    assert_eq!(
        DensityUnit::from_variants(microgram, gallon),
        DensityUnit::MicrogramPerGallon
    );
    assert_eq!(
        DensityUnit::from_variants(microgram, fluid_ounce),
        DensityUnit::MicrogramPerFluidOunce
    );
    assert_eq!(
        DensityUnit::from_variants(microgram, tablespoon),
        DensityUnit::MicrogramPerTablespoon
    );
    assert_eq!(
        DensityUnit::from_variants(microgram, teaspoon),
        DensityUnit::MicrogramPerTeaspoon
    );

    assert_eq!(
        DensityUnit::from_variants(ounce, liter),
        DensityUnit::OuncePerLiter
    );
    assert_eq!(
        DensityUnit::from_variants(ounce, milliliter),
        DensityUnit::OuncePerMilliliter
    );
    assert_eq!(
        DensityUnit::from_variants(ounce, pint),
        DensityUnit::OuncePerPint
    );
    assert_eq!(
        DensityUnit::from_variants(ounce, gallon),
        DensityUnit::OuncePerGallon
    );
    assert_eq!(
        DensityUnit::from_variants(ounce, fluid_ounce),
        DensityUnit::OuncePerFluidOunce
    );
    assert_eq!(
        DensityUnit::from_variants(ounce, tablespoon),
        DensityUnit::OuncePerTablespoon
    );
    assert_eq!(
        DensityUnit::from_variants(ounce, teaspoon),
        DensityUnit::OuncePerTeaspoon
    );
}

#[test]
fn test_get_all_density_unit_enumerations() {
    let function_enumerations = DensityUnit::get_all_enumerations();
    let manual_enumerations = &vec![
        DensityUnit::GramPerLiter,
        DensityUnit::GramPerMilliliter,
        DensityUnit::GramPerPint,
        DensityUnit::GramPerGallon,
        DensityUnit::GramPerFluidOunce,
        DensityUnit::GramPerTablespoon,
        DensityUnit::GramPerTeaspoon,
        DensityUnit::KilogramPerLiter,
        DensityUnit::KilogramPerMilliliter,
        DensityUnit::KilogramPerPint,
        DensityUnit::KilogramPerGallon,
        DensityUnit::KilogramPerFluidOunce,
        DensityUnit::KilogramPerTablespoon,
        DensityUnit::KilogramPerTeaspoon,
        DensityUnit::MilligramPerLiter,
        DensityUnit::MilligramPerMilliliter,
        DensityUnit::MilligramPerPint,
        DensityUnit::MilligramPerGallon,
        DensityUnit::MilligramPerFluidOunce,
        DensityUnit::MilligramPerTablespoon,
        DensityUnit::MilligramPerTeaspoon,
        DensityUnit::MicrogramPerLiter,
        DensityUnit::MicrogramPerMilliliter,
        DensityUnit::MicrogramPerPint,
        DensityUnit::MicrogramPerGallon,
        DensityUnit::MicrogramPerFluidOunce,
        DensityUnit::MicrogramPerTablespoon,
        DensityUnit::MicrogramPerTeaspoon,
        DensityUnit::OuncePerLiter,
        DensityUnit::OuncePerMilliliter,
        DensityUnit::OuncePerPint,
        DensityUnit::OuncePerGallon,
        DensityUnit::OuncePerFluidOunce,
        DensityUnit::OuncePerTablespoon,
        DensityUnit::OuncePerTeaspoon,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_selected_density_unit_enumerations() {
    let function_enumerations = DensityUnit::get_selected_enumerations();
    let manual_enumerations = vec![
        &DensityUnit::GramPerMilliliter,
        &DensityUnit::MilligramPerMilliliter,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(DensityUnit::GramPerLiter.get_symbol(), "g/L");
    assert_eq!(DensityUnit::GramPerMilliliter.get_symbol(), "g/mL");
    assert_eq!(DensityUnit::GramPerPint.get_symbol(), "g/pt");
    assert_eq!(DensityUnit::GramPerGallon.get_symbol(), "g/gal");
    assert_eq!(DensityUnit::GramPerFluidOunce.get_symbol(), "g/fl oz");
    assert_eq!(DensityUnit::GramPerTablespoon.get_symbol(), "g/tbsp");
    assert_eq!(DensityUnit::GramPerTeaspoon.get_symbol(), "g/tsp");

    assert_eq!(DensityUnit::KilogramPerLiter.get_symbol(), "kg/L");
    assert_eq!(DensityUnit::KilogramPerMilliliter.get_symbol(), "kg/mL");
    assert_eq!(DensityUnit::KilogramPerPint.get_symbol(), "kg/pt");
    assert_eq!(DensityUnit::KilogramPerGallon.get_symbol(), "kg/gal");
    assert_eq!(DensityUnit::KilogramPerFluidOunce.get_symbol(), "kg/fl oz");
    assert_eq!(DensityUnit::KilogramPerTablespoon.get_symbol(), "kg/tbsp");
    assert_eq!(DensityUnit::KilogramPerTeaspoon.get_symbol(), "kg/tsp");

    assert_eq!(DensityUnit::MilligramPerLiter.get_symbol(), "mg/L");
    assert_eq!(DensityUnit::MilligramPerMilliliter.get_symbol(), "mg/mL");
    assert_eq!(DensityUnit::MilligramPerPint.get_symbol(), "mg/pt");
    assert_eq!(DensityUnit::MilligramPerGallon.get_symbol(), "mg/gal");
    assert_eq!(DensityUnit::MilligramPerFluidOunce.get_symbol(), "mg/fl oz");
    assert_eq!(DensityUnit::MilligramPerTablespoon.get_symbol(), "mg/tbsp");
    assert_eq!(DensityUnit::MilligramPerTeaspoon.get_symbol(), "mg/tsp");

    assert_eq!(DensityUnit::MicrogramPerLiter.get_symbol(), "ug/L");
    assert_eq!(DensityUnit::MicrogramPerMilliliter.get_symbol(), "ug/mL");
    assert_eq!(DensityUnit::MicrogramPerPint.get_symbol(), "ug/pt");
    assert_eq!(DensityUnit::MicrogramPerGallon.get_symbol(), "ug/gal");
    assert_eq!(DensityUnit::MicrogramPerFluidOunce.get_symbol(), "ug/fl oz");
    assert_eq!(DensityUnit::MicrogramPerTablespoon.get_symbol(), "ug/tbsp");
    assert_eq!(DensityUnit::MicrogramPerTeaspoon.get_symbol(), "ug/tsp");

    assert_eq!(DensityUnit::OuncePerLiter.get_symbol(), "oz/L");
    assert_eq!(DensityUnit::OuncePerMilliliter.get_symbol(), "oz/mL");
    assert_eq!(DensityUnit::OuncePerPint.get_symbol(), "oz/pt");
    assert_eq!(DensityUnit::OuncePerGallon.get_symbol(), "oz/gal");
    assert_eq!(DensityUnit::OuncePerFluidOunce.get_symbol(), "oz/fl oz");
    assert_eq!(DensityUnit::OuncePerTablespoon.get_symbol(), "oz/tbsp");
    assert_eq!(DensityUnit::OuncePerTeaspoon.get_symbol(), "oz/tsp");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(DensityUnit::GramPerLiter.get_unit_type(), "gram per liter");
    assert_eq!(
        DensityUnit::GramPerMilliliter.get_unit_type(),
        "gram per milliliter"
    );
    assert_eq!(DensityUnit::GramPerPint.get_unit_type(), "gram per pint");
    assert_eq!(DensityUnit::GramPerGallon.get_unit_type(), "gram per gallon");
    assert_eq!(
        DensityUnit::GramPerFluidOunce.get_unit_type(),
        "gram per fluid ounce"
    );
    assert_eq!(
        DensityUnit::GramPerTablespoon.get_unit_type(),
        "gram per tablespoon"
    );
    assert_eq!(
        DensityUnit::GramPerTeaspoon.get_unit_type(),
        "gram per teaspoon"
    );

    assert_eq!(
        DensityUnit::KilogramPerLiter.get_unit_type(),
        "kilogram per liter"
    );
    assert_eq!(
        DensityUnit::KilogramPerMilliliter.get_unit_type(),
        "kilogram per milliliter"
    );
    assert_eq!(
        DensityUnit::KilogramPerPint.get_unit_type(),
        "kilogram per pint"
    );
    assert_eq!(
        DensityUnit::KilogramPerGallon.get_unit_type(),
        "kilogram per gallon"
    );
    assert_eq!(
        DensityUnit::KilogramPerFluidOunce.get_unit_type(),
        "kilogram per fluid ounce"
    );
    assert_eq!(
        DensityUnit::KilogramPerTablespoon.get_unit_type(),
        "kilogram per tablespoon"
    );
    assert_eq!(
        DensityUnit::KilogramPerTeaspoon.get_unit_type(),
        "kilogram per teaspoon"
    );

    assert_eq!(
        DensityUnit::MilligramPerLiter.get_unit_type(),
        "milligram per liter"
    );
    assert_eq!(
        DensityUnit::MilligramPerMilliliter.get_unit_type(),
        "milligram per milliliter"
    );
    assert_eq!(
        DensityUnit::MilligramPerPint.get_unit_type(),
        "milligram per pint"
    );
    assert_eq!(
        DensityUnit::MilligramPerGallon.get_unit_type(),
        "milligram per gallon"
    );
    assert_eq!(
        DensityUnit::MilligramPerFluidOunce.get_unit_type(),
        "milligram per fluid ounce"
    );
    assert_eq!(
        DensityUnit::MilligramPerTablespoon.get_unit_type(),
        "milligram per tablespoon"
    );
    assert_eq!(
        DensityUnit::MilligramPerTeaspoon.get_unit_type(),
        "milligram per teaspoon"
    );

    assert_eq!(
        DensityUnit::MicrogramPerLiter.get_unit_type(),
        "microgram per liter"
    );
    assert_eq!(
        DensityUnit::MicrogramPerMilliliter.get_unit_type(),
        "microgram per milliliter"
    );
    assert_eq!(
        DensityUnit::MicrogramPerPint.get_unit_type(),
        "microgram per pint"
    );
    assert_eq!(
        DensityUnit::MicrogramPerGallon.get_unit_type(),
        "microgram per gallon"
    );
    assert_eq!(
        DensityUnit::MicrogramPerFluidOunce.get_unit_type(),
        "microgram per fluid ounce"
    );
    assert_eq!(
        DensityUnit::MicrogramPerTablespoon.get_unit_type(),
        "microgram per tablespoon"
    );
    assert_eq!(
        DensityUnit::MicrogramPerTeaspoon.get_unit_type(),
        "microgram per teaspoon"
    );

    assert_eq!(DensityUnit::OuncePerLiter.get_unit_type(), "ounce per liter");
    assert_eq!(
        DensityUnit::OuncePerMilliliter.get_unit_type(),
        "ounce per milliliter"
    );
    assert_eq!(DensityUnit::OuncePerPint.get_unit_type(), "ounce per pint");
    assert_eq!(
        DensityUnit::OuncePerGallon.get_unit_type(),
        "ounce per gallon"
    );
    assert_eq!(
        DensityUnit::OuncePerFluidOunce.get_unit_type(),
        "ounce per fluid ounce"
    );
    assert_eq!(
        DensityUnit::OuncePerTablespoon.get_unit_type(),
        "ounce per tablespoon"
    );
    assert_eq!(
        DensityUnit::OuncePerTeaspoon.get_unit_type(),
        "ounce per teaspoon"
    );
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(
        DensityUnit::GramPerLiter.get_unit_type_plural(),
        "grams per liter"
    );
    assert_eq!(
        DensityUnit::GramPerMilliliter.get_unit_type_plural(),
        "grams per milliliter"
    );
    assert_eq!(
        DensityUnit::GramPerPint.get_unit_type_plural(),
        "grams per pint"
    );
    assert_eq!(
        DensityUnit::GramPerGallon.get_unit_type_plural(),
        "grams per gallon"
    );
    assert_eq!(
        DensityUnit::GramPerFluidOunce.get_unit_type_plural(),
        "grams per fluid ounce"
    );
    assert_eq!(
        DensityUnit::GramPerTablespoon.get_unit_type_plural(),
        "grams per tablespoon"
    );
    assert_eq!(
        DensityUnit::GramPerTeaspoon.get_unit_type_plural(),
        "grams per teaspoon"
    );

    assert_eq!(
        DensityUnit::KilogramPerLiter.get_unit_type_plural(),
        "kilograms per liter"
    );
    assert_eq!(
        DensityUnit::KilogramPerMilliliter.get_unit_type_plural(),
        "kilograms per milliliter"
    );
    assert_eq!(
        DensityUnit::KilogramPerPint.get_unit_type_plural(),
        "kilograms per pint"
    );
    assert_eq!(
        DensityUnit::KilogramPerGallon.get_unit_type_plural(),
        "kilograms per gallon"
    );
    assert_eq!(
        DensityUnit::KilogramPerFluidOunce.get_unit_type_plural(),
        "kilograms per fluid ounce"
    );
    assert_eq!(
        DensityUnit::KilogramPerTablespoon.get_unit_type_plural(),
        "kilograms per tablespoon"
    );
    assert_eq!(
        DensityUnit::KilogramPerTeaspoon.get_unit_type_plural(),
        "kilograms per teaspoon"
    );

    assert_eq!(
        DensityUnit::MilligramPerLiter.get_unit_type_plural(),
        "milligrams per liter"
    );
    assert_eq!(
        DensityUnit::MilligramPerMilliliter.get_unit_type_plural(),
        "milligrams per milliliter"
    );
    assert_eq!(
        DensityUnit::MilligramPerPint.get_unit_type_plural(),
        "milligrams per pint"
    );
    assert_eq!(
        DensityUnit::MilligramPerGallon.get_unit_type_plural(),
        "milligrams per gallon"
    );
    assert_eq!(
        DensityUnit::MilligramPerFluidOunce.get_unit_type_plural(),
        "milligrams per fluid ounce"
    );
    assert_eq!(
        DensityUnit::MilligramPerTablespoon.get_unit_type_plural(),
        "milligrams per tablespoon"
    );
    assert_eq!(
        DensityUnit::MilligramPerTeaspoon.get_unit_type_plural(),
        "milligrams per teaspoon"
    );

    assert_eq!(
        DensityUnit::MicrogramPerLiter.get_unit_type_plural(),
        "micrograms per liter"
    );
    assert_eq!(
        DensityUnit::MicrogramPerMilliliter.get_unit_type_plural(),
        "micrograms per milliliter"
    );
    assert_eq!(
        DensityUnit::MicrogramPerPint.get_unit_type_plural(),
        "micrograms per pint"
    );
    assert_eq!(
        DensityUnit::MicrogramPerGallon.get_unit_type_plural(),
        "micrograms per gallon"
    );
    assert_eq!(
        DensityUnit::MicrogramPerFluidOunce.get_unit_type_plural(),
        "micrograms per fluid ounce"
    );
    assert_eq!(
        DensityUnit::MicrogramPerTablespoon.get_unit_type_plural(),
        "micrograms per tablespoon"
    );
    assert_eq!(
        DensityUnit::MicrogramPerTeaspoon.get_unit_type_plural(),
        "micrograms per teaspoon"
    );

    assert_eq!(
        DensityUnit::OuncePerLiter.get_unit_type_plural(),
        "ounces per liter"
    );
    assert_eq!(
        DensityUnit::OuncePerMilliliter.get_unit_type_plural(),
        "ounces per milliliter"
    );
    assert_eq!(
        DensityUnit::OuncePerPint.get_unit_type_plural(),
        "ounces per pint"
    );
    assert_eq!(
        DensityUnit::OuncePerGallon.get_unit_type_plural(),
        "ounces per gallon"
    );
    assert_eq!(
        DensityUnit::OuncePerFluidOunce.get_unit_type_plural(),
        "ounces per fluid ounce"
    );
    assert_eq!(
        DensityUnit::OuncePerTablespoon.get_unit_type_plural(),
        "ounces per tablespoon"
    );
    assert_eq!(
        DensityUnit::OuncePerTeaspoon.get_unit_type_plural(),
        "ounces per teaspoon"
    );
}

#[test]
fn test_get_measurement_system() {
    let metric_mass_measurement_system = MeasurementSystem::Metric;
    let metric_volume_measurement_system = MeasurementSystem::Metric;
    let imperial_mass_measurement_system = MeasurementSystem::Imperial;
    let imperial_volume_measurement_system = MeasurementSystem::Imperial;

    assert_eq!(
        DensityUnit::GramPerLiter.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::GramPerMilliliter.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::GramPerPint.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::GramPerGallon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::GramPerFluidOunce.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::GramPerTablespoon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::GramPerTeaspoon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );

    assert_eq!(
        DensityUnit::KilogramPerLiter.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::KilogramPerMilliliter.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::KilogramPerPint.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::KilogramPerGallon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::KilogramPerFluidOunce.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::KilogramPerTablespoon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::KilogramPerTeaspoon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );

    assert_eq!(
        DensityUnit::MilligramPerLiter.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MilligramPerMilliliter.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MilligramPerPint.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MilligramPerGallon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MilligramPerFluidOunce.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MilligramPerTablespoon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MilligramPerTeaspoon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );

    assert_eq!(
        DensityUnit::MicrogramPerLiter.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MicrogramPerMilliliter.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MicrogramPerPint.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MicrogramPerGallon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MicrogramPerFluidOunce.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MicrogramPerTablespoon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::MicrogramPerTeaspoon.get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );

    assert_eq!(
        DensityUnit::OuncePerLiter.get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::OuncePerMilliliter.get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::OuncePerPint.get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::OuncePerGallon.get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::OuncePerFluidOunce.get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::OuncePerTablespoon.get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityUnit::OuncePerTeaspoon.get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
}

#[test]
fn test_get_mass_variant() {
    assert_eq!(DensityUnit::GramPerLiter.get_mass_variant(), MassUnit::Gram,);
    assert_eq!(
        DensityUnit::KilogramPerPint.get_mass_variant(),
        MassUnit::Kilogram,
    );
    assert_eq!(
        DensityUnit::MilligramPerGallon.get_mass_variant(),
        MassUnit::Milligram,
    );
    assert_eq!(
        DensityUnit::MicrogramPerTeaspoon.get_mass_variant(),
        MassUnit::Microgram,
    );
    assert_eq!(
        DensityUnit::OuncePerFluidOunce.get_mass_variant(),
        MassUnit::Ounce,
    );
}

#[test]
fn test_get_volume_variant() {
    assert_eq!(
        DensityUnit::GramPerLiter.get_volume_variant(),
        VolumeUnit::Liter,
    );
    assert_eq!(
        DensityUnit::GramPerMilliliter.get_volume_variant(),
        VolumeUnit::Milliliter,
    );
    assert_eq!(
        DensityUnit::GramPerPint.get_volume_variant(),
        VolumeUnit::Pint,
    );
    assert_eq!(
        DensityUnit::GramPerGallon.get_volume_variant(),
        VolumeUnit::Gallon,
    );
    assert_eq!(
        DensityUnit::GramPerFluidOunce.get_volume_variant(),
        VolumeUnit::FluidOunce,
    );
    assert_eq!(
        DensityUnit::GramPerTeaspoon.get_volume_variant(),
        VolumeUnit::Teaspoon,
    );
    assert_eq!(
        DensityUnit::GramPerTablespoon.get_volume_variant(),
        VolumeUnit::Tablespoon,
    );
}

#[test]
fn test_get_si_factor() {
    let percentage_error = 0.5;

    assert!((DensityUnit::GramPerLiter.si_factor() - 1f64) / 1f64 < percentage_error);
    assert!((DensityUnit::GramPerMilliliter.si_factor() - 1000f64) / 1000f64 < percentage_error);
}

#[test]
fn test_from_str() {
    assert_eq!(
        DensityUnit::from_str("grams per liter").unwrap(),
        DensityUnit::GramPerLiter
    );
    assert_eq!(
        DensityUnit::from_str("gram per liter").unwrap(),
        DensityUnit::GramPerLiter
    );
    assert_eq!(
        DensityUnit::from_str("g/l").unwrap(),
        DensityUnit::GramPerLiter
    );
}
