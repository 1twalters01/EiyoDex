use units::{
    measurement_system::MeasurementSystem,
    volume::{quantity::VolumeQuantity, unit::VolumeUnit},
};

#[test]
fn test_new_volume() {
    let value = 10 as f64;

    let liter_new_l = VolumeQuantity::new(value, VolumeUnit::Liter);
    let liter_from_l = VolumeQuantity::from_l(value);
    assert_eq!(liter_new_l, liter_from_l);

    let liter_new_ml = VolumeQuantity::new(value, VolumeUnit::Milliliter);
    let liter_from_ml = VolumeQuantity::from_ml(value);
    assert_eq!(liter_new_ml, liter_from_ml);

    let liter_new_pt = VolumeQuantity::new(value, VolumeUnit::Pint);
    let liter_from_pt = VolumeQuantity::from_pt(value);
    assert_eq!(liter_new_pt, liter_from_pt);

    let liter_new_gal = VolumeQuantity::new(value, VolumeUnit::Gallon);
    let liter_from_gal = VolumeQuantity::from_gal(value);
    assert_eq!(liter_new_gal, liter_from_gal);

    let liter_new_tbsp = VolumeQuantity::new(value, VolumeUnit::Tablespoon);
    let liter_from_tbsp = VolumeQuantity::from_tbsp(value);
    assert_eq!(liter_new_tbsp, liter_from_tbsp);

    let liter_new_tsp = VolumeQuantity::new(value, VolumeUnit::Teaspoon);
    let liter_from_tsp = VolumeQuantity::from_tsp(value);
    assert_eq!(liter_new_tsp, liter_from_tsp);
}

#[test]
fn test_volume_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20473186;

    let mut liter_new = VolumeQuantity::new(value, VolumeUnit::Liter);
    let liter_rounded = liter_new.round(5);
    let liter_coded = VolumeQuantity::new(5.68033, VolumeUnit::Liter);
    assert_eq!(liter_rounded, liter_coded);

    let mut liter_new_2 = VolumeQuantity::new(value_2, VolumeUnit::Liter);
    let liter_rounded_2 = liter_new_2.round(5);
    let liter_coded_2 = VolumeQuantity::new(147.20473, VolumeUnit::Liter);
    assert_eq!(liter_rounded_2, liter_coded_2);
}

#[test]
fn test_volume_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let volume_l = VolumeQuantity::from_l(value);
    let volume_ml = VolumeQuantity::from_ml(value);
    let volume_pt = VolumeQuantity::from_pt(value);
    let volume_gal = VolumeQuantity::from_gal(value);
    let volume_tbsp = VolumeQuantity::from_tbsp(value);
    let volume_tsp = VolumeQuantity::from_tsp(value);

    // percentage error calculations
    assert!((volume_l.as_ml() - value * 1_000 as f64).abs() / volume_l.as_l() < percentage_err);
    assert!((volume_l.as_pt() - value * 1.759754).abs() / volume_l.as_pt() < percentage_err);
    assert!((volume_l.as_gal() - value * 0.2199693).abs() / volume_l.as_gal() < percentage_err);
    assert!((volume_l.as_tbsp() - value * 56.31213).abs() / volume_l.as_tbsp() < percentage_err);
    assert!((volume_l.as_tsp() - value * 168.9364).abs() / volume_l.as_tsp() < percentage_err);

    assert!((volume_ml.as_l() - value * 1e-3).abs() / volume_ml.as_l() < percentage_err);
    assert!((volume_ml.as_pt() - value * 0.001759754).abs() / volume_ml.as_pt() < percentage_err);
    assert!(
        (volume_ml.as_gal() - value * 0.0002199693).abs() / volume_ml.as_gal() < percentage_err
    );
    assert!(
        (volume_ml.as_tbsp() - value * 0.05631213).abs() / volume_ml.as_tbsp() < percentage_err
    );
    assert!((volume_ml.as_tsp() - value * 0.1689364).abs() / volume_ml.as_tsp() < percentage_err);

    assert!((volume_pt.as_l() - value * 0.5682612).abs() / volume_pt.as_l() < percentage_err);
    assert!((volume_pt.as_ml() - value * 568.2612).abs() / volume_pt.as_ml() < percentage_err);
    assert!((volume_pt.as_gal() - value * 0.125).abs() / volume_pt.as_gal() < percentage_err);
    assert!((volume_pt.as_tbsp() - value * 32f64).abs() / volume_pt.as_tbsp() < percentage_err);
    assert!((volume_pt.as_tsp() - value * 96f64).abs() / volume_pt.as_tsp() < percentage_err);

    assert!((volume_gal.as_l() - value * 4.54609).abs() / volume_gal.as_l() < percentage_err);
    assert!((volume_gal.as_ml() - value * 4546.09).abs() / volume_gal.as_ml() < percentage_err);
    assert!((volume_gal.as_pt() - value * 8f64).abs() / volume_gal.as_pt() < percentage_err);
    assert!((volume_gal.as_tbsp() - value * 256f64).abs() / volume_gal.as_tbsp() < percentage_err);
    assert!((volume_gal.as_tsp() - value * 786f64).abs() / volume_gal.as_tsp() < percentage_err);

    assert!((volume_tbsp.as_l() - value * 0.01775816).abs() / volume_tbsp.as_l() < percentage_err);
    assert!((volume_tbsp.as_ml() - value * 17.75816).abs() / volume_tbsp.as_ml() < percentage_err);
    assert!((volume_tbsp.as_pt() - value * 0.03125).abs() / volume_tbsp.as_pt() < percentage_err);
    assert!(
        (volume_tbsp.as_gal() - value * 0.00390625).abs() / volume_tbsp.as_gal() < percentage_err
    );
    assert!((volume_tbsp.as_tsp() - value * 3f64).abs() / volume_tbsp.as_tsp() < percentage_err);

    assert!((volume_tsp.as_l() - value * 0.005919387).abs() / volume_tsp.as_l() < percentage_err);
    assert!((volume_tsp.as_ml() - value * 5.919388).abs() / volume_tsp.as_ml() < percentage_err);
    assert!((volume_tsp.as_pt() - value * 0.01041667).abs() / volume_tsp.as_pt() < percentage_err);
    assert!(
        (volume_tsp.as_gal() - value * 0.001302083).abs() / volume_tsp.as_gal() < percentage_err
    );
    assert!(
        (volume_tsp.as_tbsp() - value * 0.3333333).abs() / volume_tsp.as_tbsp() < percentage_err
    );
}

#[test]
fn test_volume_to_unit() {
    let value = 5.6;
    let new_value = value / 0.5682612;

    let mass_l = VolumeQuantity::from_l(value);
    let mass_pt = VolumeQuantity::from_pt(new_value).round(5);
    let mass_l_to_pt = mass_l.to_unit(VolumeUnit::Pint).round(5);

    print!("mass_ounces1: {},\nmass_ounces2: {}", mass_pt, mass_l_to_pt);
    assert_eq!(mass_pt, mass_l_to_pt);
}

#[test]
fn test_volume_to_fn() {
    let value = 6.9;
    let new_value = value / 0.5682612;

    let mass_l = VolumeQuantity::from_l(value);
    let mass_pt = VolumeQuantity::from_pt(new_value).round(6);
    let mass_l_to_pt = mass_l.to_pt().round(6);

    print!(
        "volume_pints 1: {},\nvolume_pints 2: {}",
        mass_pt, mass_l_to_pt
    );
    assert_eq!(mass_pt, mass_l_to_pt);
}

#[test]
fn test_volume_is_zero() {
    let zero_volume = VolumeQuantity::from_l(0f64);
    let volume = VolumeQuantity::from_l(5.5);

    assert!(zero_volume.is_zero());
    assert!(!volume.is_zero());
}

#[test]
fn test_volume_is_negative() {
    let negative_volume = VolumeQuantity::from_l(-5.5);
    let volume = VolumeQuantity::from_l(5.5);

    assert!(negative_volume.is_negative());
    assert!(!volume.is_negative());
}

#[test]
fn test_volume_get_value() {
    let volume = VolumeQuantity::new(6.882, VolumeUnit::Milliliter);
    assert_eq!(volume.get_value(), 6.882);
}

#[test]
fn test_volume_set_value() {
    let mut volume = VolumeQuantity::new(6.882, VolumeUnit::Milliliter);
    volume.set_value(8.92);
    assert_eq!(volume.get_value(), 8.92);
}

#[test]
fn test_volume_get_unit() {
    let volume = VolumeQuantity::new(6.882, VolumeUnit::Milliliter);
    assert_eq!(volume.get_unit(), VolumeUnit::Milliliter);
}

#[test]
fn test_volume_set_unit() {
    let mut volume = VolumeQuantity::new(6.882, VolumeUnit::Milliliter);
    volume.set_unit(VolumeUnit::Pint);
    assert_eq!(volume.get_unit(), VolumeUnit::Pint);
}

#[test]
fn test_volume_get_symbol() {
    let value = 4.2;
    let volume_l = VolumeQuantity::from_l(value);
    let volume_ml = VolumeQuantity::from_ml(value);
    let volume_pt = VolumeQuantity::from_pt(value);
    let volume_gal = VolumeQuantity::from_gal(value);
    let volume_tbsp = VolumeQuantity::from_tbsp(value);
    let volume_tsp = VolumeQuantity::from_tsp(value);

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

    let volume_l = VolumeQuantity::from_l(value);
    let volume_ml = VolumeQuantity::from_ml(value);
    let volume_pt = VolumeQuantity::from_pt(value);
    let volume_gal = VolumeQuantity::from_gal(value);
    let volume_tbsp = VolumeQuantity::from_tbsp(value);
    let volume_tsp = VolumeQuantity::from_tsp(value);

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
    let volume_l = VolumeQuantity::from_l(value);
    let volume_ml = VolumeQuantity::from_ml(value);
    let volume_pt = VolumeQuantity::from_pt(value);
    let volume_gal = VolumeQuantity::from_gal(value);
    let volume_tbsp = VolumeQuantity::from_tbsp(value);
    let volume_tsp = VolumeQuantity::from_tsp(value);

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
    let volume_l = VolumeQuantity::from_l(value);
    let volume_ml = VolumeQuantity::from_ml(value);
    let volume_pt = VolumeQuantity::from_pt(value);
    let volume_gal = VolumeQuantity::from_gal(value);
    let volume_tbsp = VolumeQuantity::from_tbsp(value);
    let volume_tsp = VolumeQuantity::from_tsp(value);

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
    let volume_l = VolumeQuantity::from_l(value);
    let volume_ml = VolumeQuantity::from_ml(value);
    let volume_pt = VolumeQuantity::from_pt(value);
    let volume_gal = VolumeQuantity::from_gal(value);
    let volume_tbsp = VolumeQuantity::from_tbsp(value);
    let volume_tsp = VolumeQuantity::from_tsp(value);

    assert_eq!(volume_l.to_string(), "4.2L");
    assert_eq!(volume_ml.to_string(), "4.2mL");
    assert_eq!(volume_pt.to_string(), "4.2pt");
    assert_eq!(volume_gal.to_string(), "4.2gal");
    assert_eq!(volume_tbsp.to_string(), "4.2tbsp");
    assert_eq!(volume_tsp.to_string(), "4.2tsp");
}

#[test]
fn test_volume_add() {
    let volume_ml_1 = VolumeQuantity::from_ml(100f64);
    let volume_ml_2 = VolumeQuantity::from_ml(500f64);
    let volume_l = VolumeQuantity::from_l(2f64);

    let volume_ml_1_plus_ml_2 = VolumeQuantity::from_ml(600f64);
    let volume_l_plus_ml_1 = VolumeQuantity::from_l(2.1);
    let volume_ml_2_plus_l = VolumeQuantity::from_ml(2500f64);

    assert_eq!(volume_ml_1 + volume_ml_2, volume_ml_1_plus_ml_2);
    assert_eq!(volume_l + volume_ml_1, volume_l_plus_ml_1);
    assert_eq!(volume_ml_2 + volume_l, volume_ml_2_plus_l);
}

#[test]
fn test_volume_subtract() {
    let volume_ml_1 = VolumeQuantity::from_ml(6700f64);
    let volume_ml_2 = VolumeQuantity::from_ml(4700f64);
    let volume_l = VolumeQuantity::from_l(1.2f64);

    let volume_ml_1_minus_ml_2 = VolumeQuantity::from_ml(2000f64);
    let volume_l_minus_ml_1 = VolumeQuantity::from_l(-5.5);
    let volume_ml_2_minus_l = VolumeQuantity::from_ml(3500f64);

    assert_eq!((volume_ml_1 - volume_ml_2).round(1), volume_ml_1_minus_ml_2);
    assert_eq!((volume_l - volume_ml_1).round(1), volume_l_minus_ml_1);
    assert_eq!(volume_ml_2 - volume_l, volume_ml_2_minus_l);
}

#[test]
fn test_volume_multiply() {
    let volume_l_1 = VolumeQuantity::from_l(70f64);
    let volume_l_2 = VolumeQuantity::from_l(350f64);
    let liter_l_3 = VolumeQuantity::from_l(267.4f64);

    assert_eq!(volume_l_1 * 5, volume_l_2);
    assert_eq!(volume_l_1 * 3.82, liter_l_3);
}

#[test]
fn test_volume_divide() {
    let volume_l_1 = VolumeQuantity::from_l(350f64);
    let volume_l_2 = VolumeQuantity::from_l(70f64);

    assert_eq!(volume_l_1 / 5, volume_l_2);
}

#[test]
fn test_volume_sum() {
    let volume_1 = VolumeQuantity::from_l(30f64);
    let volume_2 = VolumeQuantity::from_l(20f64);
    let volume_3 = VolumeQuantity::from_l(50f64).to_floz();
    let volume_4 = VolumeQuantity::from_l(20f64).to_floz();
    let volume_5 = VolumeQuantity::from_l(130f64);
    let volume_total = VolumeQuantity::from_l(250f64);

    let volumes = vec![volume_1, volume_2, volume_3, volume_4, volume_5];

    let sum: VolumeQuantity = volumes.iter().map(|volume| *volume * 2).sum();
    assert_eq!(sum.get_unit(), volume_5.get_unit());
    assert_eq!(sum, (volume_total * 2).to_unit(volume_5.get_unit()));
}

#[test]
fn test_volume_partial_order() {
    let volume_ml_1 = VolumeQuantity::from_ml(6700f64);
    let volume_ml_2 = VolumeQuantity::from_ml(4700f64);
    let volume_l = VolumeQuantity::from_l(5.2f64);
    assert!(volume_ml_1 > volume_ml_2);
    assert!(volume_ml_1 > volume_l);
    assert!(volume_l > volume_ml_2);
}
