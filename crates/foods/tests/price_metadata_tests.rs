use foods::{
    merchant::Merchant,
    price_metadata::PriceMetadata,
};
use units::specific_currency::{SpecificCurrency, SpecificCurrencyUnit};
use uuid::Uuid;

#[test]
pub fn test_price_metadata() {
    let name = String::from("Butcher Y");
    let id = None;
    let merchant = Merchant::new(name, id);

    let mut specific_currency: Option<SpecificCurrency> = None;
    let mut price_metadata = PriceMetadata::new(merchant.clone(), specific_currency);

    assert_eq!(price_metadata.get_merchant().borrow().clone(), merchant);
    let mut merchant_2 = merchant.clone();
    merchant_2.set_description(String::from("description"));
    price_metadata.set_merchant(merchant_2.clone());
    assert_ne!(price_metadata.get_merchant().borrow().clone(), merchant);
    assert_eq!(price_metadata.get_merchant().borrow().clone(), merchant_2);

    assert_eq!(price_metadata.get_specific_currency(), None);
    let value = 3f64;
    let unit = SpecificCurrencyUnit::GBPPerGram;
    specific_currency = Some(SpecificCurrency::new(value, unit));
    price_metadata.set_specific_currency(specific_currency);
    assert_eq!(price_metadata.get_specific_currency(), specific_currency);
}
