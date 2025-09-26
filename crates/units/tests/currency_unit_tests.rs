use std::collections::BTreeSet;
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
