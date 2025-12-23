use chrono::NaiveDate;
use units::{
    currency::{Currency, CurrencyUnit},
};

#[test]
fn test_new_currency() {
    let value = 10f64;

    let new_dollars = Currency::new(value, CurrencyUnit::USD);
    let from_dollars = Currency::from_usd(value);
    assert_eq!(new_dollars, from_dollars);

    let new_pounds = Currency::new(value, CurrencyUnit::GBP);
    let from_pounds = Currency::from_gbp(value);
    assert_eq!(new_pounds, from_pounds);

    let new_euros = Currency::new(value, CurrencyUnit::EUR);
    let from_euros = Currency::from_eur(value);
    assert_eq!(new_euros, from_euros);

    let new_yen = Currency::new(value, CurrencyUnit::JPY);
    let from_yen = Currency::from_jpy(value);
    assert_eq!(new_yen, from_yen);
}

#[test]
fn test_currency_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20473186;

    let mut currency_new = Currency::new(value, CurrencyUnit::USD);
    let currency_rounded = currency_new.round(5);
    let currency_coded = Currency::new(5.68033, CurrencyUnit::USD);
    assert_eq!(currency_rounded, currency_coded);

    let mut currency_new_2 = Currency::new(value_2, CurrencyUnit::USD);
    let currency_rounded_2 = currency_new_2.round(5);
    let currency_coded_2 = Currency::new(147.20473, CurrencyUnit::USD);
    assert_eq!(currency_rounded_2, currency_coded_2);
}

// #[test]
// fn test_currency_as_fn() {
//     let value_gbp = 5.67;
//     let gbp = Currency::from_gbp(value_gbp);
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
//     let gbp = Currency::from_gbp(value_gbp);
//     let eur = Currency::from_eur(value_eur);
//     let gbp_to_eur = gbp.to_eur();
//
//     assert_eq!(eur, gbp_to_eur.unwrap());
// }

#[test]
fn test_currency_is_zero() {
    let zero_pounds = Currency::from_gbp(0f64);
    let pounds = Currency::from_gbp(5.5);

    assert!(zero_pounds.is_zero());
    assert!(!pounds.is_zero());
}

#[test]
fn test_currency_is_negative() {
    let negative_pounds = Currency::from_gbp(-5.5f64);
    let pounds = Currency::from_gbp(5.5);

    assert!(negative_pounds.is_negative());
    assert!(!pounds.is_negative());
}

#[test]
fn test_currency_get_value() {
    let currency = Currency::new(6.882, CurrencyUnit::USD);
    assert_eq!(currency.get_value(), 6.882);
}

#[test]
fn test_currency_set_value() {
    let mut currency = Currency::new(6.882, CurrencyUnit::USD);
    currency.set_value(8.92);
    assert_eq!(currency.get_value(), 8.92);
}

#[test]
fn test_currency_get_unit() {
    let currency = Currency::new(6.882, CurrencyUnit::USD);
    assert_eq!(currency.get_unit(), CurrencyUnit::USD);
}

#[test]
fn test_currency_set_unit() {
    let mut currency = Currency::new(6.882, CurrencyUnit::USD);
    currency.set_unit(CurrencyUnit::GBP);
    assert_eq!(currency.get_unit(), CurrencyUnit::GBP);
}

#[test]
fn test_get_symbols() {
    let value = 4.2;
    let currency_usd = Currency::from_usd(value);
    let currency_gbp = Currency::from_gbp(value);
    let currency_eur = Currency::from_eur(value);
    let currency_jpy = Currency::from_jpy(value);

    assert_eq!(currency_usd.get_symbol(), "$");
    assert_eq!(currency_gbp.get_symbol(), "£");
    assert_eq!(currency_eur.get_symbol(), "€");
    assert_eq!(currency_jpy.get_symbol(), "¥");
}

#[test]
fn test_get_unit_types() {
    let value = 4.2;
    let currency_usd = Currency::from_usd(value);
    let currency_gbp = Currency::from_gbp(value);
    let currency_eur = Currency::from_eur(value);
    let currency_jpy = Currency::from_jpy(value);

    assert_eq!(currency_usd.get_unit_type(), "dollar");
    assert_eq!(currency_gbp.get_unit_type(), "pound");
    assert_eq!(currency_eur.get_unit_type(), "euro");
    assert_eq!(currency_jpy.get_unit_type(), "yen");
}

#[test]
fn test_get_unit_types_plural() {
    let value = 4.2;
    let currency_usd = Currency::from_usd(value);
    let currency_gbp = Currency::from_gbp(value);
    let currency_eur = Currency::from_eur(value);
    let currency_jpy = Currency::from_jpy(value);

    assert_eq!(currency_usd.get_unit_type_plural(), "dollars");
    assert_eq!(currency_gbp.get_unit_type_plural(), "pounds");
    assert_eq!(currency_eur.get_unit_type_plural(), "euros");
    assert_eq!(currency_jpy.get_unit_type_plural(), "yen");
}

#[test]
fn test_get_codes() {
    let value = 4.2;
    let currency_usd = Currency::from_usd(value);
    let currency_gbp = Currency::from_gbp(value);
    let currency_eur = Currency::from_eur(value);
    let currency_jpy = Currency::from_jpy(value);

    assert_eq!(currency_usd.get_code(), "USD");
    assert_eq!(currency_gbp.get_code(), "GBP");
    assert_eq!(currency_eur.get_code(), "EUR");
    assert_eq!(currency_jpy.get_code(), "JPY");
}

// #[tokio::test]
// async fn test_current_conversion_async() {
//     let value = 4.23;
//     let currency_usd = Currency::from_usd(value);
//     let currency_gbp = Currency::from_gbp(value);
//
//     assert_eq!(currency_usd.convert_to_async(CurrencyUnit::USD).await, Ok(Currency::from_usd(1f64 * value)));
//     assert_eq!(currency_gbp.convert_to_async(CurrencyUnit::USD).await, Ok(Currency::from_usd(1.3435 * value)));
//     assert_eq!(currency_usd.convert_to_async(CurrencyUnit::GBP).await, Ok(Currency::from_gbp(0.744324525493115 * value)));
//     assert_eq!(currency_gbp.convert_to_async(CurrencyUnit::EUR).await, Ok(Currency::from_eur(1.1438910174542356 * value)));
//     assert_eq!(currency_gbp.convert_to_async(CurrencyUnit::GBP).await, Ok(Currency::from_gbp(1f64 * value)));
// }

// #[test]
// fn test_current_conversion_sync() {
//     let value = 4.23;
//     let currency_usd = Currency::from_usd(value);
//     let currency_gbp = Currency::from_gbp(value);
//     
//     assert_eq!(currency_usd.convert_to_sync(CurrencyUnit::USD), Ok(Currency::from_usd(1f64 * value)));
//     assert_eq!(currency_gbp.convert_to_sync(CurrencyUnit::USD), Ok(Currency::from_usd(1.3435 * value)));
//     assert_eq!(currency_usd.convert_to_sync(CurrencyUnit::GBP), Ok(Currency::from_gbp(0.744324525493115 * value)));
//     assert_eq!(currency_gbp.convert_to_sync(CurrencyUnit::EUR), Ok(Currency::from_eur(1.1438910174542356 * value)));
//     assert_eq!(currency_gbp.convert_to_sync(CurrencyUnit::GBP), Ok(Currency::from_gbp(1f64 * value)));
// }

// #[tokio::test]
// async fn test_historic_conversion_async() {
//     let date = NaiveDate::from_ymd_opt(1999, 11, 25).unwrap();
//
//     let value = 4.23;
//     let currency_usd = Currency::from_usd(value);
//     let currency_gbp = Currency::from_gbp(value);
//
//     assert_eq!(currency_usd.convert_to_historic_async(CurrencyUnit::USD, date).await, Ok(Currency::from_usd(1f64 * value)));
//     assert_eq!(currency_gbp.convert_to_historic_async(CurrencyUnit::USD, date).await, Ok(Currency::from_usd(1.615 * value)));
//     assert_eq!(currency_usd.convert_to_historic_async(CurrencyUnit::GBP, date).await, Ok(Currency::from_gbp(0.744324525493115 * value)));
//     assert_eq!(currency_gbp.convert_to_historic_async(CurrencyUnit::EUR, date).await, Ok(Currency::from_eur(1.3750532141336738 * value)));
//     assert_eq!(currency_gbp.convert_to_historic_async(CurrencyUnit::GBP, date).await, Ok(Currency::from_gbp(1f64 * value)));
// }

// #[test]
// fn test_historic_conversion_sync() {
//     let date = NaiveDate::from_ymd_opt(1999, 11, 25).unwrap();
//
//     let value = 4.23;
//     let currency_usd = Currency::from_usd(value);
//     let currency_gbp = Currency::from_gbp(value);
//
//     assert_eq!(currency_usd.convert_to_historic_sync(CurrencyUnit::USD, date), Ok(Currency::from_usd(1f64 * 4.23)));
//     assert_eq!(currency_gbp.convert_to_historic_sync(CurrencyUnit::USD, date), Ok(Currency::from_usd(1.615 * 4.23)));
//     assert_eq!(currency_usd.convert_to_historic_sync(CurrencyUnit::GBP, date), Ok(Currency::from_gbp(0.744324525493115 * 4.23)));
//     assert_eq!(currency_gbp.convert_to_historic_sync(CurrencyUnit::EUR, date), Ok(Currency::from_eur(1.3750532141336738 * 4.23)));
//     assert_eq!(currency_gbp.convert_to_historic_sync(CurrencyUnit::GBP, date), Ok(Currency::from_gbp(1f64 * 4.23)));
// }
