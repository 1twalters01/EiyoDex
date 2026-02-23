use nutrients::{
    nutrient::{link_parent_child, Nutrient},
    nutrient_amount::NutrientAmount,
    nutrient_amount_list::NutrientAmountList,
    schema::{
        nutrient_classes::{ChemicalType, EssentialityType, QuantityType},
        nutrient_type::NutrientType,
    },
    units::NutrientUnit,
};
use units::mass::unit::MassUnit;
use uuid::Uuid;

#[test]
fn test_id_funcs() {
    let iron_id = None;
    let potassium_id = None;
    let nutrient_list_id = Uuid::from_u128(15u128);
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };

    let value = 15f64;

    let iron: NutrientAmount = NutrientAmount::from_rc_refcell(
        value,
        Nutrient::new_rc_refcell(
            iron_id,
            String::from("Iron"),
            nutrient_type.clone(),
            NutrientUnit::Mass(MassUnit::Milligram),
        ),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let potassium: NutrientAmount = NutrientAmount::from_rc_refcell(
        value,
        Nutrient::new_rc_refcell(
            potassium_id,
            String::from("Potassium"),
            nutrient_type.clone(),
            NutrientUnit::Mass(MassUnit::Milligram),
        ),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let mut nutrient_amount_list = NutrientAmountList::from_vec(Vec::from([iron, potassium]));

    assert_ne!(nutrient_amount_list.get_id(), nutrient_list_id);
    nutrient_amount_list.set_id(nutrient_list_id);
    assert_eq!(nutrient_amount_list.get_id(), nutrient_list_id);
}

#[test]
fn test_push_and_remove_from_nutrient_list() {
    let iron_id = None;
    let potassium_id = None;
    let nutrient_list_id = Uuid::from_u128(15u128);
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };

    let value = 15f64;

    let iron: NutrientAmount = NutrientAmount::from_rc_refcell(
        value,
        Nutrient::new_rc_refcell(
            iron_id,
            String::from("Iron"),
            nutrient_type.clone(),
            NutrientUnit::Mass(MassUnit::Milligram),
        ),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let potassium: NutrientAmount = NutrientAmount::from_rc_refcell(
        value,
        Nutrient::new_rc_refcell(
            potassium_id,
            String::from("Potassium"),
            nutrient_type.clone(),
            NutrientUnit::Mass(MassUnit::Milligram),
        ),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let mut nutrient_amount_list_iron = NutrientAmountList::from_vec(Vec::from([iron.clone()]));
    nutrient_amount_list_iron.set_id(nutrient_list_id);
    let mut nutrient_amount_list_potassium =
        NutrientAmountList::from_vec(Vec::from([potassium.clone()]));
    nutrient_amount_list_potassium.set_id(nutrient_list_id);
    let mut nutrient_amount_list_iron_and_potassium =
        NutrientAmountList::from_vec(Vec::from([iron.clone(), potassium.clone()]));
    nutrient_amount_list_iron_and_potassium.set_id(nutrient_list_id);
    let mut nutrient_amount_list_potassium_and_iron =
        NutrientAmountList::from_vec(Vec::from([potassium.clone(), iron.clone()]));
    nutrient_amount_list_potassium_and_iron.set_id(nutrient_list_id);

    let mut nutrient_amount_list = NutrientAmountList::from_vec(Vec::from([iron.clone()]));
    nutrient_amount_list.set_id(nutrient_list_id);

    nutrient_amount_list.push(iron.clone());
    assert_eq!(nutrient_amount_list, nutrient_amount_list_iron);

    nutrient_amount_list.push(potassium);
    assert_eq!(
        nutrient_amount_list,
        nutrient_amount_list_iron_and_potassium
    );
    assert_eq!(
        nutrient_amount_list,
        nutrient_amount_list_potassium_and_iron
    );

    nutrient_amount_list.remove(&iron);
    assert_eq!(nutrient_amount_list, nutrient_amount_list_potassium);
}

#[test]
fn test_sum_ascendants_vec() {}

#[test]
fn test_sum_descendants_vec() {
    let id = None;
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };

    // Create iron, heme iron and non-heme iron
    let value_1 = 1f64;
    let iron = Nutrient::new_rc_refcell(
        id,
        String::from("Iron"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let iron_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_1,
        iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_2 = 10f64;
    let heme_iron = Nutrient::new_rc_refcell(
        id,
        String::from("Heme Iron"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let heme_iron_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_2,
        heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_3 = 100f64;
    let non_heme_iron = Nutrient::new_rc_refcell(
        id,
        String::from("Non-heme Iron"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_3,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_4 = 1_000f64;
    let non_heme_iron_a = Nutrient::new_rc_refcell(
        id,
        String::from("Non-heme Iron A"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_amount_a: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_4,
        non_heme_iron_a.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_5 = 10_000f64;
    let non_heme_iron_b = Nutrient::new_rc_refcell(
        id,
        String::from("Non-heme Iron B"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_amount_b: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_5,
        non_heme_iron_b.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_6 = 100_000f64;
    let potassium = Nutrient::new_rc_refcell(
        id,
        String::from("Potassium"),
        nutrient_type,
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let potassium_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_6,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    // Link parents and children
    let _ = link_parent_child(&iron, &heme_iron);
    let _ = link_parent_child(&iron, &non_heme_iron);
    let _ = link_parent_child(&non_heme_iron, &non_heme_iron_a);
    let _ = link_parent_child(&non_heme_iron, &non_heme_iron_b);

    let mineral_vec = Vec::from([
        iron_amount,
        heme_iron_amount,
        non_heme_iron_amount,
        non_heme_iron_amount_a,
        non_heme_iron_amount_b,
        potassium_amount,
    ]);
    let minerals = NutrientAmountList::from_vec(mineral_vec);
    println!("{:#?}", minerals.get_nutrient_names());

    let iron_sum = minerals.sum_amounts_from_descendants_rc_refcell(iron);
    println!("iron_sum: {:#?}", iron_sum);
    println!("manual sum: {:#?}", value_2 + value_3 + value_4 + value_5);
    assert_eq!(iron_sum.get_value(), value_2 + value_3 + value_4 + value_5);
}
