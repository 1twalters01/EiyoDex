use std::collections::BTreeSet;
use units::volume::VolumeUnit;

#[test]
fn test_get_volume_unit_enumerations() {
    let function_enumerations = VolumeUnit::get_enumerations();
    let manual_enumerations = vec![
        VolumeUnit::Liter,
        VolumeUnit::Milliliter,
        VolumeUnit::Pint,
        VolumeUnit::Gallon,
        VolumeUnit::FluidOunce,
        VolumeUnit::Teaspoon,
        VolumeUnit::Tablespoon,
        VolumeUnit::FakeLiter,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

// #[test]
// fn test_get_symbols() {
//     assert_eq!(VolumeUnit::Liter.as_symbol(), "L");
//     assert_eq!(VolumeUnit::Milliliter.as_symbol(), "mL");
//     assert_eq!(VolumeUnit::Pint.as_symbol(), "pt");
//     assert_eq!(VolumeUnit::Gallon.as_symbol(), "gal");
//     assert_eq!(VolumeUnit::FluidOunce.as_symbol(), "fl oz");
//     assert_eq!(VolumeUnit::Teaspoon.as_symbol(), "tsp");
//     assert_eq!(VolumeUnit::Tablespoon.as_symbol(), "tbsp");
//
//     assert_eq!(VolumeUnit::FakeLiter.as_symbol(), "fL");
// }

#[test]
fn test_get_unit_types() {
    assert_eq!(VolumeUnit::Liter.as_unit_type(), "liter");
    assert_eq!(VolumeUnit::Milliliter.as_unit_type(), "milliliter");
    assert_eq!(VolumeUnit::Pint.as_unit_type(), "pint");
    assert_eq!(VolumeUnit::Gallon.as_unit_type(), "gallon");
    assert_eq!(VolumeUnit::FluidOunce.as_unit_type(), "fluid ounce");
    assert_eq!(VolumeUnit::Tablespoon.as_unit_type(), "tablespoon");
    assert_eq!(VolumeUnit::Teaspoon.as_unit_type(), "teaspoon");

    assert_eq!(VolumeUnit::FakeLiter.as_unit_type(), "fakeliter");
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(VolumeUnit::Liter.as_unit_type_plural(), "liters");
    assert_eq!(VolumeUnit::Milliliter.as_unit_type_plural(), "milliliters");
    assert_eq!(VolumeUnit::Pint.as_unit_type_plural(), "pints");
    assert_eq!(VolumeUnit::Gallon.as_unit_type_plural(), "gallons");
    assert_eq!(VolumeUnit::FluidOunce.as_unit_type_plural(), "fluid ounces");
    assert_eq!(VolumeUnit::Tablespoon.as_unit_type_plural(), "tablespoons");
    assert_eq!(VolumeUnit::Teaspoon.as_unit_type_plural(), "teaspoons");

    assert_eq!(VolumeUnit::FakeLiter.as_unit_type_plural(), "fakeliters");
}

#[test]
fn test_get_liters_factor() {
    assert_eq!(VolumeUnit::Liter.liters_factor(), 1 as f64);
    assert_eq!(VolumeUnit::Milliliter.liters_factor(), 0.001);
    assert_eq!(VolumeUnit::Pint.liters_factor(), 0.5682612);
    assert_eq!(VolumeUnit::Gallon.liters_factor(), 4.54609);
    assert_eq!(VolumeUnit::FluidOunce.liters_factor(), 0.02841306);
    assert_eq!(VolumeUnit::Tablespoon.liters_factor(), 0.01775816);
    assert_eq!(VolumeUnit::Teaspoon.liters_factor(), 0.005919387);
}

#[test]
fn test_from_str() {
    assert_eq!(VolumeUnit::from_str("l").unwrap(), VolumeUnit::Liter);
    assert_eq!(VolumeUnit::from_str("L").unwrap(), VolumeUnit::Liter);
    assert_eq!(VolumeUnit::from_str("liter").unwrap(), VolumeUnit::Liter);
    assert_eq!(VolumeUnit::from_str("Liter").unwrap(), VolumeUnit::Liter);
    assert_eq!(VolumeUnit::from_str("liters").unwrap(), VolumeUnit::Liter);
    assert_eq!(VolumeUnit::from_str("lITeRs").unwrap(), VolumeUnit::Liter);
    assert_ne!(VolumeUnit::from_str("ml").unwrap(), VolumeUnit::Liter);

    assert_eq!(VolumeUnit::from_str("ml").unwrap(), VolumeUnit::Milliliter);
    assert_eq!(
        VolumeUnit::from_str("milliliter").unwrap(),
        VolumeUnit::Milliliter
    );
    assert_eq!(
        VolumeUnit::from_str("milliliters").unwrap(),
        VolumeUnit::Milliliter
    );

    assert_eq!(VolumeUnit::from_str("pt").unwrap(), VolumeUnit::Pint);
    assert_eq!(VolumeUnit::from_str("pint").unwrap(), VolumeUnit::Pint);
    assert_eq!(VolumeUnit::from_str("pints").unwrap(), VolumeUnit::Pint);

    assert_eq!(VolumeUnit::from_str("gal").unwrap(), VolumeUnit::Gallon);
    assert_eq!(VolumeUnit::from_str("gallon").unwrap(), VolumeUnit::Gallon);
    assert_eq!(VolumeUnit::from_str("gallons").unwrap(), VolumeUnit::Gallon);

    assert_eq!(
        VolumeUnit::from_str("fl oz").unwrap(),
        VolumeUnit::FluidOunce
    );
    assert_eq!(
        VolumeUnit::from_str("floz").unwrap(),
        VolumeUnit::FluidOunce
    );
    assert_eq!(
        VolumeUnit::from_str("fluid ounce").unwrap(),
        VolumeUnit::FluidOunce
    );
    assert_eq!(
        VolumeUnit::from_str("fluid ounces").unwrap(),
        VolumeUnit::FluidOunce
    );

    assert_eq!(
        VolumeUnit::from_str("tbsp").unwrap(),
        VolumeUnit::Tablespoon
    );
    assert_eq!(
        VolumeUnit::from_str("tablespoon").unwrap(),
        VolumeUnit::Tablespoon
    );
    assert_eq!(
        VolumeUnit::from_str("tablespoons").unwrap(),
        VolumeUnit::Tablespoon
    );

    assert_eq!(VolumeUnit::from_str("tsp").unwrap(), VolumeUnit::Teaspoon);
    assert_eq!(
        VolumeUnit::from_str("teaspoon").unwrap(),
        VolumeUnit::Teaspoon
    );
    assert_eq!(
        VolumeUnit::from_str("teaspoons").unwrap(),
        VolumeUnit::Teaspoon
    );
}
