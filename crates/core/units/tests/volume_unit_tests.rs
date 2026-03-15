use std::{collections::BTreeSet, str::FromStr};
use units::{measurement_system::MeasurementSystem, volume::unit::VolumeUnit};
use utils::database::DatabaseService;

#[test]
fn test_get_volume_unit_enumerations() {
    let function_enumerations = VolumeUnit::get_enumerations();
    let manual_enumerations = &vec![
        VolumeUnit::Liter,
        VolumeUnit::Milliliter,
        VolumeUnit::Pint,
        VolumeUnit::Gallon,
        VolumeUnit::FluidOunce,
        VolumeUnit::Teaspoon,
        VolumeUnit::Tablespoon,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(VolumeUnit::Liter.get_symbol(), "L");
    assert_eq!(VolumeUnit::Milliliter.get_symbol(), "mL");
    assert_eq!(VolumeUnit::Pint.get_symbol(), "pt");
    assert_eq!(VolumeUnit::Gallon.get_symbol(), "gal");
    assert_eq!(VolumeUnit::FluidOunce.get_symbol(), "fl oz");
    assert_eq!(VolumeUnit::Teaspoon.get_symbol(), "tsp");
    assert_eq!(VolumeUnit::Tablespoon.get_symbol(), "tbsp");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(VolumeUnit::Liter.get_unit_type(), "liter");
    assert_eq!(VolumeUnit::Milliliter.get_unit_type(), "milliliter");
    assert_eq!(VolumeUnit::Pint.get_unit_type(), "pint");
    assert_eq!(VolumeUnit::Gallon.get_unit_type(), "gallon");
    assert_eq!(VolumeUnit::FluidOunce.get_unit_type(), "fluid ounce");
    assert_eq!(VolumeUnit::Tablespoon.get_unit_type(), "tablespoon");
    assert_eq!(VolumeUnit::Teaspoon.get_unit_type(), "teaspoon");
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(VolumeUnit::Liter.get_unit_type_plural(), "liters");
    assert_eq!(VolumeUnit::Milliliter.get_unit_type_plural(), "milliliters");
    assert_eq!(VolumeUnit::Pint.get_unit_type_plural(), "pints");
    assert_eq!(VolumeUnit::Gallon.get_unit_type_plural(), "gallons");
    assert_eq!(
        VolumeUnit::FluidOunce.get_unit_type_plural(),
        "fluid ounces"
    );
    assert_eq!(VolumeUnit::Tablespoon.get_unit_type_plural(), "tablespoons");
    assert_eq!(VolumeUnit::Teaspoon.get_unit_type_plural(), "teaspoons");
}

#[test]
fn test_get_measurement_systems() {
    assert_eq!(
        VolumeUnit::Liter.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        VolumeUnit::Milliliter.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        VolumeUnit::Pint.get_measurement_system(),
        MeasurementSystem::Imperial
    );
    assert_eq!(
        VolumeUnit::Gallon.get_measurement_system(),
        MeasurementSystem::Imperial
    );
    assert_eq!(
        VolumeUnit::FluidOunce.get_measurement_system(),
        MeasurementSystem::Imperial
    );
    assert_eq!(
        VolumeUnit::Tablespoon.get_measurement_system(),
        MeasurementSystem::Imperial
    );
    assert_eq!(
        VolumeUnit::Teaspoon.get_measurement_system(),
        MeasurementSystem::Imperial
    );
}

#[test]
fn test_get_si_factor() {
    assert_eq!(VolumeUnit::Liter.si_factor(), 0.001);
    assert_eq!(VolumeUnit::Milliliter.si_factor(), 0.000001);
    assert_eq!(VolumeUnit::Pint.si_factor(), 0.0005682612);
    assert_eq!(VolumeUnit::Gallon.si_factor(), 0.00454609);
    assert_eq!(VolumeUnit::FluidOunce.si_factor(), 0.00002841306);
    assert_eq!(VolumeUnit::Tablespoon.si_factor(), 0.00001775816);
    assert_eq!(VolumeUnit::Teaspoon.si_factor(), 0.000005919387);
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

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();
    let res = VolumeUnit::save_enumerations_to_database(&pool).await;
    assert!(res.is_ok());

    let volume = VolumeUnit::Pint.get_unit_type();
    let rows = sqlx::query!(
        r#"
            SELECT id, unit_type
            FROM units_volume_types
            WHERE unit_type = ?
        "#,
        volume
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let unit_type = rows.unit_type;

    assert_eq!(unit_type, "pint");
}

#[tokio::test]
async fn test_get_database_id() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();
    let res = VolumeUnit::save_enumerations_to_database(&pool).await;
    assert!(res.is_ok());

    let pint = VolumeUnit::Pint;
    let id = pint.get_database_id(&pool).await;
    assert!(id.is_ok());
}
