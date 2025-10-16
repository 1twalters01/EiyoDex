use units::{
    density::{Density, DensityUnit},
    mass::MassUnit,
    measurement_system::MeasurementSystem,
    volume::VolumeUnit,
};

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
        Density::from_variants(value, gram, liter),
        Density::new(value, DensityUnit::GramPerLiter),
    );
    assert_eq!(
        Density::from_variants(value, gram, milliliter),
        Density::new(value, DensityUnit::GramPerMilliliter),
    );
    assert_eq!(
        Density::from_variants(value, gram, pint),
        Density::new(value, DensityUnit::GramPerPint),
    );
    assert_eq!(
        Density::from_variants(value, gram, gallon),
        Density::new(value, DensityUnit::GramPerGallon),
    );
    assert_eq!(
        Density::from_variants(value, gram, fluid_ounce),
        Density::new(value, DensityUnit::GramPerFluidOunce),
    );
    assert_eq!(
        Density::from_variants(value, gram, tablespoon),
        Density::new(value, DensityUnit::GramPerTablespoon),
    );
    assert_eq!(
        Density::from_variants(value, gram, teaspoon),
        Density::new(value, DensityUnit::GramPerTeaspoon),
    );

    assert_eq!(
        Density::from_variants(value, kilogram, liter),
        Density::new(value, DensityUnit::KilogramPerLiter),
    );
    assert_eq!(
        Density::from_variants(value, kilogram, milliliter),
        Density::new(value, DensityUnit::KilogramPerMilliliter),
    );
    assert_eq!(
        Density::from_variants(value, kilogram, pint),
        Density::new(value, DensityUnit::KilogramPerPint),
    );
    assert_eq!(
        Density::from_variants(value, kilogram, gallon),
        Density::new(value, DensityUnit::KilogramPerGallon),
    );
    assert_eq!(
        Density::from_variants(value, kilogram, fluid_ounce),
        Density::new(value, DensityUnit::KilogramPerFluidOunce),
    );
    assert_eq!(
        Density::from_variants(value, kilogram, tablespoon),
        Density::new(value, DensityUnit::KilogramPerTablespoon),
    );
    assert_eq!(
        Density::from_variants(value, kilogram, teaspoon),
        Density::new(value, DensityUnit::KilogramPerTeaspoon),
    );

    assert_eq!(
        Density::from_variants(value, milligram, liter),
        Density::new(value, DensityUnit::MilligramPerLiter),
    );
    assert_eq!(
        Density::from_variants(value, milligram, milliliter),
        Density::new(value, DensityUnit::MilligramPerMilliliter),
    );
    assert_eq!(
        Density::from_variants(value, milligram, pint),
        Density::new(value, DensityUnit::MilligramPerPint),
    );
    assert_eq!(
        Density::from_variants(value, milligram, gallon),
        Density::new(value, DensityUnit::MilligramPerGallon),
    );
    assert_eq!(
        Density::from_variants(value, milligram, fluid_ounce),
        Density::new(value, DensityUnit::MilligramPerFluidOunce),
    );
    assert_eq!(
        Density::from_variants(value, milligram, tablespoon),
        Density::new(value, DensityUnit::MilligramPerTablespoon),
    );
    assert_eq!(
        Density::from_variants(value, milligram, teaspoon),
        Density::new(value, DensityUnit::MilligramPerTeaspoon),
    );

    assert_eq!(
        Density::from_variants(value, microgram, liter),
        Density::new(value, DensityUnit::MicrogramPerLiter),
    );
    assert_eq!(
        Density::from_variants(value, microgram, milliliter),
        Density::new(value, DensityUnit::MicrogramPerMilliliter),
    );
    assert_eq!(
        Density::from_variants(value, microgram, pint),
        Density::new(value, DensityUnit::MicrogramPerPint),
    );
    assert_eq!(
        Density::from_variants(value, microgram, gallon),
        Density::new(value, DensityUnit::MicrogramPerGallon),
    );
    assert_eq!(
        Density::from_variants(value, microgram, fluid_ounce),
        Density::new(value, DensityUnit::MicrogramPerFluidOunce),
    );
    assert_eq!(
        Density::from_variants(value, microgram, tablespoon),
        Density::new(value, DensityUnit::MicrogramPerTablespoon),
    );
    assert_eq!(
        Density::from_variants(value, microgram, teaspoon),
        Density::new(value, DensityUnit::MicrogramPerTeaspoon),
    );

    assert_eq!(
        Density::from_variants(value, ounce, liter),
        Density::new(value, DensityUnit::OuncePerLiter),
    );
    assert_eq!(
        Density::from_variants(value, ounce, milliliter),
        Density::new(value, DensityUnit::OuncePerMilliliter),
    );
    assert_eq!(
        Density::from_variants(value, ounce, pint),
        Density::new(value, DensityUnit::OuncePerPint),
    );
    assert_eq!(
        Density::from_variants(value, ounce, gallon),
        Density::new(value, DensityUnit::OuncePerGallon),
    );
    assert_eq!(
        Density::from_variants(value, ounce, fluid_ounce),
        Density::new(value, DensityUnit::OuncePerFluidOunce),
    );
    assert_eq!(
        Density::from_variants(value, ounce, tablespoon),
        Density::new(value, DensityUnit::OuncePerTablespoon),
    );
    assert_eq!(
        Density::from_variants(value, ounce, teaspoon),
        Density::new(value, DensityUnit::OuncePerTeaspoon),
    );
}

#[test]
fn test_density_new() {
    let value = 10 as f64;

    let density_new_g_per_l = Density::new(value, DensityUnit::GramPerLiter);
    let density_from_g_per_l = Density::from_g_per_l(value);
    assert_eq!(density_new_g_per_l, density_from_g_per_l);

    let density_new_kg_per_ml = Density::new(value, DensityUnit::KilogramPerMilliliter);
    let density_from_kg_per_ml = Density::from_kg_per_ml(value);
    assert_eq!(density_new_kg_per_ml, density_from_kg_per_ml);

    let density_new_mg_per_pt = Density::new(value, DensityUnit::MilligramPerPint);
    let density_from_mg_per_pt = Density::from_mg_per_pt(value);
    assert_eq!(density_new_mg_per_pt, density_from_mg_per_pt);

    let density_new_ug_per_gal = Density::new(value, DensityUnit::MicrogramPerGallon);
    let density_from_ug_per_gal = Density::from_ug_per_gal(value);
    assert_eq!(density_new_ug_per_gal, density_from_ug_per_gal);

    let density_new_oz_per_tsp = Density::new(value, DensityUnit::OuncePerTeaspoon);
    let density_from_oz_per_tsp = Density::from_oz_per_tsp(value);
    assert_eq!(density_new_oz_per_tsp, density_from_oz_per_tsp);
}

#[test]
fn test_density_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20472986;

    let mut density_new = Density::new(value, DensityUnit::KilogramPerPint);
    let density_rounded = density_new.round(5);
    let density_manual = Density::new(5.68033, DensityUnit::KilogramPerPint);
    assert_eq!(density_rounded, density_manual);

    let mut density_new_2 = Density::new(value_2, DensityUnit::OuncePerPint);
    let density_rounded_2 = density_new_2.round(5);
    let density_coded_2 = Density::new(147.20473, DensityUnit::OuncePerPint);
    assert_eq!(density_rounded_2, density_coded_2);
}

#[test]
fn test_density_as_fn() {}

#[test]
fn test_density_to_unit() {
    let value = 6.9;
    let new_value = value / 1e6f64;

    let density_g_per_l = Density::from_g_per_l(value);
    let density_kg_per_ml = Density::from_kg_per_ml(new_value);
    let density_g_per_l_to_kg_per_ml = density_g_per_l.to_unit(DensityUnit::KilogramPerMilliliter);
    assert_eq!(density_kg_per_ml, density_g_per_l_to_kg_per_ml);
}

#[test]
fn test_density_to_fn() {
    let value = 6.9;
    let new_value = value / 1e6f64;

    let density_g_per_l = Density::from_g_per_l(value);
    let density_kg_per_ml = Density::from_kg_per_ml(new_value);
    let density_g_per_l_to_kg_per_ml = density_g_per_l.to_kg_per_ml();
    assert_eq!(density_kg_per_ml, density_g_per_l_to_kg_per_ml);
}

#[test]
fn test_density_is_zero() {
    let zero_density = Density::from_oz_per_tsp(0f64);
    let density = Density::from_oz_per_tsp(5.5);

    assert!(zero_density.is_zero());
    assert!(!density.is_zero());
}

#[test]
fn test_density_is_negative() {
    let negative_density = Density::from_oz_per_tsp(-5.5f64);
    let density = Density::from_oz_per_tsp(5.5);

    assert!(negative_density.is_negative());
    assert!(!density.is_negative());
}

#[test]
fn test_density_get_value() {
    let density = Density::new(6.882, DensityUnit::KilogramPerFluidOunce);
    assert_eq!(density.get_value(), 6.882);
}

#[test]
fn test_density_set_value() {
    let mut density = Density::new(6.882, DensityUnit::MicrogramPerGallon);
    assert_eq!(density.get_value(), 6.882);
    density.set_value(8.92);
    assert_eq!(density.get_value(), 8.92);
}

#[test]
fn test_density_get_unit() {
    let density = Density::new(6.882, DensityUnit::KilogramPerFluidOunce);
    assert_eq!(density.get_unit(), DensityUnit::KilogramPerFluidOunce);
}

#[test]
fn test_density_set_unit() {
    let mut density = Density::new(6.882, DensityUnit::MicrogramPerGallon);
    assert_eq!(density.get_unit(), DensityUnit::MicrogramPerGallon);
    density.set_unit(DensityUnit::OuncePerTeaspoon);
    assert_eq!(density.get_unit(), DensityUnit::OuncePerTeaspoon);
}

#[test]
fn test_density_get_symbol() {}

#[test]
fn test_density_measurement_system() {}

#[test]
fn test_density_get_unit_type() {}

#[test]
fn test_density_get_unit_type_plural() {}

#[test]
fn test_density_to_string() {}

#[test]
fn test_density_add() {}

#[test]
fn test_density_subtract() {}

#[test]
fn test_density_mul_f64() {}

#[test]
fn test_power_mul_by_volume() {}

#[test]
fn test_power_div_f64() {}

#[test]
fn test_power_div_mass_by_volume() {}

#[test]
fn test_power_sum() {}

#[test]
fn test_power_partial_order() {}
