#[derive(Debug, PartialEq, Clone)]
pub struct FoodCategoryRecord {
    id: Vec<u8>,
    name: String,
    description: String,
}

impl FoodCategoryRecord {
    pub fn from_values(id: Vec<u8>, name: String, description: String) {
        Self {id, name, description}
    }

    pub fn from_food_category_entity(food_category_entity: Entity<FoodCategory>) {}
  
    pub fn from_food_category(food_category: FoodCategory) {}

    pub fn to_food_category(&self) -> FoodCategory {}

    pub fn to_food_category_entity(&self) -> Entity<FoodCategory> {}

    pub async fn load_from_database(id: Vec<u8>, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {}

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<Vec<u8>, sqlx::Error> {}

    pub async fn delete_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {}
}

pub FoodCategoryRelationshipRecord {
    parent_category_id: Vec<u8>,
    child_id: Vec<u8>,
    child_type_id: i64,
}

impl FoodCategoryRelationshipRecord {
    pub fn from_values(parent_category_id: Vec<u8>, child_id: Vec<u8>, child_type_id: i64) {
        Self { parent_category_id, child_id, child_type_id }
    }
}
