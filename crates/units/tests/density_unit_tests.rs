use std::{collections::BTreeSet, str::FromStr};
use units::{density::DensityUnit, mass::MassUnit, volume::VolumeUnit};

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
fn test_get_all_density_unit_enumerations() {}

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
fn test_get_symbols() {}

#[test]
fn test_get_unit_types() {}

#[test]
fn test_get_plural_unit_types() {}

#[test]
fn test_get_measurement_system() {}

#[test]
fn test_get_mass_variant() {}

#[test]
fn test_get_volume_variant() {}

#[test]
fn test_get_si_factor() {}

#[test]
fn test_from_str() {}
