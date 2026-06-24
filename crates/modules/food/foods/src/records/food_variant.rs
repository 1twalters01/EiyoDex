pub struct FoodVariantRecord {
    id: Vec<u8>,
    name: String,
    description: String,
    preparation_method_id: Vec<u8>,
}

impl FoodVariantRecord {
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

pub struct FoodVariantFoodAttributeRelationshipRecord {
    food_variant_id: Vec<u8>,
    food_attribute_id: Vec<u8>,
}

impl FoodVariantFoodAttributeRelationshipRecord {
    pub fn from_values(food_variant_id: Vec<u8>, food_attribute_id: Vec<u8>) {
        Self { food_variant_id, food_attribute_id }
    }
}

pub struct FoodVariantFoodTagRelationshipRecord {
    food_variant_id: Vec<u8>,
    food_tag_id: Vec<u8>,
}

impl FoodVariantFoodTagRelationshipRecord {
    pub fn from_values(food_variant_id: Vec<u8>, food_tag_id: Vec<u8>) {
        Self { food_variant_id, food_tag_id }
    }
}

pub struct FoodVariantFoodInstanceRelationshipRecord {
    food_variant_id: Vec<u8>,
    food_instance_id: Vec<u8>,
}

impl FoodVariantFoodTagRelationshipRecord {
    pub fn from_values(food_variant_id: Vec<u8>, food_instance_id: Vec<u8>) {
        Self { food_variant_id, foo_instance_id }
    }
}

