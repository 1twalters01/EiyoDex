# Foods
Crate for handling food items

## Definitions
Food Quantity = An instance and amount of food. Has the data source attached to determine what food instance is being used.
Data Source = The source of the data e.g. NCCBD or USDA
Food = What you are having. Is a DAG.
Food Instance = What is done to it before eating e.g. hard boiled, soft boiled, etc.
Food Nutrition Data = The nutrients in a particular food instance
Food Tag = Any tags the user may want to place on the food
Food Tag Categories = Categories you can put your food tags in. Is a DAG
Price Metadata = Metadata on the price of a particular food instance
Merchant = The provider of the price metadata e.g. Tesco

## Example
Animal - Food | Children = Poultry, Red Meat, etc.
-> Poultry - Food | Children = Chicken, Turkey, etc.
---> Chicken - Food| Children = Chicken leg, Chicken breast, etc.
------> Chicken leg - Food | Children = Organic Chicken, Non Organic Chicken, etc.
---------> Organic Chicken - Food | Children = Baked, Fried, Deep Fried, etc.
---------> Baked Organic Chicken - Food Instance


## Food
Metadata: {
    id: uuid4
    name: String
    subset_of: superset_id (uuid)
}
Data: {
    Source: {
        source_id: uuid
    }
    Tags: Tag_id uuid4
    pral alkalinity: {
        per: {
            units: Unit
            size: f64
        }
        pral alkalinity: f64
    }
    nutrients: {
        per: {
            units: Unit
            size: f64
        }
        nutrients: nutrient_id[]
        nutrient amount: f64
    }
}[]


## Tag
id: uuid
name: String
subset of: tag_id | None
