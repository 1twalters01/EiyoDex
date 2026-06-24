#[derive(Debug, PartialEq, Clone)]
pub struct FoodTaxonomyRecord {
    id: Vec<u8>,
    name: String,
    description: String,
    parent_category_id: Vec<u8>,
}

impl FoodTaxonomyRecord {
    pub fn from_values(id: Vec<u8>, name: String, description: String) {
        Self { id, name, description }
    }

    pub fn from_food_taxonomy_entity(food_taxonomy_entity: Entity<FoodTaxonomy>) {}
  
    pub fn from_food_taxonomy(food_category: FoodTaxonomy) {}

    pub fn to_food_taxonomy(&self) -> FoodTaxonomy {}

    pub fn to_food_taxonomy_entity(&self) -> Entity<FoodTaxonomy> {}

    pub async fn load_from_database(id: Vec<u8>, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {}

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<Vec<u8>, sqlx::Error> {}

    pub async fn delete_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {}
}

pub struct FoodTaxonomyFoodVariantRelationshipRecord {
    food_taxonomy_id: Vec<u8>,
    food_variant_id: Vec<u8>,
}

impl FoodCategoryRelationshipRecord {
    pub fn from_values(food_taxonomy_id: Vec<u8>, food_variant_id: Vec<u8>) {
        Self { food_taxonomy_id, food_variant_id }
    }
}

