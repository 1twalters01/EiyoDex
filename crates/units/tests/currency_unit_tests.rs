use std::{collections::BTreeSet, str::FromStr};
use chrono::NaiveDate;
use units::currency::CurrencyUnit;

#[test]
fn test_get_mass_unit_enumerations() {
    let function_enumerations = CurrencyUnit::get_enumerations();
    let manual_enumerations = vec![
        &CurrencyUnit::USD,
        &CurrencyUnit::GBP,
        &CurrencyUnit::EUR,
        &CurrencyUnit::JPY,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(CurrencyUnit::USD.as_symbol(), "$");
    assert_eq!(CurrencyUnit::GBP.as_symbol(), "£");
    assert_eq!(CurrencyUnit::EUR.as_symbol(), "€");
    assert_eq!(CurrencyUnit::JPY.as_symbol(), "¥");
}

#[test]
fn test_get_codes() {
    assert_eq!(CurrencyUnit::USD.as_code(), "USD");
    assert_eq!(CurrencyUnit::GBP.as_code(), "GBP");
    assert_eq!(CurrencyUnit::EUR.as_code(), "EUR");
    assert_eq!(CurrencyUnit::JPY.as_code(), "JPY");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(CurrencyUnit::USD.as_unit_type(), "dollar");
    assert_eq!(CurrencyUnit::GBP.as_unit_type(), "pound");
    assert_eq!(CurrencyUnit::EUR.as_unit_type(), "euro");
    assert_eq!(CurrencyUnit::JPY.as_unit_type(), "yen");
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(CurrencyUnit::USD.as_unit_type_plural(), "dollars");
    assert_eq!(CurrencyUnit::GBP.as_unit_type_plural(), "pounds");
    assert_eq!(CurrencyUnit::EUR.as_unit_type_plural(), "euros");
    assert_eq!(CurrencyUnit::JPY.as_unit_type_plural(), "yen");
}

#[tokio::test]
async fn test_to_dollar_now_async() {
    // assert_eq!(CurrencyUnit::USD.to_usd_now_async().await, Ok(1f64));
    // assert_eq!(CurrencyUnit::GBP.to_usd_now_async().await, Ok(1.3435));
    // assert_eq!(CurrencyUnit::EUR.to_usd_now_async().await, Ok(1.1745));
}

#[test]
fn test_to_dollar_now_sync() {
    // assert_eq!(CurrencyUnit::USD.to_usd_now_sync(), Ok(1f64));
    // assert_eq!(CurrencyUnit::GBP.to_usd_now_sync(), Ok(1.3435));
    // assert_eq!(CurrencyUnit::EUR.to_usd_now_sync(), Ok(1.1745));
}

#[tokio::test]
async fn test_to_dollar_at_time_async() {
    // let date = NaiveDate::from_ymd_opt(1999, 11, 25).unwrap();
    // assert_eq!(CurrencyUnit::USD.to_usd_at_time_async(date).await, Ok(1f64));
    // assert_eq!(CurrencyUnit::GBP.to_usd_at_time_async(date).await, Ok(1.615));
}

#[test]
fn test_to_dollar_at_time_sync() {
    // let date = NaiveDate::from_ymd_opt(1999, 11, 25).unwrap();
    // assert_eq!(CurrencyUnit::USD.to_usd_at_time_sync(date), Ok(1f64));
    // assert_eq!(CurrencyUnit::GBP.to_usd_at_time_sync(date), Ok(1.615));
}

#[tokio::test]
async fn test_get_current_exchange_rate_async() {
    // assert_eq!(CurrencyUnit::USD.get_current_exchange_rate_async(&CurrencyUnit::USD).await, Ok(1f64));
    // assert_eq!(CurrencyUnit::GBP.get_current_exchange_rate_async(&CurrencyUnit::USD).await, Ok(1.3435));
    // assert_eq!(CurrencyUnit::USD.get_current_exchange_rate_async(&CurrencyUnit::GBP).await, Ok(0.744324525493115));
    // assert_eq!(CurrencyUnit::GBP.get_current_exchange_rate_async(&CurrencyUnit::EUR).await, Ok(1.1438910174542356));
    assert_eq!(CurrencyUnit::GBP.get_current_exchange_rate_async(&CurrencyUnit::GBP).await, Ok(1f64));
}

#[test]
fn test_get_current_exchange_rate_sync() {
    // assert_eq!(CurrencyUnit::USD.get_current_exchange_rate_sync(&CurrencyUnit::USD), Ok(1f64));
    // assert_eq!(CurrencyUnit::GBP.get_current_exchange_rate_sync(&CurrencyUnit::USD), Ok(1.3435));
    // assert_eq!(CurrencyUnit::USD.get_current_exchange_rate_sync(&CurrencyUnit::GBP), Ok(0.744324525493115));
    // assert_eq!(CurrencyUnit::GBP.get_current_exchange_rate_sync(&CurrencyUnit::EUR), Ok(1.1438910174542356));
    assert_eq!(CurrencyUnit::GBP.get_current_exchange_rate_sync(&CurrencyUnit::GBP), Ok(1f64));
}

#[test]
fn test_from_str() {
    assert_eq!(CurrencyUnit::from_str("usd").unwrap(), CurrencyUnit::USD);
    assert_eq!(CurrencyUnit::from_str("uSd").unwrap(), CurrencyUnit::USD);
    assert_eq!(CurrencyUnit::from_str("USD").unwrap(), CurrencyUnit::USD);
    assert_eq!(CurrencyUnit::from_str("GBP").unwrap(), CurrencyUnit::GBP);
    assert_eq!(CurrencyUnit::from_str("EUR").unwrap(), CurrencyUnit::EUR);
    assert_eq!(CurrencyUnit::from_str("JPY").unwrap(), CurrencyUnit::JPY);
}
