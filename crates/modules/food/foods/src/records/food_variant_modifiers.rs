struct PreparationMethodRecord {
    id: Vec<u8>,
    name: String,
    description: String,
}

impl PreparationMethodRecord {
    pub fn from_values(
        id: Vec<u8>,
        name: String,
        description: String,
    ) -> Self {
        Self { id, name, description }
    }

    pub fn from_preparation_method(
        preparation_method: PreparationMethod
    ) -> Self {
        let id = preparation_method.get_id();
        let name = preparation_method.get_name();
        let description = preparation_method.get_description();
        Self { id, name, description }
    }




    pub async fn load_from_database_using_name() {}
    pub async fn load_from_database_using_id() {}
    pub async fn save_to_database() {}
    pub async fn delete_preparation_method_from_database() {}
}
