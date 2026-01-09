use std::{cell::RefCell, rc::Rc};
use units::specific_currency::SpecificCurrency;

use crate::merchant::Merchant;

#[derive(Clone, PartialEq)]
pub struct PriceMetadata {
    merchant: Rc<RefCell<Merchant>>,
    specific_currency: Option<SpecificCurrency>,
}

impl PriceMetadata {
    pub fn new(merchant: Merchant, specific_currency: Option<SpecificCurrency>) -> Self {
        PriceMetadata {
            merchant: Rc::new(RefCell::new(merchant)),
            specific_currency: specific_currency,
        }
    }

    pub fn get_merchant(&self) -> Rc<RefCell<Merchant>> {
        self.merchant.clone()
    }

    pub fn set_merchant(&mut self, merchant: Merchant) {
        self.merchant = Rc::new(RefCell::new(merchant));
    }

    pub fn get_specific_currency(&self) -> Option<SpecificCurrency> {
        self.specific_currency
    }

    pub fn set_specific_currency(&mut self, specific_currency: Option<SpecificCurrency>) {
        self.specific_currency = specific_currency;
    }
}

