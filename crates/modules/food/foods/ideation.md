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
    parent: Option<Weak<RefCell<FoodCategory>>>,
    children: Vec<Rc<RefCell<FoodCategoryChild>>>,
}

struct FoodTaxonomy {
    id: Uuid,
    name: String,
    description: String,
    price_metadata: Vec<Rc<RefCell<PriceMetadata>>>,
    parent: Weak<RefCell<FoodCategory>>,
    children: Vec<Rc<RefCell<FoodVariants>>>,
}
```

PriceMetadata - Metadata about the price of a FoodTaxonomy item
```rust
enum ItemQuantity {
    SpecificCurrency(SpecificCurrencyQuantity), // price/mass or price/volume enum
    PricePerCount(PricePerCount) // price per n amount e.g. price per 12 eggs
}

struct PricePerCount {
    price: Currency,
    count: f64, // Use decimal instead?
}

struct PriceMetadata {
    merchant: Rc<RefCell<Merchant>>,
    item_quantity: Option<ItemQuantity>, 
    timestamp: DateTime,
}
```

Merchant - The seller of the FoodTaxonomy item e.g. tesco
```rust
pub struct Merchant {
    id: Uuid,
    name: String,
    description: String,
    website: String,
}
```

DataSource - The food nutrition data provider e.g. USDA or NCCDB
DataSourceVersion - The version of the data source
DataSourceInstance - The food nutrition data in question
```rust
pub struct DataSourceProvider {
    id: Uuid,
    name: String,
    description: String,
}

pub struct DataSourceVersion {
    version: String,
}

pub struct DataSourceInstance {
    data_source_provider: Rc<RefCell<DataSourceProvider>>,
    data_source_version: Rc<RefCell<DataSourceVersion>>,
    description: String,
    nutrient_quantity_list: Rc<RefCell<NutrientQuantityList>>,
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
FoodInstance - A specific instance of a food
FoodQuantity - The amount of food
```rust
struct FoodVariant {
    id: Uuid,
    name: String,
    description: String,
    preparation_method: Rc<RefCell<PreparationMethod>>,
    food_attribute: HashSet<Rc<RefCell<FoodAttribute>>>,
    food_tags: Vec<Rc<RefCell<FoodTags>>>,
    food_instances: Vec<Rc<RefCell<FoodInstance>>>,
    parent: Weak<RefCell<FoodTaxonomy>>,
}

struct FoodInstance {
    id: Uuid,
    food_variant: Weak<RefCell<FoodVariant>>,
    data_source_instance: DataSourceInstance,
}

struct FoodQuantity {
    value: f64,
    food_variant: Rc<RefCell<FoodVariant>>,
    data_source: Rc<RefCell<DataSourceProvider>>,
    data_source_version: DataSourceVersion,
    data_source_instance: Rc<RefCell<DataSourceInstance>>,
    consumed_at: DateTime<Utc>,
}
```

Recipe - A list of food quantities
```rust
struct Recipe {
    food_quantities: Vec<FoodQuantity>
}
```
