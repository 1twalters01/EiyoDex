use units::{
    measurement_system::MeasurementSystem,
    volume::{Volume, VolumeUnit},
};

#[test]
fn test_new_volume() {
    let value = 10 as f64;

    let liter_new_l = Volume::new(value, VolumeUnit::Liter);
    let liter_from_l = Volume::from_l(value);
    assert_eq!(liter_new_l, liter_from_l);

    let liter_new_ml = Volume::new(value, VolumeUnit::Milliliter);
    let liter_from_ml = Volume::from_ml(value);
    assert_eq!(liter_new_ml, liter_from_ml);

    let liter_new_pt = Volume::new(value, VolumeUnit::Pint);
    let liter_from_pt = Volume::from_pt(value);
    assert_eq!(liter_new_pt, liter_from_pt);

    let liter_new_gal = Volume::new(value, VolumeUnit::Gallon);
    let liter_from_gal = Volume::from_gal(value);
    assert_eq!(liter_new_gal, liter_from_gal);

    let liter_new_tbsp = Volume::new(value, VolumeUnit::Tablespoon);
    let liter_from_tbsp = Volume::from_tbsp(value);
    assert_eq!(liter_new_tbsp, liter_from_tbsp);

    let liter_new_tsp = Volume::new(value, VolumeUnit::Teaspoon);
    let liter_from_tsp = Volume::from_tsp(value);
    assert_eq!(liter_new_tsp, liter_from_tsp);
}

#[test]
fn test_volume_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20473186;

    let mut liter_new = Volume::new(value, VolumeUnit::Liter);
    let liter_rounded = liter_new.round(5);
    let liter_coded = Volume::new(5.68033, VolumeUnit::Liter);
    assert_eq!(liter_rounded, liter_coded);

    let mut liter_new_2 = Volume::new(value_2, VolumeUnit::Liter);
    let liter_rounded_2 = liter_new_2.round(5);
    let liter_coded_2 = Volume::new(147.20473, VolumeUnit::Liter);
    assert_eq!(liter_rounded_2, liter_coded_2);
}

#[test]
fn test_volume_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let volume_l = Volume::from_l(value);
    let volume_ml = Volume::from_ml(value);
    let volume_pt = Volume::from_pt(value);
    let volume_gal = Volume::from_gal(value);
    let volume_tbsp = Volume::from_tbsp(value);
    let volume_tsp = Volume::from_tsp(value);

    // percentage error calculations
    assert!((volume_l.as_ml() - value * 1_000 as f64).abs() / volume_l.as_l() < percentage_err);
    assert!((volume_l.as_pt() - value * 0.001).abs() / volume_l.as_pt() < percentage_err);
    assert!((volume_l.as_gal() - value * 1e6).abs() / volume_l.as_gal() < percentage_err);
    assert!((volume_l.as_tbsp() - value * 0.0352739).abs() / volume_l.as_tbsp() < percentage_err);
    assert!((volume_l.as_tsp() - value * 0.0352739).abs() / volume_l.as_tsp() < percentage_err);

    assert!((volume_ml.as_l() - value * 0.001 as f64).abs() / volume_ml.as_l() < percentage_err);
    assert!((volume_ml.as_pt() - value * 1e-6).abs() / volume_ml.as_pt() < percentage_err);
    assert!(
        (volume_ml.as_gal() - value * 1_000 as f64).abs() / volume_ml.as_gal() < percentage_err
    );
    assert!(
        (volume_ml.as_tbsp() - value * 3.527396e-5).abs() / volume_ml.as_tbsp() < percentage_err
    );
    assert!((volume_ml.as_tsp() - value * 3.527396e-5).abs() / volume_ml.as_tsp() < percentage_err);

    assert!((volume_pt.as_l() - value * 1000 as f64).abs() / volume_pt.as_l() < percentage_err);
    assert!((volume_pt.as_ml() - value * 1e6).abs() / volume_pt.as_ml() < percentage_err);
    assert!((volume_pt.as_gal() - value * 1e9).abs() / volume_pt.as_gal() < percentage_err);
    assert!((volume_pt.as_tbsp() - value * 35.27396).abs() / volume_pt.as_tbsp() < percentage_err);
    assert!((volume_pt.as_tsp() - value * 35.27396).abs() / volume_pt.as_tsp() < percentage_err);

    assert!((volume_gal.as_l() - value * 1e-6).abs() / volume_gal.as_l() < percentage_err);
    assert!((volume_gal.as_ml() - value * 0.001).abs() / volume_gal.as_ml() < percentage_err);
    assert!((volume_gal.as_pt() - value * 1e-9).abs() / volume_gal.as_pt() < percentage_err);
    assert!(
        (volume_gal.as_tbsp() - value * 3.527396e-8).abs() / volume_gal.as_tbsp() < percentage_err
    );
    assert!(
        (volume_gal.as_tsp() - value * 3.527396e-8).abs() / volume_gal.as_tsp() < percentage_err
    );

    assert!((volume_tbsp.as_l() - value * 28.34952).abs() / volume_tbsp.as_l() < percentage_err);
    assert!((volume_tbsp.as_ml() - value * 28349.52).abs() / volume_tbsp.as_ml() < percentage_err);
    assert!(
        (volume_tbsp.as_pt() - value * 0.02834952).abs() / volume_tbsp.as_pt() < percentage_err
    );
    assert!(
        (volume_tbsp.as_gal() - value * 2.834952e+7).abs() / volume_tbsp.as_gal() < percentage_err
    );
    assert!(
        (volume_tbsp.as_tsp() - value * 2.834952e+7).abs() / volume_tbsp.as_tsp() < percentage_err
    );

    assert!((volume_tsp.as_l() - value * 28.34952).abs() / volume_tsp.as_l() < percentage_err);
    assert!((volume_tsp.as_ml() - value * 28349.52).abs() / volume_tsp.as_ml() < percentage_err);
    assert!((volume_tsp.as_pt() - value * 0.02834952).abs() / volume_tsp.as_pt() < percentage_err);
    assert!(
        (volume_tsp.as_gal() - value * 2.834952e+7).abs() / volume_tsp.as_gal() < percentage_err
    );
    assert!(
        (volume_tsp.as_tbsp() - value * 2.834952e+7).abs() / volume_tsp.as_tbsp() < percentage_err
    );
}

#[test]
fn test_volume_to_unit() {
    let value = 5.6;
    let new_value = value / 0.5682612;

    let mass_l = Volume::from_l(value);
    let mass_pt = Volume::from_pt(new_value);
    let mass_l_to_pt = mass_l.to_unit(VolumeUnit::Pint);

    print!("mass_ounces1: {},\nmass_ounces2: {}", mass_pt, mass_l_to_pt);
    assert_eq!(mass_pt, mass_l_to_pt);
}

#[test]
fn test_volume_to_fn() {
    let value = 6.9;
    let new_value = value / 0.5682612;

    let mass_l = Volume::from_l(value);
    let mass_pt = Volume::from_pt(new_value);
    let mass_l_to_pt = mass_l.to_pt();

    print!(
        "volume_pints 1: {},\nvolume_pints 2: {}",
        mass_pt, mass_l_to_pt
    );
    assert_eq!(mass_pt, mass_l_to_pt);
}

#[test]
fn test_volume_is_zero() {
    let zero_volume = Volume::from_l(0f64);
    let volume = Volume::from_l(5.5);

    assert!(zero_volume.is_zero());
    assert!(!volume.is_zero());
}

#[test]
fn test_volume_is_negative() {
    let negative_volume = Volume::from_l(-5.5);
    let volume = Volume::from_l(5.5);

    assert!(negative_volume.is_negative());
    assert!(!volume.is_negative());
}

#[test]
fn test_volume_get_value() {
    let volume = Volume::new(6.882, VolumeUnit::Milliliter);
    assert_eq!(volume.get_value(), 6.882);
}

#[test]
fn test_volume_set_value() {
    let mut volume = Volume::new(6.882, VolumeUnit::Milliliter);
    volume.set_value(8.92);
    assert_eq!(volume.get_value(), 8.92);
}

#[test]
fn test_volume_get_unit() {
    let volume = Volume::new(6.882, VolumeUnit::Milliliter);
    assert_eq!(volume.get_unit(), VolumeUnit::Milliliter);
}

#[test]
fn test_volume_set_unit() {
    let mut volume = Volume::new(6.882, VolumeUnit::Milliliter);
    volume.set_unit(VolumeUnit::Pint);
    assert_eq!(volume.get_unit(), VolumeUnit::Pint);
}

#[test]
fn test_volume_get_symbol() {
    let value = 4.2;
    let volume_l = Volume::from_l(value);
    let volume_ml = Volume::from_ml(value);
    let volume_pt = Volume::from_pt(value);
    let volume_gal = Volume::from_gal(value);
    let volume_tbsp = Volume::from_tbsp(value);
    let volume_tsp = Volume::from_tsp(value);

    assert_eq!(volume_l.get_symbol(), "L");
    assert_eq!(volume_ml.get_symbol(), "mL");
    assert_eq!(volume_pt.get_symbol(), "pt");
    assert_eq!(volume_gal.get_symbol(), "gal");
    assert_eq!(volume_tbsp.get_symbol(), "tbsp");
    assert_eq!(volume_tsp.get_symbol(), "tsp");
}

#[test]
fn test_mass_get_measurement_system() {
    let value = 4.2;

    let volume_l = Volume::from_l(value);
    let volume_ml = Volume::from_ml(value);
    let volume_pt = Volume::from_pt(value);
    let volume_gal = Volume::from_gal(value);
    let volume_tbsp = Volume::from_tbsp(value);
    let volume_tsp = Volume::from_tsp(value);

    assert_eq!(volume_l.get_measurement_system(), MeasurementSystem::Metric);
    assert_eq!(
        volume_ml.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        volume_pt.get_measurement_system(),
        MeasurementSystem::Imperial
    );
    assert_eq!(
        volume_gal.get_measurement_system(),
        MeasurementSystem::Imperial
    );
    assert_eq!(
        volume_tbsp.get_measurement_system(),
        MeasurementSystem::Imperial
    );
    assert_eq!(
        volume_tsp.get_measurement_system(),
        MeasurementSystem::Imperial
    );
}

#[test]
fn test_volume_get_unit_type() {
    let value = 4.2;
    let volume_l = Volume::from_l(value);
    let volume_ml = Volume::from_ml(value);
    let volume_pt = Volume::from_pt(value);
    let volume_gal = Volume::from_gal(value);
    let volume_tbsp = Volume::from_tbsp(value);
    let volume_tsp = Volume::from_tsp(value);

    assert_eq!(volume_l.get_unit_type(), "liter");
    assert_eq!(volume_ml.get_unit_type(), "milliliter");
    assert_eq!(volume_pt.get_unit_type(), "pint");
    assert_eq!(volume_gal.get_unit_type(), "gallon");
    assert_eq!(volume_tbsp.get_unit_type(), "tablespoon");
    assert_eq!(volume_tsp.get_unit_type(), "teaspoon");
}

#[test]
fn test_volume_get_unit_type_plural() {
    let value = 4.2;
    let volume_l = Volume::from_l(value);
    let volume_ml = Volume::from_ml(value);
    let volume_pt = Volume::from_pt(value);
    let volume_gal = Volume::from_gal(value);
    let volume_tbsp = Volume::from_tbsp(value);
    let volume_tsp = Volume::from_tsp(value);

    assert_eq!(volume_l.get_unit_type_plural(), "liters");
    assert_eq!(volume_ml.get_unit_type_plural(), "milliliters");
    assert_eq!(volume_pt.get_unit_type_plural(), "pints");
    assert_eq!(volume_gal.get_unit_type_plural(), "gallons");
    assert_eq!(volume_tbsp.get_unit_type_plural(), "tablespoons");
    assert_eq!(volume_tsp.get_unit_type_plural(), "teaspoons");
}

#[test]
fn test_volume_to_string() {
    let value = 4.2;
    let volume_l = Volume::from_l(value);
    let volume_ml = Volume::from_ml(value);
    let volume_pt = Volume::from_pt(value);
    let volume_gal = Volume::from_gal(value);
    let volume_tbsp = Volume::from_tbsp(value);
    let volume_tsp = Volume::from_tsp(value);

    assert_eq!(volume_l.to_string(), "4.2L");
    assert_eq!(volume_ml.to_string(), "4.2mL");
    assert_eq!(volume_pt.to_string(), "4.2pt");
    assert_eq!(volume_gal.to_string(), "4.2gal");
    assert_eq!(volume_tbsp.to_string(), "4.2tbsp");
    assert_eq!(volume_tsp.to_string(), "4.2tsp");
}

#[test]
fn test_volume_add() {
    let volume_ml_1 = Volume::from_ml(100f64);
    let volume_ml_2 = Volume::from_ml(500f64);
    let volume_l = Volume::from_l(2f64);

    let volume_ml_1_plus_ml_2 = Volume::from_ml(600f64);
    let volume_l_plus_ml_1 = Volume::from_l(2.1);
    let volume_ml_2_plus_l = Volume::from_ml(2500f64);

    assert_eq!(volume_ml_1 + volume_ml_2, volume_ml_1_plus_ml_2);
    assert_eq!(volume_l + volume_ml_1, volume_l_plus_ml_1);
    assert_eq!(volume_ml_2 + volume_l, volume_ml_2_plus_l);
}

#[test]
fn test_volume_subtract() {
    let volume_ml_1 = Volume::from_ml(6700f64);
    let volume_ml_2 = Volume::from_ml(4700f64);
    let volume_l = Volume::from_l(1.2f64);

    let volume_ml_1_minus_ml_2 = Volume::from_ml(2000f64);
    let volume_l_minus_ml_1 = Volume::from_l(-5.5);
    let volume_ml_2_minus_l = Volume::from_ml(3500f64);

    assert_eq!(volume_ml_1 - volume_ml_2, volume_ml_1_minus_ml_2);
    assert_eq!(volume_l - volume_ml_1, volume_l_minus_ml_1);
    assert_eq!(volume_ml_2 - volume_l, volume_ml_2_minus_l);
}

#[test]
fn test_volume_multiply() {
    let volume_l_1 = Volume::from_l(70f64);
    let volume_l_2 = Volume::from_l(350f64);
    let liter_l_3 = Volume::from_l(267.4f64);

    assert_eq!(volume_l_1 * 5, volume_l_2);
    assert_eq!(volume_l_1 * 3.82, liter_l_3);
}

#[test]
fn test_volume_divide() {
    let volume_l_1 = Volume::from_l(350f64);
    let volume_l_2 = Volume::from_l(70f64);

    assert_eq!(volume_l_1 / 5, volume_l_2);
}

#[test]
fn test_volume_partial_order() {
    let volume_ml_1 = Volume::from_ml(6700f64);
    let volume_ml_2 = Volume::from_ml(4700f64);
    let volume_l = Volume::from_l(5.2f64);
    assert!(volume_ml_1 > volume_ml_2);
    assert!(volume_ml_1 > volume_l);
    assert!(volume_l > volume_ml_2);
}
