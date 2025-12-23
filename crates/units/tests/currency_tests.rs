use units::{
    currency::{Currency, CurrencyUnit},
};

#[test]
fn test_new_currency() {
}

#[test]
fn test_currency_rounding() {
}

#[test]
fn test_currency_as_fn() {
}

#[test]
fn test_currency_to_unit() {
}

#[test]
fn test_currency_to_fn() {
}

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

#[test]
fn test_current_conversion() {
}

#[test]
fn test_historic_conversion() {
}
