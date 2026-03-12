use units::{
    density::{
        measurement_system::DensityMeasurementSystem, quantity::DensityQuantity, unit::DensityUnit,
    }, mass::{quantity::MassQuantity, unit::MassUnit}, measurement_system::MeasurementSystem, record::{GetFromDatabaseUsingId, Record}, volume::{quantity::VolumeQuantity, unit::VolumeUnit}
};
use utils::database::DatabaseService;

#[test]
fn test_density_from_variants() {
    let value = 5.82;

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
        DensityQuantity::from_variants(value, gram, liter),
        DensityQuantity::new(value, DensityUnit::GramPerLiter),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, gram, milliliter),
        DensityQuantity::new(value, DensityUnit::GramPerMilliliter),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, gram, pint),
        DensityQuantity::new(value, DensityUnit::GramPerPint),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, gram, gallon),
        DensityQuantity::new(value, DensityUnit::GramPerGallon),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, gram, fluid_ounce),
        DensityQuantity::new(value, DensityUnit::GramPerFluidOunce),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, gram, tablespoon),
        DensityQuantity::new(value, DensityUnit::GramPerTablespoon),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, gram, teaspoon),
        DensityQuantity::new(value, DensityUnit::GramPerTeaspoon),
    );

    assert_eq!(
        DensityQuantity::from_variants(value, kilogram, liter),
        DensityQuantity::new(value, DensityUnit::KilogramPerLiter),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, kilogram, milliliter),
        DensityQuantity::new(value, DensityUnit::KilogramPerMilliliter),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, kilogram, pint),
        DensityQuantity::new(value, DensityUnit::KilogramPerPint),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, kilogram, gallon),
        DensityQuantity::new(value, DensityUnit::KilogramPerGallon),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, kilogram, fluid_ounce),
        DensityQuantity::new(value, DensityUnit::KilogramPerFluidOunce),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, kilogram, tablespoon),
        DensityQuantity::new(value, DensityUnit::KilogramPerTablespoon),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, kilogram, teaspoon),
        DensityQuantity::new(value, DensityUnit::KilogramPerTeaspoon),
    );

    assert_eq!(
        DensityQuantity::from_variants(value, milligram, liter),
        DensityQuantity::new(value, DensityUnit::MilligramPerLiter),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, milligram, milliliter),
        DensityQuantity::new(value, DensityUnit::MilligramPerMilliliter),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, milligram, pint),
        DensityQuantity::new(value, DensityUnit::MilligramPerPint),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, milligram, gallon),
        DensityQuantity::new(value, DensityUnit::MilligramPerGallon),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, milligram, fluid_ounce),
        DensityQuantity::new(value, DensityUnit::MilligramPerFluidOunce),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, milligram, tablespoon),
        DensityQuantity::new(value, DensityUnit::MilligramPerTablespoon),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, milligram, teaspoon),
        DensityQuantity::new(value, DensityUnit::MilligramPerTeaspoon),
    );

    assert_eq!(
        DensityQuantity::from_variants(value, microgram, liter),
        DensityQuantity::new(value, DensityUnit::MicrogramPerLiter),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, microgram, milliliter),
        DensityQuantity::new(value, DensityUnit::MicrogramPerMilliliter),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, microgram, pint),
        DensityQuantity::new(value, DensityUnit::MicrogramPerPint),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, microgram, gallon),
        DensityQuantity::new(value, DensityUnit::MicrogramPerGallon),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, microgram, fluid_ounce),
        DensityQuantity::new(value, DensityUnit::MicrogramPerFluidOunce),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, microgram, tablespoon),
        DensityQuantity::new(value, DensityUnit::MicrogramPerTablespoon),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, microgram, teaspoon),
        DensityQuantity::new(value, DensityUnit::MicrogramPerTeaspoon),
    );

    assert_eq!(
        DensityQuantity::from_variants(value, ounce, liter),
        DensityQuantity::new(value, DensityUnit::OuncePerLiter),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, ounce, milliliter),
        DensityQuantity::new(value, DensityUnit::OuncePerMilliliter),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, ounce, pint),
        DensityQuantity::new(value, DensityUnit::OuncePerPint),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, ounce, gallon),
        DensityQuantity::new(value, DensityUnit::OuncePerGallon),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, ounce, fluid_ounce),
        DensityQuantity::new(value, DensityUnit::OuncePerFluidOunce),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, ounce, tablespoon),
        DensityQuantity::new(value, DensityUnit::OuncePerTablespoon),
    );
    assert_eq!(
        DensityQuantity::from_variants(value, ounce, teaspoon),
        DensityQuantity::new(value, DensityUnit::OuncePerTeaspoon),
    );
}

#[test]
fn test_density_new() {
    let value = 10 as f64;

    let density_new_g_per_l = DensityQuantity::new(value, DensityUnit::GramPerLiter);
    let density_from_g_per_l = DensityQuantity::from_g_per_l(value);
    assert_eq!(density_new_g_per_l, density_from_g_per_l);

    let density_new_kg_per_ml = DensityQuantity::new(value, DensityUnit::KilogramPerMilliliter);
    let density_from_kg_per_ml = DensityQuantity::from_kg_per_ml(value);
    assert_eq!(density_new_kg_per_ml, density_from_kg_per_ml);

    let density_new_mg_per_pt = DensityQuantity::new(value, DensityUnit::MilligramPerPint);
    let density_from_mg_per_pt = DensityQuantity::from_mg_per_pt(value);
    assert_eq!(density_new_mg_per_pt, density_from_mg_per_pt);

    let density_new_ug_per_gal = DensityQuantity::new(value, DensityUnit::MicrogramPerGallon);
    let density_from_ug_per_gal = DensityQuantity::from_ug_per_gal(value);
    assert_eq!(density_new_ug_per_gal, density_from_ug_per_gal);

    let density_new_oz_per_tsp = DensityQuantity::new(value, DensityUnit::OuncePerTeaspoon);
    let density_from_oz_per_tsp = DensityQuantity::from_oz_per_tsp(value);
    assert_eq!(density_new_oz_per_tsp, density_from_oz_per_tsp);
}

#[test]
fn test_density_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20472986;

    let mut density_new = DensityQuantity::new(value, DensityUnit::KilogramPerPint);
    let density_rounded = density_new.round(5);
    let density_manual = DensityQuantity::new(5.68033, DensityUnit::KilogramPerPint);
    assert_eq!(density_rounded, density_manual);

    let mut density_new_2 = DensityQuantity::new(value_2, DensityUnit::OuncePerPint);
    let density_rounded_2 = density_new_2.round(5);
    let density_coded_2 = DensityQuantity::new(147.20473, DensityUnit::OuncePerPint);
    assert_eq!(density_rounded_2, density_coded_2);
}

#[test]
fn test_density_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let density_kg_per_l = DensityQuantity::from_kg_per_l(value);
    let density_kg_per_ml = DensityQuantity::from_kg_per_ml(value);
    let density_g_per_teaspoon = DensityQuantity::from_g_per_tsp(value);

    assert!(
        (density_kg_per_l.as_kg_per_ml() - value / 1000f64).abs() / density_kg_per_l.as_kg_per_ml()
            < percentage_err
    );
    assert!(
        (density_kg_per_l.as_g_per_tsp() - value * 5.91939).abs() / density_kg_per_l.as_g_per_tsp()
            < percentage_err
    );
    assert!(
        (density_kg_per_ml.as_g_per_tsp() - value * 5919.39).abs()
            / density_kg_per_ml.as_g_per_tsp()
            < percentage_err
    );
    assert!(
        (density_g_per_teaspoon.as_kg_per_ml() - value / 5919.39).abs()
            / density_g_per_teaspoon.as_kg_per_ml()
            < percentage_err
    );
}

#[test]
fn test_density_to_unit() {
    let value = 6.9;
    let new_value = value / 1e6f64;

    let density_g_per_l = DensityQuantity::from_g_per_l(value);
    let density_kg_per_ml = DensityQuantity::from_kg_per_ml(new_value);
    let density_g_per_l_to_kg_per_ml = density_g_per_l.to_unit(DensityUnit::KilogramPerMilliliter);
    assert_eq!(density_kg_per_ml, density_g_per_l_to_kg_per_ml);
}

#[test]
fn test_density_to_fn() {
    let value = 6.9;
    let new_value = value / 1e6f64;

    let density_g_per_l = DensityQuantity::from_g_per_l(value);
    let density_kg_per_ml = DensityQuantity::from_kg_per_ml(new_value);
    let density_g_per_l_to_kg_per_ml = density_g_per_l.to_kg_per_ml();
    assert_eq!(density_kg_per_ml, density_g_per_l_to_kg_per_ml);
}

#[test]
fn test_density_is_zero() {
    let zero_density = DensityQuantity::from_oz_per_tsp(0f64);
    let density = DensityQuantity::from_oz_per_tsp(5.5);

    assert!(zero_density.is_zero());
    assert!(!density.is_zero());
}

#[test]
fn test_density_is_negative() {
    let negative_density = DensityQuantity::from_oz_per_tsp(-5.5f64);
    let density = DensityQuantity::from_oz_per_tsp(5.5);

    assert!(negative_density.is_negative());
    assert!(!density.is_negative());
}

#[test]
fn test_density_get_value() {
    let density = DensityQuantity::new(6.882, DensityUnit::KilogramPerFluidOunce);
    assert_eq!(density.get_value(), 6.882);
}

#[test]
fn test_density_set_value() {
    let mut density = DensityQuantity::new(6.882, DensityUnit::MicrogramPerGallon);
    assert_eq!(density.get_value(), 6.882);
    density.set_value(8.92);
    assert_eq!(density.get_value(), 8.92);
}

#[test]
fn test_density_get_unit() {
    let density = DensityQuantity::new(6.882, DensityUnit::KilogramPerFluidOunce);
    assert_eq!(density.get_unit(), DensityUnit::KilogramPerFluidOunce);
}

#[test]
fn test_density_set_unit() {
    let mut density = DensityQuantity::new(6.882, DensityUnit::MicrogramPerGallon);
    assert_eq!(density.get_unit(), DensityUnit::MicrogramPerGallon);
    density.set_unit(DensityUnit::OuncePerTeaspoon);
    assert_eq!(density.get_unit(), DensityUnit::OuncePerTeaspoon);
}

#[test]
fn test_density_get_symbol() {
    let value = 4.86;

    assert_eq!(DensityQuantity::from_g_per_l(value).get_symbol(), "g/L",);
    assert_eq!(DensityQuantity::from_g_per_ml(value).get_symbol(), "g/mL",);
    assert_eq!(DensityQuantity::from_g_per_pt(value).get_symbol(), "g/pt",);
    assert_eq!(DensityQuantity::from_g_per_gal(value).get_symbol(), "g/gal",);
    assert_eq!(
        DensityQuantity::from_g_per_floz(value).get_symbol(),
        "g/fl oz",
    );
    assert_eq!(
        DensityQuantity::from_g_per_tbsp(value).get_symbol(),
        "g/tbsp",
    );
    assert_eq!(DensityQuantity::from_g_per_tsp(value).get_symbol(), "g/tsp",);

    assert_eq!(DensityQuantity::from_kg_per_l(value).get_symbol(), "kg/L",);
    assert_eq!(DensityQuantity::from_kg_per_ml(value).get_symbol(), "kg/mL",);
    assert_eq!(DensityQuantity::from_kg_per_pt(value).get_symbol(), "kg/pt",);
    assert_eq!(
        DensityQuantity::from_kg_per_gal(value).get_symbol(),
        "kg/gal",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_floz(value).get_symbol(),
        "kg/fl oz",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_tbsp(value).get_symbol(),
        "kg/tbsp",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_tsp(value).get_symbol(),
        "kg/tsp",
    );

    assert_eq!(DensityQuantity::from_mg_per_l(value).get_symbol(), "mg/L",);
    assert_eq!(DensityQuantity::from_mg_per_ml(value).get_symbol(), "mg/mL",);
    assert_eq!(DensityQuantity::from_mg_per_pt(value).get_symbol(), "mg/pt",);
    assert_eq!(
        DensityQuantity::from_mg_per_gal(value).get_symbol(),
        "mg/gal",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_floz(value).get_symbol(),
        "mg/fl oz",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_tbsp(value).get_symbol(),
        "mg/tbsp",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_tsp(value).get_symbol(),
        "mg/tsp",
    );

    assert_eq!(DensityQuantity::from_ug_per_l(value).get_symbol(), "ug/L",);
    assert_eq!(DensityQuantity::from_ug_per_ml(value).get_symbol(), "ug/mL",);
    assert_eq!(DensityQuantity::from_ug_per_pt(value).get_symbol(), "ug/pt",);
    assert_eq!(
        DensityQuantity::from_ug_per_gal(value).get_symbol(),
        "ug/gal",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_floz(value).get_symbol(),
        "ug/fl oz",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_tbsp(value).get_symbol(),
        "ug/tbsp",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_tsp(value).get_symbol(),
        "ug/tsp",
    );

    assert_eq!(DensityQuantity::from_oz_per_l(value).get_symbol(), "oz/L",);
    assert_eq!(DensityQuantity::from_oz_per_ml(value).get_symbol(), "oz/mL",);
    assert_eq!(DensityQuantity::from_oz_per_pt(value).get_symbol(), "oz/pt",);
    assert_eq!(
        DensityQuantity::from_oz_per_gal(value).get_symbol(),
        "oz/gal",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_floz(value).get_symbol(),
        "oz/fl oz",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_tbsp(value).get_symbol(),
        "oz/tbsp",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_tsp(value).get_symbol(),
        "oz/tsp",
    );
}

#[test]
fn test_density_measurement_system() {
    let value = 4.86;
    let metric_mass_measurement_system = MeasurementSystem::Metric;
    let metric_volume_measurement_system = MeasurementSystem::Metric;
    let imperial_mass_measurement_system = MeasurementSystem::Imperial;
    let imperial_volume_measurement_system = MeasurementSystem::Imperial;

    assert_eq!(
        DensityQuantity::from_g_per_l(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_g_per_ml(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_g_per_pt(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_g_per_gal(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_g_per_floz(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_g_per_tbsp(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_g_per_tsp(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );

    assert_eq!(
        DensityQuantity::from_kg_per_l(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_kg_per_ml(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_kg_per_pt(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_kg_per_gal(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_kg_per_floz(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_kg_per_tbsp(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_kg_per_tsp(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );

    assert_eq!(
        DensityQuantity::from_mg_per_l(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_mg_per_ml(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_mg_per_pt(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_mg_per_gal(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_mg_per_floz(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_mg_per_tbsp(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_mg_per_tsp(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );

    assert_eq!(
        DensityQuantity::from_ug_per_l(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_ug_per_ml(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_ug_per_pt(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_ug_per_gal(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_ug_per_floz(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_ug_per_tbsp(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_ug_per_tsp(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            metric_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );

    assert_eq!(
        DensityQuantity::from_oz_per_l(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_oz_per_ml(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            metric_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_oz_per_pt(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_oz_per_gal(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_oz_per_floz(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_oz_per_tbsp(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
    assert_eq!(
        DensityQuantity::from_oz_per_tsp(value).get_measurement_system(),
        DensityMeasurementSystem::new(
            imperial_mass_measurement_system,
            imperial_volume_measurement_system
        ),
    );
}

#[test]
fn test_density_get_unit_type() {
    let value = 4.86;

    assert_eq!(
        DensityQuantity::from_g_per_l(value).get_unit_type(),
        "gram per liter",
    );
    assert_eq!(
        DensityQuantity::from_g_per_ml(value).get_unit_type(),
        "gram per milliliter",
    );
    assert_eq!(
        DensityQuantity::from_g_per_pt(value).get_unit_type(),
        "gram per pint",
    );
    assert_eq!(
        DensityQuantity::from_g_per_gal(value).get_unit_type(),
        "gram per gallon",
    );
    assert_eq!(
        DensityQuantity::from_g_per_floz(value).get_unit_type(),
        "gram per fluid ounce",
    );
    assert_eq!(
        DensityQuantity::from_g_per_tbsp(value).get_unit_type(),
        "gram per tablespoon",
    );
    assert_eq!(
        DensityQuantity::from_g_per_tsp(value).get_unit_type(),
        "gram per teaspoon",
    );

    assert_eq!(
        DensityQuantity::from_kg_per_l(value).get_unit_type(),
        "kilogram per liter",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_ml(value).get_unit_type(),
        "kilogram per milliliter",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_pt(value).get_unit_type(),
        "kilogram per pint",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_gal(value).get_unit_type(),
        "kilogram per gallon",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_floz(value).get_unit_type(),
        "kilogram per fluid ounce",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_tbsp(value).get_unit_type(),
        "kilogram per tablespoon",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_tsp(value).get_unit_type(),
        "kilogram per teaspoon",
    );

    assert_eq!(
        DensityQuantity::from_mg_per_l(value).get_unit_type(),
        "milligram per liter",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_ml(value).get_unit_type(),
        "milligram per milliliter",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_pt(value).get_unit_type(),
        "milligram per pint",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_gal(value).get_unit_type(),
        "milligram per gallon",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_floz(value).get_unit_type(),
        "milligram per fluid ounce",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_tbsp(value).get_unit_type(),
        "milligram per tablespoon",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_tsp(value).get_unit_type(),
        "milligram per teaspoon",
    );

    assert_eq!(
        DensityQuantity::from_ug_per_l(value).get_unit_type(),
        "microgram per liter",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_ml(value).get_unit_type(),
        "microgram per milliliter",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_pt(value).get_unit_type(),
        "microgram per pint",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_gal(value).get_unit_type(),
        "microgram per gallon",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_floz(value).get_unit_type(),
        "microgram per fluid ounce",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_tbsp(value).get_unit_type(),
        "microgram per tablespoon",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_tsp(value).get_unit_type(),
        "microgram per teaspoon",
    );

    assert_eq!(
        DensityQuantity::from_oz_per_l(value).get_unit_type(),
        "ounce per liter",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_ml(value).get_unit_type(),
        "ounce per milliliter",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_pt(value).get_unit_type(),
        "ounce per pint",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_gal(value).get_unit_type(),
        "ounce per gallon",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_floz(value).get_unit_type(),
        "ounce per fluid ounce",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_tbsp(value).get_unit_type(),
        "ounce per tablespoon",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_tsp(value).get_unit_type(),
        "ounce per teaspoon",
    );
}

#[test]
fn test_density_get_unit_type_plural() {
    let value = 4.86;

    assert_eq!(
        DensityQuantity::from_g_per_l(value).get_unit_type_plural(),
        "grams per liter",
    );
    assert_eq!(
        DensityQuantity::from_g_per_ml(value).get_unit_type_plural(),
        "grams per milliliter",
    );
    assert_eq!(
        DensityQuantity::from_g_per_pt(value).get_unit_type_plural(),
        "grams per pint",
    );
    assert_eq!(
        DensityQuantity::from_g_per_gal(value).get_unit_type_plural(),
        "grams per gallon",
    );
    assert_eq!(
        DensityQuantity::from_g_per_floz(value).get_unit_type_plural(),
        "grams per fluid ounce",
    );
    assert_eq!(
        DensityQuantity::from_g_per_tbsp(value).get_unit_type_plural(),
        "grams per tablespoon",
    );
    assert_eq!(
        DensityQuantity::from_g_per_tsp(value).get_unit_type_plural(),
        "grams per teaspoon",
    );

    assert_eq!(
        DensityQuantity::from_kg_per_l(value).get_unit_type_plural(),
        "kilograms per liter",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_ml(value).get_unit_type_plural(),
        "kilograms per milliliter",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_pt(value).get_unit_type_plural(),
        "kilograms per pint",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_gal(value).get_unit_type_plural(),
        "kilograms per gallon",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_floz(value).get_unit_type_plural(),
        "kilograms per fluid ounce",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_tbsp(value).get_unit_type_plural(),
        "kilograms per tablespoon",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_tsp(value).get_unit_type_plural(),
        "kilograms per teaspoon",
    );

    assert_eq!(
        DensityQuantity::from_mg_per_l(value).get_unit_type_plural(),
        "milligrams per liter",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_ml(value).get_unit_type_plural(),
        "milligrams per milliliter",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_pt(value).get_unit_type_plural(),
        "milligrams per pint",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_gal(value).get_unit_type_plural(),
        "milligrams per gallon",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_floz(value).get_unit_type_plural(),
        "milligrams per fluid ounce",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_tbsp(value).get_unit_type_plural(),
        "milligrams per tablespoon",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_tsp(value).get_unit_type_plural(),
        "milligrams per teaspoon",
    );

    assert_eq!(
        DensityQuantity::from_ug_per_l(value).get_unit_type_plural(),
        "micrograms per liter",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_ml(value).get_unit_type_plural(),
        "micrograms per milliliter",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_pt(value).get_unit_type_plural(),
        "micrograms per pint",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_gal(value).get_unit_type_plural(),
        "micrograms per gallon",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_floz(value).get_unit_type_plural(),
        "micrograms per fluid ounce",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_tbsp(value).get_unit_type_plural(),
        "micrograms per tablespoon",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_tsp(value).get_unit_type_plural(),
        "micrograms per teaspoon",
    );

    assert_eq!(
        DensityQuantity::from_oz_per_l(value).get_unit_type_plural(),
        "ounces per liter",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_ml(value).get_unit_type_plural(),
        "ounces per milliliter",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_pt(value).get_unit_type_plural(),
        "ounces per pint",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_gal(value).get_unit_type_plural(),
        "ounces per gallon",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_floz(value).get_unit_type_plural(),
        "ounces per fluid ounce",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_tbsp(value).get_unit_type_plural(),
        "ounces per tablespoon",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_tsp(value).get_unit_type_plural(),
        "ounces per teaspoon",
    );
}

#[test]
fn test_density_to_string() {
    let value = 4.86;

    assert_eq!(DensityQuantity::from_g_per_l(value).to_string(), "4.86g/L",);
    assert_eq!(
        DensityQuantity::from_g_per_ml(value).to_string(),
        "4.86g/mL",
    );
    assert_eq!(
        DensityQuantity::from_g_per_pt(value).to_string(),
        "4.86g/pt",
    );
    assert_eq!(
        DensityQuantity::from_g_per_gal(value).to_string(),
        "4.86g/gal",
    );
    assert_eq!(
        DensityQuantity::from_g_per_floz(value).to_string(),
        "4.86g/fl oz",
    );
    assert_eq!(
        DensityQuantity::from_g_per_tbsp(value).to_string(),
        "4.86g/tbsp",
    );
    assert_eq!(
        DensityQuantity::from_g_per_tsp(value).to_string(),
        "4.86g/tsp",
    );

    assert_eq!(
        DensityQuantity::from_kg_per_l(value).to_string(),
        "4.86kg/L",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_ml(value).to_string(),
        "4.86kg/mL",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_pt(value).to_string(),
        "4.86kg/pt",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_gal(value).to_string(),
        "4.86kg/gal",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_floz(value).to_string(),
        "4.86kg/fl oz",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_tbsp(value).to_string(),
        "4.86kg/tbsp",
    );
    assert_eq!(
        DensityQuantity::from_kg_per_tsp(value).to_string(),
        "4.86kg/tsp",
    );

    assert_eq!(
        DensityQuantity::from_mg_per_l(value).to_string(),
        "4.86mg/L",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_ml(value).to_string(),
        "4.86mg/mL",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_pt(value).to_string(),
        "4.86mg/pt",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_gal(value).to_string(),
        "4.86mg/gal",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_floz(value).to_string(),
        "4.86mg/fl oz",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_tbsp(value).to_string(),
        "4.86mg/tbsp",
    );
    assert_eq!(
        DensityQuantity::from_mg_per_tsp(value).to_string(),
        "4.86mg/tsp",
    );

    assert_eq!(
        DensityQuantity::from_ug_per_l(value).to_string(),
        "4.86ug/L",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_ml(value).to_string(),
        "4.86ug/mL",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_pt(value).to_string(),
        "4.86ug/pt",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_gal(value).to_string(),
        "4.86ug/gal",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_floz(value).to_string(),
        "4.86ug/fl oz",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_tbsp(value).to_string(),
        "4.86ug/tbsp",
    );
    assert_eq!(
        DensityQuantity::from_ug_per_tsp(value).to_string(),
        "4.86ug/tsp",
    );

    assert_eq!(
        DensityQuantity::from_oz_per_l(value).to_string(),
        "4.86oz/L",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_ml(value).to_string(),
        "4.86oz/mL",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_pt(value).to_string(),
        "4.86oz/pt",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_gal(value).to_string(),
        "4.86oz/gal",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_floz(value).to_string(),
        "4.86oz/fl oz",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_tbsp(value).to_string(),
        "4.86oz/tbsp",
    );
    assert_eq!(
        DensityQuantity::from_oz_per_tsp(value).to_string(),
        "4.86oz/tsp",
    );
}

#[test]
fn test_density_add() {
    let density_1 = DensityQuantity::from_g_per_ml(5.5);
    let density_2 = DensityQuantity::from_kg_per_l(5.5);
    let density_total = DensityQuantity::from_g_per_ml(11f64);
    assert_eq!(density_1 + density_2, density_total);
}

#[test]
fn test_density_subtract() {
    let density_1 = DensityQuantity::from_g_per_ml(8.5);
    let density_2 = DensityQuantity::from_kg_per_l(5.5);
    let density_total = DensityQuantity::from_g_per_ml(3f64);
    assert_eq!((density_1 - density_2).round(5), density_total);
}

#[test]
fn test_density_mul_f64() {
    let density_1 = DensityQuantity::from_kg_per_l(70f64);
    let density_2 = DensityQuantity::from_kg_per_l(350f64);
    let density_3 = DensityQuantity::from_kg_per_l(267.4f64);

    assert_eq!(density_1 * 5, density_2);
    assert_eq!(density_1 * 3.82, density_3);
}

#[test]
fn test_density_mul_by_volume() {
    let density = DensityQuantity::from_kg_per_l(70f64);
    let volume = VolumeQuantity::from_ml(2000f64);
    let mass = MassQuantity::from_kg(140f64);
    assert_eq!((density * volume), mass);
    assert_eq!((volume * density), mass);
}

#[test]
fn test_density_div_f64() {
    let density_1 = DensityQuantity::from_kg_per_l(350f64);
    let density_2 = DensityQuantity::from_kg_per_l(70f64);

    assert_eq!(density_1 / 5, density_2);
}

#[test]
fn test_density_div_mass_by_volume() {
    let mass = MassQuantity::from_kg(140f64);
    let volume = VolumeQuantity::from_l(2f64);
    let density = DensityQuantity::from_kg_per_l(70f64);
    assert_eq!(mass / volume, density);
}

#[test]
fn test_density_sum() {
    let density_1 = DensityQuantity::from_kg_per_l(30f64);
    let density_2 = DensityQuantity::from_g_per_ml(20f64);
    let density_3 = DensityQuantity::from_kg_per_l(50f64).to_oz_per_pt();
    let density_4 = DensityQuantity::from_kg_per_l(20f64).to_oz_per_tsp();
    let density_5 = DensityQuantity::from_kg_per_l(130f64).to_ug_per_pt();
    let density_total = DensityQuantity::from_kg_per_l(250f64);

    let densities = vec![density_1, density_2, density_3, density_4, density_5];

    let mut sum: DensityQuantity = densities.iter().map(|density| *density * 2).sum();
    assert_eq!(sum.get_unit(), density_5.get_unit());
    assert_eq!(
        sum.round(5),
        (density_total * 2).to_unit(density_5.get_unit()).round(5)
    );
}

#[test]
fn test_density_partial_order() {
    let density_1 = DensityQuantity::from_kg_per_l(50f64);
    let density_2 = DensityQuantity::from_g_per_ml(60f64);
    let density_3 = DensityQuantity::from_kg_per_l(70f64);
    assert!(density_1 < density_2);
    assert!(density_1 < density_3);
    assert!(density_2 < density_3);
}

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();

    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;

    let density_oz_per_tsp = DensityQuantity::from_oz_per_tsp(6700f64);
    let density_record = Record::new(density_oz_per_tsp);

    let res = density_record.save_to_database(&pool).await;
    assert!(res.is_ok());

    let density_saved =
        DensityQuantity::get_from_database_using_id(density_record.get_uuid(), &pool).await;
    assert!(density_saved.is_ok());
    assert_eq!(density_saved.unwrap(), density_record);

    let res = density_record.delete_from_database_using_id(&pool).await;
    assert!(res.is_ok());

    let density_saved_2 =
        DensityQuantity::get_from_database_using_id(density_record.get_uuid(), &pool).await;
    assert!(density_saved_2.is_err());
}

