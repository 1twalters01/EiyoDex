use units::{
    currency::{quantity::CurrencyQuantity, unit::CurrencyUnit},
    density::{quantity::DensityQuantity, unit::DensityUnit},
    mass::{quantity::MassQuantity, unit::MassUnit},
    measurement_system::MeasurementSystem,
    entity::{GetFromDatabaseUsingId, Entity},
    specific_currency::{
        quantity::SpecificCurrencyQuantity,
        unit::{Denominator, DenominatorType, SpecificCurrencyUnit},
    },
    volume::{quantity::VolumeQuantity, unit::VolumeUnit},
};
use utils::database::DatabaseService;

#[test]
fn test_density_from_variants() {
    let value = 5.82;

    let gbp = CurrencyUnit::GBP;
    let gram = Denominator::MassDenominator(MassUnit::Gram);
    let liter = Denominator::VolumeDenominator(VolumeUnit::Liter);

    assert_eq!(
        SpecificCurrencyQuantity::from_variants(value, gbp, gram),
        SpecificCurrencyQuantity::new(value, SpecificCurrencyUnit::GBPPerGram),
    );
    assert_eq!(
        SpecificCurrencyQuantity::from_variants(value, gbp, liter),
        SpecificCurrencyQuantity::new(value, SpecificCurrencyUnit::GBPPerLiter),
    );
}

#[test]
fn test_density_new() {
    let value = 10 as f64;

    let specific_currency_new =
        SpecificCurrencyQuantity::new(value, SpecificCurrencyUnit::USDPerGram);
    let specific_currency_from = SpecificCurrencyQuantity::from_usd_per_g(value);
    assert_eq!(specific_currency_new, specific_currency_from);
}

// #[test]
// fn test_density_rounding() {
//     let value = 5.6803294822;
//     let value_2 = 147.20472986;
//
//     let mut specific_currency_new = SpecificCurrencyQuantity::new(value,
// SpecificCurrencyUnit::GBPPerLiter);     let specific_currency_rounded =
// specific_currency_new.round(5);     let specific_currency_manual =
// SpecificCurrencyQuantity::new(5.68033, SpecificCurrencyUnit::GBPPerLiter);
//     assert_eq!(specific_currency_rounded, specific_currency_manual);
//
//     let mut specific_currency_new_2 = SpecificCurrencyQuantity::new(value_2,
// SpecificCurrencyUnit::GBPPerLiter);     let specific_currency_rounded_2 =
// specific_currency_new_2.round(5);     let specific_currency_coded_2 =
// SpecificCurrencyQuantity::new(147.20473, SpecificCurrencyUnit::GBPPerLiter);
//     assert_eq!(specific_currency_rounded_2, specific_currency_coded_2);
// }

// #[test]
// fn test_specific_currency_as_fn() {
//     let value = 6.9;
//     let new_value = value / 1000f64;
//     let new_value_2 = value * 0.8698711302029329;
//
//     let eur_per_l = SpecificCurrencyQuantity::from_eur_per_l(value);
//     let eur_per_ml = SpecificCurrencyQuantity::from_eur_per_ml(new_value);
//     let gbp_per_l = SpecificCurrencyQuantity::from_gbp_per_l(new_value_2);
//     let eur_per_ml_transformed = eur_per_l.as_eur_per_ml().unwrap();
//     let gbp_per_l_transformed = eur_per_l.as_gbp_per_l().unwrap();
//     assert_eq!(eur_per_ml.get_value(), eur_per_ml_transformed);
//     assert_eq!(gbp_per_l.get_value(), gbp_per_l_transformed);
// }

// #[test]
// fn test_specific_currency_to_unit() {
//     let value = 6.9;
//     let new_value = value / 1000f64;
//     let new_value_2 = value * 0.8698711302029329;
//
//     let eur_per_l = SpecificCurrencyQuantity::from_eur_per_l(value);
//     let eur_per_ml = SpecificCurrencyQuantity::from_eur_per_ml(new_value);
//     let gbp_per_l = SpecificCurrencyQuantity::from_gbp_per_l(new_value_2);
//     let eur_per_ml_transformed =
// eur_per_l.to_unit(SpecificCurrencyUnit::EURPerMilliliter).unwrap();
//     let gbp_per_l_transformed =
// eur_per_l.to_unit(SpecificCurrencyUnit::GBPPerLiter).unwrap();     assert_eq!
// (eur_per_ml, eur_per_ml_transformed);     assert_eq!(gbp_per_l,
// gbp_per_l_transformed); }

// #[test]
// fn test_specific_currency_to_fn() {
//     let value = 6.9;
//     let new_value = value / 1000f64;
//     let new_value_2 = value * 0.8698711302029329;
//
//     let eur_per_l = SpecificCurrencyQuantity::from_eur_per_l(value);
//     let eur_per_ml = SpecificCurrencyQuantity::from_eur_per_ml(new_value);
//     let gbp_per_l = SpecificCurrencyQuantity::from_gbp_per_l(new_value_2);
//     let eur_per_ml_transformed = eur_per_l.to_eur_per_ml().unwrap();
//     let gbp_per_l_transformed = eur_per_l.to_gbp_per_l().unwrap();
//     assert_eq!(eur_per_ml, eur_per_ml_transformed);
//     assert_eq!(gbp_per_l, gbp_per_l_transformed);
// }

#[test]
fn test_specific_currency_is_zero() {
    let zero_density = SpecificCurrencyQuantity::from_eur_per_g(0f64);
    let specific_currency = SpecificCurrencyQuantity::from_eur_per_g(5.5);

    assert!(zero_density.is_zero());
    assert!(!specific_currency.is_zero());
}

#[test]
fn test_density_is_negative() {
    let negative_density = SpecificCurrencyQuantity::from_eur_per_g(-5.5f64);
    let specific_currency = SpecificCurrencyQuantity::from_eur_per_g(5.5);

    assert!(negative_density.is_negative());
    assert!(!specific_currency.is_negative());
}

#[test]
fn test_density_get_value() {
    let specific_currency =
        SpecificCurrencyQuantity::new(6.882, SpecificCurrencyUnit::USDPerKilogram);
    assert_eq!(specific_currency.get_value(), 6.882);
}

#[test]
fn test_density_set_value() {
    let mut specific_currency =
        SpecificCurrencyQuantity::new(6.882, SpecificCurrencyUnit::GBPPerGallon);
    assert_eq!(specific_currency.get_value(), 6.882);
    specific_currency.set_value(8.92);
    assert_eq!(specific_currency.get_value(), 8.92);
}

#[test]
fn test_density_get_unit() {
    let specific_currency =
        SpecificCurrencyQuantity::new(6.882, SpecificCurrencyUnit::USDPerKilogram);
    assert_eq!(
        specific_currency.get_unit(),
        SpecificCurrencyUnit::USDPerKilogram
    );
}

#[test]
fn test_density_set_unit() {
    let mut specific_currency =
        SpecificCurrencyQuantity::new(6.882, SpecificCurrencyUnit::GBPPerGallon);
    assert_eq!(
        specific_currency.get_unit(),
        SpecificCurrencyUnit::GBPPerGallon
    );
    specific_currency.set_unit(SpecificCurrencyUnit::USDPerPint);
    assert_eq!(
        specific_currency.get_unit(),
        SpecificCurrencyUnit::USDPerPint
    );
}

#[test]
fn test_get_denominator_type() {
    let specific_currency = SpecificCurrencyQuantity::new(6.882, SpecificCurrencyUnit::USDPerGram);
    assert_eq!(
        specific_currency.get_denominator_type(),
        DenominatorType::MassDenominator
    );

    let specific_currency_2 =
        SpecificCurrencyQuantity::new(6.882, SpecificCurrencyUnit::EURPerLiter);
    assert_eq!(
        specific_currency_2.get_denominator_type(),
        DenominatorType::VolumeDenominator
    );
}

#[test]
fn test_specific_currency_get_symbol() {
    let specific_currency = SpecificCurrencyQuantity::new(6.882, SpecificCurrencyUnit::USDPerGram);
    assert_eq!(specific_currency.get_symbol(), "$/g");
}

#[test]
fn test_specific_currency_get_measurement_system() {
    let specific_currency = SpecificCurrencyQuantity::new(6.882, SpecificCurrencyUnit::USDPerGram);
    assert_eq!(
        specific_currency.get_measurement_system(),
        MeasurementSystem::Metric
    );
}

#[test]
fn test_specific_currency_get_unit_type() {
    let specific_currency = SpecificCurrencyQuantity::new(6.882, SpecificCurrencyUnit::USDPerGram);
    assert_eq!(specific_currency.get_unit_type(), "dollar per gram");
}

#[test]
fn test_specific_currency_get_unit_type_plural() {
    let specific_currency = SpecificCurrencyQuantity::new(6.882, SpecificCurrencyUnit::USDPerGram);
    assert_eq!(specific_currency.get_unit_type_plural(), "dollars per gram");
}

#[test]
fn test_specific_currency_to_string() {
    let specific_currency = SpecificCurrencyQuantity::new(6.88, SpecificCurrencyUnit::USDPerGram);
    assert_eq!(specific_currency.to_string(), "6.88$/g");
}

#[test]
fn test_specific_currency_multiplication() {
    let specific_currency =
        SpecificCurrencyQuantity::new(6.1, SpecificCurrencyUnit::USDPerMilliliter);
    let specific_currency_2 =
        SpecificCurrencyQuantity::new(18.3, SpecificCurrencyUnit::USDPerMilliliter);
    assert_eq!((specific_currency * 3).round(2), specific_currency_2);
}

#[test]
fn test_specific_currency_multiplication_volume_and_specific_currency() {
    let specific_currency =
        SpecificCurrencyQuantity::new(6.1, SpecificCurrencyUnit::USDPerMilliliter);
    let volume = VolumeQuantity::new(2.5, VolumeUnit::Liter);
    let currency = CurrencyQuantity::new(15_250f64, CurrencyUnit::USD);
    assert_eq!(volume * specific_currency, currency);
    assert_eq!(specific_currency * volume, currency);
}

#[test]
fn test_specific_currency_multiplication_mass_and_specific_currency() {
    let specific_currency = SpecificCurrencyQuantity::new(6.1, SpecificCurrencyUnit::USDPerGram);
    let mass = MassQuantity::new(2.5, MassUnit::Gram);
    let currency = CurrencyQuantity::new(15.25, CurrencyUnit::USD);
    assert_eq!(mass * specific_currency, currency);
    assert_eq!(specific_currency * mass, currency);
}

#[test]
fn test_specific_currency_multiplication_density_and_specific_currency() {
    let specific_currency = SpecificCurrencyQuantity::new(6.1, SpecificCurrencyUnit::EURPerOunce);
    let density = DensityQuantity::new(2.5, DensityUnit::OuncePerLiter);
    let specific_currency_2 =
        SpecificCurrencyQuantity::new(15.25, SpecificCurrencyUnit::EURPerLiter);
    assert_eq!((density * specific_currency).round(2), specific_currency_2);
    assert_eq!((specific_currency * density).round(2), specific_currency_2);
}

#[test]
fn test_specific_currency_division() {
    let specific_currency =
        SpecificCurrencyQuantity::new(18.3, SpecificCurrencyUnit::USDPerMilliliter);
    let specific_currency_2 =
        SpecificCurrencyQuantity::new(6.1, SpecificCurrencyUnit::USDPerMilliliter);
    assert_eq!((specific_currency / 3).round(2), specific_currency_2);
}

#[test]
fn test_specific_currency_division_currency_and_volume() {
    let currency = CurrencyQuantity::new(15.25f64, CurrencyUnit::USD);
    let volume = VolumeQuantity::new(2.5, VolumeUnit::Liter);
    let specific_currency = SpecificCurrencyQuantity::new(6.1, SpecificCurrencyUnit::USDPerLiter);
    assert_eq!(currency / volume, specific_currency);
}

#[test]
fn test_specific_currency_division_currency_and_mass() {
    let currency = CurrencyQuantity::new(15.25, CurrencyUnit::USD);
    let mass = MassQuantity::new(2.5, MassUnit::Gram);
    let specific_currency = SpecificCurrencyQuantity::new(6.1, SpecificCurrencyUnit::USDPerGram);
    assert_eq!(currency / mass, specific_currency);
}

#[test]
fn test_specific_currency_division_specific_currency_and_density() {
    let specific_currency = SpecificCurrencyQuantity::new(15.25, SpecificCurrencyUnit::EURPerLiter);
    let density = DensityQuantity::new(2.5, DensityUnit::OuncePerLiter);
    let specific_currency_2 = SpecificCurrencyQuantity::new(6.1, SpecificCurrencyUnit::EURPerOunce);
    assert_eq!((specific_currency / density).round(2), specific_currency_2);
}

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();

    let _ = CurrencyUnit::save_enumerations_to_database(&pool).await;
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;

    let specific_currency_eur_per_ug = SpecificCurrencyQuantity::from_eur_per_ug(6700f64);
    let specific_currency_record = Entity::new(specific_currency_eur_per_ug);

    let res = specific_currency_record.save_to_database(&pool).await;
    assert!(res.is_ok());

    let specific_currency_saved = SpecificCurrencyQuantity::get_from_database_using_id(
        specific_currency_record.get_id(),
        &pool,
    )
    .await;
    println!("{:#?}", specific_currency_saved);
    assert!(specific_currency_saved.is_ok());
    assert_eq!(specific_currency_saved.unwrap(), specific_currency_record);

    let res = specific_currency_record
        .delete_from_database_using_id(&pool)
        .await;
    assert!(res.is_ok());

    let density_saved_2 = SpecificCurrencyQuantity::get_from_database_using_id(
        specific_currency_record.get_id(),
        &pool,
    )
    .await;
    assert!(density_saved_2.is_err());
}
