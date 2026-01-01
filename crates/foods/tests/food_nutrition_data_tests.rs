use foods::{data_sources::DataSource, food_nutrition_data::FoodNutritionData};
use nutrients::{
    nutrient::Nutrient, nutrient_amount::NutrientAmount, nutrient_list::NutrientAmountList,
    units::NutrientUnit,
};
use units::mass::MassUnit;
use uuid::Uuid;

#[test]
pub fn test_id() {
    let iron_value = 5f64;
    let potassium_value = 10f64;
    let iron_id = None;
    let potassium_id = None;

    let iron: NutrientAmount = NutrientAmount::from_rc_refcell(
        iron_value,
        Some(Nutrient::new_rc_refcell(
            iron_id,
            String::from("Iron"),
            NutrientUnit::Mass(MassUnit::Milligram),
        )),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let potassium: NutrientAmount = NutrientAmount::from_rc_refcell(
        potassium_value,
        Some(Nutrient::new_rc_refcell(
            potassium_id,
            String::from("Potassium"),
            NutrientUnit::Mass(MassUnit::Milligram),
        )),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let nutrient_amount_list = NutrientAmountList::from_vec(Vec::from([iron, potassium]));

    let data_source_id = None;
    let data_source_name = String::from("NCCDB");
    let data_source = DataSource::new(data_source_id, data_source_name.clone());

    let food_nutrition_data_id = Uuid::from_u128(183u128);
    let mut food_nutrition_data = FoodNutritionData::new(data_source, nutrient_amount_list);

    assert_ne!(food_nutrition_data.get_id(), food_nutrition_data_id);
    food_nutrition_data.set_id(food_nutrition_data_id);
    assert_eq!(food_nutrition_data.get_id(), food_nutrition_data_id);
}

#[test]
pub fn test_data_source() {
    let iron_value = 5f64;
    let potassium_value = 10f64;
    let iron_id = None;
    let potassium_id = None;

    let iron: NutrientAmount = NutrientAmount::from_rc_refcell(
        iron_value,
        Some(Nutrient::new_rc_refcell(
            iron_id,
            String::from("Iron"),
            NutrientUnit::Mass(MassUnit::Milligram),
        )),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let potassium: NutrientAmount = NutrientAmount::from_rc_refcell(
        potassium_value,
        Some(Nutrient::new_rc_refcell(
            potassium_id,
            String::from("Potassium"),
            NutrientUnit::Mass(MassUnit::Milligram),
        )),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let nutrient_amount_list = NutrientAmountList::from_vec(Vec::from([iron, potassium]));

    let data_source_id = None;
    let data_source_name_1 = String::from("NCCDB");
    let data_source_name_2 = String::from("USDA");
    let data_source_1 = DataSource::new(data_source_id, data_source_name_1.clone());
    let data_source_2 = DataSource::new(data_source_id, data_source_name_2.clone());

    let mut food_nutrition_data =
        FoodNutritionData::new(data_source_1.clone(), nutrient_amount_list);

    assert_eq!(food_nutrition_data.get_data_source(), data_source_1);
    assert_ne!(food_nutrition_data.get_data_source(), data_source_2);
    food_nutrition_data.set_data_source(data_source_2.clone());
    assert_ne!(food_nutrition_data.get_data_source(), data_source_1);
    assert_eq!(food_nutrition_data.get_data_source(), data_source_2);
}

#[test]
pub fn test_nutrient_amount_list() {
    let iron_value = 5f64;
    let potassium_value = 10f64;
    let iron_id = None;
    let potassium_id = None;

    let iron: NutrientAmount = NutrientAmount::from_rc_refcell(
        iron_value,
        Some(Nutrient::new_rc_refcell(
            iron_id,
            String::from("Iron"),
            NutrientUnit::Mass(MassUnit::Milligram),
        )),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let potassium: NutrientAmount = NutrientAmount::from_rc_refcell(
        potassium_value,
        Some(Nutrient::new_rc_refcell(
            potassium_id,
            String::from("Potassium"),
            NutrientUnit::Mass(MassUnit::Milligram),
        )),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let nutrient_amount_list_id = Uuid::from_u128(0u128);
    let mut nutrient_amount_list_1 = NutrientAmountList::from_vec(Vec::from([iron.clone()]));
    let mut nutrient_amount_list_2 = NutrientAmountList::from_vec(Vec::from([potassium.clone()]));
    let mut nutrient_amount_list_3 =
        NutrientAmountList::from_vec(Vec::from([iron.clone(), potassium.clone()]));
    nutrient_amount_list_1.set_id(nutrient_amount_list_id);
    nutrient_amount_list_2.set_id(nutrient_amount_list_id);
    nutrient_amount_list_3.set_id(nutrient_amount_list_id);

    let data_source_id = None;
    let data_source_name = String::from("NCCDB");
    let data_source = DataSource::new(data_source_id, data_source_name.clone());

    let mut food_nutrition_data =
        FoodNutritionData::new(data_source, nutrient_amount_list_1.clone());

    assert_eq!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_1
    );
    assert_ne!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_2
    );

    food_nutrition_data.set_nutrient_amount_list(nutrient_amount_list_2.clone());
    assert_ne!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_1
    );
    assert_eq!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_2
    );

    food_nutrition_data.add_nutrient_amount(iron.clone());
    assert_ne!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_1
    );
    assert_ne!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_2
    );
    assert_eq!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_3
    );

    food_nutrition_data.remove_nutrient(&potassium.clone());
    assert_eq!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_1
    );
    assert_ne!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_2
    );
    assert_ne!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_3
    );

    food_nutrition_data.remove_nutrient(&iron);
    assert_ne!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_1
    );

    let nutrient_amount_vec = Vec::from([iron, potassium]);
    food_nutrition_data.extend_nutrient_amounts(nutrient_amount_vec);
    assert_eq!(
        food_nutrition_data.get_nutrient_amount_list(),
        nutrient_amount_list_3
    );
}
