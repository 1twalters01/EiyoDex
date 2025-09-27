use foods::price_metadata::{Merchant, PriceMetadata};
use units::specific_currency::{SpecificCurrency, SpecificCurrencyUnit};
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;

#[test]
pub fn test_merchant() {
    let name = String::from("Butcher Y");
    let id = None;
    let mut merchant = Merchant::new(name, id);

    assert_eq!(id, None);
    let new_id = Uuid::new_v4();
    merchant.set_id(new_id);
    assert_eq!(merchant.get_id(), new_id);

    assert_eq!(merchant.get_name(), String::from("Butcher Y"));
    merchant.set_name(String::from("Supermarket Z"));
    assert_eq!(merchant.get_name(), String::from("Supermarket Z"));

    assert_eq!(merchant.get_description(), String::from(""));
    let description = String::from("Merchant description");
    merchant.set_description(description);
    assert_eq!(
        merchant.get_description(),
        String::from("Merchant description")
    );

    assert_eq!(merchant.get_website(), String::from(""));
    let website = String::from("merchantsite.com");
    merchant.set_website(website);
    assert_eq!(merchant.get_website(), String::from("merchantsite.com"));
}

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
    specific_currency = Some(SpecificCurrency::new(
        value,
        unit,
    ));
    price_metadata.set_specific_currency(specific_currency);
    assert_eq!(price_metadata.get_specific_currency(), specific_currency);
}
