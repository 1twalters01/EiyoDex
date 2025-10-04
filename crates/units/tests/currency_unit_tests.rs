use std::{collections::BTreeSet, str::FromStr};
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
fn test_from_str() {
    assert_eq!(CurrencyUnit::from_str("usd").unwrap(), CurrencyUnit::USD);
    assert_eq!(CurrencyUnit::from_str("uSd").unwrap(), CurrencyUnit::USD);
    assert_eq!(CurrencyUnit::from_str("USD").unwrap(), CurrencyUnit::USD);
    assert_eq!(CurrencyUnit::from_str("GBP").unwrap(), CurrencyUnit::GBP);
    assert_eq!(CurrencyUnit::from_str("EUR").unwrap(), CurrencyUnit::EUR);
    assert_eq!(CurrencyUnit::from_str("JPY").unwrap(), CurrencyUnit::JPY);
}
