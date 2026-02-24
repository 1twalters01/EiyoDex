Note: Will consider using uuids instead of Rc<RefCell<T>> and Weak<RefCell<T>>
Note: Will consider using Vec instead of HashSet


FoodCategory - Animal -> Poultry -> Chicken
FoodTaxonomy - The food in question e.g. Chicken leg
```rust
enum FoodCategoryChild {
    FoodCategory(Rc<RefCell<FoodCategory>>),
    FoodTaxonomy(Rc<RefCell<FoodTaxonomy>>),
}

struct FoodCategory {
    id: Uuid,
    name: String,
    description: String,
    parent: Weak<RefCell<FoodCategory>,
    children: Vec<Rc<RefCell<FoodCategoryChild>>>,
}

struct FoodTaxonomy {
    id: Uuid,
    name: String,
    description: String,
    price_metadata: Vec<PriceMetadata>
    parent: Weak<RefCell<FoodCategory>>,

}
```

PriceMetadata - Metadata about the price of a FoodTaxonomy item
Merchant - The seller of the FoodTaxonomy item e.g. tesco
```rust
enum ItemQuantity {
    SpecificCurrency(SpecificCurrencyQuantity), // price/mass or price/volume enum
    PricePerCount(PricePerCount) // price per n amount e.g. price per 12 eggs
}

struct PricePerCount {
    price: Currency
    count: f64 // Use decimal instead?
}

struct PriceMetadata {
    merchant: Rc<RefCell<Merchant>>,
    specific_currency: Option<ItemQuantity>, 
    timestamp: DateTime
}

pub struct Merchant {
    id: Uuid,
    name: String,
    description: String,
    website: String,
}
```

DataSource - The food nutrition data provider e.g. USDA or NCCDB
FoodNutritionData - The food nutrition data in question
```rust
pub struct DataSource {
    id: Uuid,
    name: String,
    description: String,
}

pub struct DataSourceInstance {
    data_source: Rc<RefCell<DataSource>>,
    version: String, // Could create a Version type
    food_nutrition_data: HashSet<Rc<RefCell<FoodNutritionData>>>,
}

pub struct FoodNutritionData {
    id: Uuid,
    data_source_instance: Rc<RefCell<DataSourceInstance>>,
    description: String,
    nutrient_quantity_list: NutrientQuantityList,
}
```

Preparation Method - How it is cooked, e.g. baked, fried, deep fried, deep fried and for how long
FoodAttribute - What is done to it, e.g. skin removed, skin eaten, organic, e.g.
FoodTags - Tags a user might want to put on things
```rust
pub struct PreparationMethod {
    id: Uuid,
    name: String,
    description: String,
}

pub struct FoodAttribute {
    id: Uuid,
    name: String,
    description: String,
}

pub struct FoodTags {
    id: Uuid,
    name: String,
    description: String,
}
```

FoodVariant - The specific item the user is eating e.g. baked chicken leg, no skin
```rust
struct FoodVariant {
    id: Uuid,
    name: String,
    description: String,
    preparation_method: Rc<RefCell<PreparationMethod>>,
    food_attribute: HashSet<Rc<RefCell<FoodAttribute>>>,
    food_tags: Vec<Rc<RefCell<FoodTags>>>,
}
```

FoodInstance - A specific instance of a food
```rust
struct FoodInstance {
    id: Uuid,
    food_variant: FoodVariant,
    data_source_instance: DataSourceInstance,
}
```

FoodQuantity - The amount of food
```rust
struct FoodQuantity {
    value: f64,
    food_variant: Rc<RefCell<FoodVariant>,
    data_source_instance: Rc<RefCell<DataSourceInstance>>,
    consumed_at: DateTime<Utc>,
}
```

FoodQuantityList - A list of food quantities e.g. in a recipe
```rust
struct FoodQuantityList {
    food_quantities: Vec<FoodQuantity>
}
```
