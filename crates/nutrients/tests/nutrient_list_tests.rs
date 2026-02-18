use nutrients::{
    nutrient::{link_parent_child, Nutrient},
    nutrient_amount::NutrientAmount,
    nutrient_list::NutrientAmountList,
    units::NutrientUnit,
};
use units::mass_unit::MassUnit;
use uuid::Uuid;

#[test]
fn test_id_funcs() {
    let iron_id = None;
    let potassium_id = None;
    let nutrient_list_id = Uuid::from_u128(15u128);

    let value = 15f64;

    let iron: NutrientAmount = NutrientAmount::from_rc_refcell(
        value,
        Some(Nutrient::new_rc_refcell(
            iron_id,
            String::from("Iron"),
            NutrientUnit::Mass(MassUnit::Milligram),
        )),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let potassium: NutrientAmount = NutrientAmount::from_rc_refcell(
        value,
        Some(Nutrient::new_rc_refcell(
            potassium_id,
            String::from("Potassium"),
            NutrientUnit::Mass(MassUnit::Milligram),
        )),
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

    let value = 15f64;

    let iron: NutrientAmount = NutrientAmount::from_rc_refcell(
        value,
        Some(Nutrient::new_rc_refcell(
            iron_id,
            String::from("Iron"),
            NutrientUnit::Mass(MassUnit::Milligram),
        )),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let potassium: NutrientAmount = NutrientAmount::from_rc_refcell(
        value,
        Some(Nutrient::new_rc_refcell(
            potassium_id,
            String::from("Potassium"),
            NutrientUnit::Mass(MassUnit::Milligram),
        )),
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

    // Create iron, heme iron and non-heme iron
    let value_1 = 1f64;
    let iron = Nutrient::new_rc_refcell(
        id,
        String::from("Iron"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let iron_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_1,
        Some(iron.clone()),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_2 = 10f64;
    let heme_iron = Nutrient::new_rc_refcell(
        id,
        String::from("Heme Iron"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let heme_iron_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_2,
        Some(heme_iron.clone()),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_3 = 50f64;
    let non_heme_iron = Nutrient::new_rc_refcell(
        id,
        String::from("Non-heme Iron"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_3,
        Some(non_heme_iron.clone()),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_4 = 100f64;
    let non_heme_iron_a = Nutrient::new_rc_refcell(
        id,
        String::from("Non-heme Iron A"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_amount_a: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_4,
        Some(non_heme_iron_a.clone()),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_5 = 1000f64;
    let non_heme_iron_b = Nutrient::new_rc_refcell(
        id,
        String::from("Non-heme Iron B"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_amount_b: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_5,
        Some(non_heme_iron_b.clone()),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_6 = 50f64;
    let potassium = Nutrient::new_rc_refcell(
        id,
        String::from("Potassium"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let potassium_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_6,
        Some(potassium.clone()),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    // Link parents and children
    link_parent_child(&iron, &heme_iron);
    link_parent_child(&iron, &non_heme_iron);
    link_parent_child(&non_heme_iron, &non_heme_iron_a);
    link_parent_child(&non_heme_iron, &non_heme_iron_b);

    let mineral_vec = Vec::from([
        iron_amount,
        heme_iron_amount,
        non_heme_iron_amount,
        non_heme_iron_amount_a,
        non_heme_iron_amount_b,
        potassium_amount,
    ]);
    let minerals = NutrientAmountList::from_vec(mineral_vec);
    println!("{:#?}", minerals.get_names());

    let iron_sum = minerals.sum_amounts_from_descendants_rc_refcell(iron);
    println!("iron_sum: {:#?}", iron_sum);
    assert_eq!(iron_sum.get_value(), value_2 + value_3 + value_4 + value_5);
}
