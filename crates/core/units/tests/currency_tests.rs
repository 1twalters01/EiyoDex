// use chrono::NaiveDate;
use units::{
    currency::{quantity::CurrencyQuantity, unit::CurrencyUnit},
    record::{GetFromDatabaseUsingId, Record},
};
use utils::database::DatabaseService;

#[test]
fn test_new_currency() {
    let value = 10f64;

    let new_dollars = CurrencyQuantity::new(value, CurrencyUnit::USD);
    let from_dollars = CurrencyQuantity::from_usd(value);
    assert_eq!(new_dollars, from_dollars);

    let new_pounds = CurrencyQuantity::new(value, CurrencyUnit::GBP);
    let from_pounds = CurrencyQuantity::from_gbp(value);
    assert_eq!(new_pounds, from_pounds);

    let new_euros = CurrencyQuantity::new(value, CurrencyUnit::EUR);
    let from_euros = CurrencyQuantity::from_eur(value);
    assert_eq!(new_euros, from_euros);

    let new_yen = CurrencyQuantity::new(value, CurrencyUnit::JPY);
    let from_yen = CurrencyQuantity::from_jpy(value);
    assert_eq!(new_yen, from_yen);
}

#[test]
fn test_currency_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20473186;

    let mut currency_new = CurrencyQuantity::new(value, CurrencyUnit::USD);
    let currency_rounded = currency_new.round(5);
    let currency_coded = CurrencyQuantity::new(5.68033, CurrencyUnit::USD);
    assert_eq!(currency_rounded, currency_coded);

    let mut currency_new_2 = CurrencyQuantity::new(value_2, CurrencyUnit::USD);
    let currency_rounded_2 = currency_new_2.round(5);
    let currency_coded_2 = CurrencyQuantity::new(147.20473, CurrencyUnit::USD);
    assert_eq!(currency_rounded_2, currency_coded_2);
}

// #[test]
// fn test_currency_as_fn() {
//     let value_gbp = 5.67;
//     let gbp = CurrencyQuantity::from_gbp(value_gbp);
//     let gbp_as_eur = gbp.as_eur();
//
//     let value_eur = value_gbp * 1.1438910174542356;
//
//     assert_eq!(value_eur, gbp_as_eur.unwrap());
// }

// #[test]
// fn test_currency_to_fn() {
//     let value_gbp = 5.67f64;
//     let value_eur = value_gbp * 1.1438910174542356;
//
//     let gbp = CurrencyQuantity::from_gbp(value_gbp);
//     let eur = CurrencyQuantity::from_eur(value_eur);
//     let gbp_to_eur = gbp.to_eur();
//
//     assert_eq!(eur, gbp_to_eur.unwrap());
// }

#[test]
fn test_currency_is_zero() {
    let zero_pounds = CurrencyQuantity::from_gbp(0f64);
    let pounds = CurrencyQuantity::from_gbp(5.5);

    assert!(zero_pounds.is_zero());
    assert!(!pounds.is_zero());
}

#[test]
fn test_currency_is_negative() {
    let negative_pounds = CurrencyQuantity::from_gbp(-5.5f64);
    let pounds = CurrencyQuantity::from_gbp(5.5);

    assert!(negative_pounds.is_negative());
    assert!(!pounds.is_negative());
}

#[test]
fn test_currency_get_value() {
    let currency = CurrencyQuantity::new(6.882, CurrencyUnit::USD);
    assert_eq!(currency.get_value(), 6.882);
}

#[test]
fn test_currency_set_value() {
    let mut currency = CurrencyQuantity::new(6.882, CurrencyUnit::USD);
    currency.set_value(8.92);
    assert_eq!(currency.get_value(), 8.92);
}

#[test]
fn test_currency_get_unit() {
    let currency = CurrencyQuantity::new(6.882, CurrencyUnit::USD);
    assert_eq!(currency.get_unit(), CurrencyUnit::USD);
}

#[test]
fn test_currency_set_unit() {
    let mut currency = CurrencyQuantity::new(6.882, CurrencyUnit::USD);
    currency.set_unit(CurrencyUnit::GBP);
    assert_eq!(currency.get_unit(), CurrencyUnit::GBP);
}

#[test]
fn test_get_symbols() {
    let value = 4.2;
    let currency_usd = CurrencyQuantity::from_usd(value);
    let currency_gbp = CurrencyQuantity::from_gbp(value);
    let currency_eur = CurrencyQuantity::from_eur(value);
    let currency_jpy = CurrencyQuantity::from_jpy(value);

    assert_eq!(currency_usd.get_symbol(), "$");
    assert_eq!(currency_gbp.get_symbol(), "£");
    assert_eq!(currency_eur.get_symbol(), "€");
    assert_eq!(currency_jpy.get_symbol(), "¥");
}

#[test]
fn test_get_unit_types() {
    let value = 4.2;
    let currency_usd = CurrencyQuantity::from_usd(value);
    let currency_gbp = CurrencyQuantity::from_gbp(value);
    let currency_eur = CurrencyQuantity::from_eur(value);
    let currency_jpy = CurrencyQuantity::from_jpy(value);

    assert_eq!(currency_usd.get_unit_type(), "dollar");
    assert_eq!(currency_gbp.get_unit_type(), "pound");
    assert_eq!(currency_eur.get_unit_type(), "euro");
    assert_eq!(currency_jpy.get_unit_type(), "yen");
}

#[test]
fn test_get_unit_types_plural() {
    let value = 4.2;
    let currency_usd = CurrencyQuantity::from_usd(value);
    let currency_gbp = CurrencyQuantity::from_gbp(value);
    let currency_eur = CurrencyQuantity::from_eur(value);
    let currency_jpy = CurrencyQuantity::from_jpy(value);

    assert_eq!(currency_usd.get_unit_type_plural(), "dollars");
    assert_eq!(currency_gbp.get_unit_type_plural(), "pounds");
    assert_eq!(currency_eur.get_unit_type_plural(), "euros");
    assert_eq!(currency_jpy.get_unit_type_plural(), "yen");
}

#[test]
fn test_get_codes() {
    let value = 4.2;
    let currency_usd = CurrencyQuantity::from_usd(value);
    let currency_gbp = CurrencyQuantity::from_gbp(value);
    let currency_eur = CurrencyQuantity::from_eur(value);
    let currency_jpy = CurrencyQuantity::from_jpy(value);

    assert_eq!(currency_usd.get_code(), "USD");
    assert_eq!(currency_gbp.get_code(), "GBP");
    assert_eq!(currency_eur.get_code(), "EUR");
    assert_eq!(currency_jpy.get_code(), "JPY");
}

// #[tokio::test]
// async fn test_current_conversion_async() {
//     let value = 4.23;
//     let currency_usd = CurrencyQuantity::from_usd(value);
//     let currency_gbp = CurrencyQuantity::from_gbp(value);
//
//     assert_eq!(currency_usd.convert_to_async(CurrencyUnit::USD).await,
// Ok(CurrencyQuantity::from_usd(1f64 * value)));     assert_eq!(currency_gbp.
// convert_to_async(CurrencyUnit::USD).await,
// Ok(CurrencyQuantity::from_usd(1.3435 * value)));
// assert_eq!(currency_usd.convert_to_async(CurrencyUnit::GBP).
// await, Ok(CurrencyQuantity::from_gbp(0.744324525493115 * value)));
// assert_eq! (currency_gbp.convert_to_async(CurrencyUnit::EUR).await,
// Ok(CurrencyQuantity::from_eur(1.1438910174542356 * value)));     assert_eq!
// (currency_gbp.convert_to_async(CurrencyUnit::GBP).await,
// Ok(CurrencyQuantity::from_gbp(1f64 * value))); }

// #[test]
// fn test_current_conversion_sync() {
//     let value = 4.23;
//     let currency_usd = CurrencyQuantity::from_usd(value);
//     let currency_gbp = CurrencyQuantity::from_gbp(value);
//
//     assert_eq!(currency_usd.convert_to_sync(CurrencyUnit::USD),
// Ok(CurrencyQuantity::from_usd(1f64 * value)));     assert_eq!(currency_gbp.
// convert_to_sync(CurrencyUnit::USD), Ok(CurrencyQuantity::from_usd(1.3435 *
// value)));     assert_eq!(currency_usd.convert_to_sync(CurrencyUnit::GBP),
// Ok(CurrencyQuantity::from_gbp(0.744324525493115 * value)));     assert_eq!
// (currency_gbp.convert_to_sync(CurrencyUnit::EUR),
// Ok(CurrencyQuantity::from_eur(1.1438910174542356 * value)));     assert_eq!
// (currency_gbp.convert_to_sync(CurrencyUnit::GBP),
// Ok(CurrencyQuantity::from_gbp(1f64
// * value))); }

// #[tokio::test]
// async fn test_historic_conversion_async() {
//     let date = NaiveDate::from_ymd_opt(1999, 11, 25).unwrap();
//
//     let value = 4.23;
//     let currency_usd = CurrencyQuantity::from_usd(value);
//     let currency_gbp = CurrencyQuantity::from_gbp(value);
//
//     assert_eq!(currency_usd.convert_to_historic_async(CurrencyUnit::USD,
// date).await, Ok(CurrencyQuantity::from_usd(1f64 * value)));     assert_eq!
// (currency_gbp.convert_to_historic_async(CurrencyUnit::USD, date).await,
// Ok(CurrencyQuantity::from_usd(1.615 * value)));     assert_eq!(currency_usd.
// convert_to_historic_async(CurrencyUnit::GBP, date).await,
// Ok(CurrencyQuantity::from_gbp(0.744324525493115 * value)));     assert_eq!
// (currency_gbp.convert_to_historic_async(CurrencyUnit::EUR, date).await,
// Ok(CurrencyQuantity::from_eur(1.3750532141336738 * value)));     assert_eq!
// (currency_gbp.convert_to_historic_async(CurrencyUnit::GBP, date).await,
// Ok(CurrencyQuantity::from_gbp(1f64 * value))); }

// #[test]
// fn test_historic_conversion_sync() {
//     let date = NaiveDate::from_ymd_opt(1999, 11, 25).unwrap();
//
//     let value = 4.23;
//     let currency_usd = CurrencyQuantity::from_usd(value);
//     let currency_gbp = CurrencyQuantity::from_gbp(value);
//
//     assert_eq!(currency_usd.convert_to_historic_sync(CurrencyUnit::USD,
// date), Ok(CurrencyQuantity::from_usd(1f64 * 4.23)));
// assert_eq!(currency_gbp. convert_to_historic_sync(CurrencyUnit::USD, date),
// Ok(CurrencyQuantity::from_usd(1.615 * 4.23)));     assert_eq!(currency_usd.
// convert_to_historic_sync(CurrencyUnit::GBP, date),
// Ok(CurrencyQuantity::from_gbp(0.744324525493115 * 4.23)));     assert_eq!
// (currency_gbp.convert_to_historic_sync(CurrencyUnit::EUR, date),
// Ok(CurrencyQuantity::from_eur(1.3750532141336738 * 4.23)));     assert_eq!
// (currency_gbp.convert_to_historic_sync(CurrencyUnit::GBP, date),
// Ok(CurrencyQuantity::from_gbp(1f64 * 4.23))); }

#[test]
fn test_multiplication() {
    let value = 4.23;
    let value_2 = 8.46;
    let currency = CurrencyQuantity::from_usd(value);
    let currency_2 = CurrencyQuantity::from_usd(value_2);

    assert_eq!(currency * 2f64, currency_2);
}

#[test]
fn test_division() {
    let value = 8.46;
    let value_2 = 4.23;
    let currency = CurrencyQuantity::from_usd(value);
    let currency_2 = CurrencyQuantity::from_usd(value_2);

    assert_eq!(currency / 2f64, currency_2);
}

#[test]
fn test_addition() {
    let value_1 = 4.23;
    let value_2 = 2.32;
    let value_3 = 6.55;
    let currency_1 = CurrencyQuantity::from_usd(value_1);
    let currency_2 = CurrencyQuantity::from_usd(value_2);
    let currency_3 = CurrencyQuantity::from_usd(value_3);

    assert_eq!((currency_1 + currency_2).round(2), currency_3);
}

#[test]
fn test_subtraction() {
    let value_1 = 6.55;
    let value_2 = 2.32;
    let value_3 = 4.23;
    let currency_1 = CurrencyQuantity::from_usd(value_1);
    let currency_2 = CurrencyQuantity::from_usd(value_2);
    let currency_3 = CurrencyQuantity::from_usd(value_3);

    assert_eq!((currency_1 - currency_2).round(2), currency_3);
}

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();

    let _ = CurrencyUnit::save_enumerations_to_database(&pool).await;

    let currency_gbp = CurrencyQuantity::from_gbp(6700f64);
    let currency_record = Record::new(currency_gbp);

    let res = currency_record.save_to_database(&pool).await;
    assert!(res.is_ok());

    let currency_saved =
        CurrencyQuantity::get_from_database_using_id(currency_record.get_uuid(), &pool).await;
    assert!(currency_saved.is_ok());
    assert_eq!(currency_saved.unwrap(), currency_record);

    let res = currency_record.delete_from_database_using_id(&pool).await;
    assert!(res.is_ok());

    let currency_saved_2 =
        CurrencyQuantity::get_from_database_using_id(currency_record.get_uuid(), &pool).await;
    assert!(currency_saved_2.is_err());
}
