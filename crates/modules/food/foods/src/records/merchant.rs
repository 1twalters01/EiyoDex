pub struct MerchantRecord {
    id: Vec<u8>,
    name: String,
    description: String,
    website: Option<String>,
}

impl MerchantRecord {
    pub fn from_values(id: Vec<u8, name: String, description: String, website: Option<String>) -> Self {
        Self { id, name, description, website }
    }

    pub fn from_merchant_record(merchant: Merchant) -> Self {
        let id = Id::<Merchant>::new(InnerIdType::Uuid).to_bytes().to_vec();
        let name = merchant.get_name();
        let description = merchant.get_desription();
        let website = merchnat.get_website();
        Self { id, name, description, website }
    }

    pub to_merchant(&self) -> Merchant {}
    




    pub async fn load_from_database_using_name() {}
    pub async fn load_from_database_using_id() {}
    pub async fn save_to_database() {}
    pub async fn delete_merchant_from_database() {}
}
