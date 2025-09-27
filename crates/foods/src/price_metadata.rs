use units::specific_currency::SpecificCurrency;
use uuid::Uuid;

pub struct PriceMetadata {
    merchant: Merchant,
    specific_currency: SpecificCurrency,
}

impl PriceMetadata {
    pub fn get_merchant(&self) -> Merchant {
        self.merchant.clone()
    }

    pub fn set_merchant(&mut self, merchant: Merchant) {
        self.merchant = merchant;
    }

    pub fn get_specific_currency(&self) -> SpecificCurrency {
        self.specific_currency
    }

    pub fn set_specific_currency(&mut self, specific_currency: SpecificCurrency) {
        self.specific_currency = specific_currency;
    }
}

#[derive(Clone)]
pub struct Merchant {
    id: Uuid,
    name: String,
    description: String,
    link: String,
}

impl Merchant {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_link(&self) -> String {
        self.link.clone()
    }

    pub fn set_link(&mut self, link: String) {
        self.link = link;
    }
}
