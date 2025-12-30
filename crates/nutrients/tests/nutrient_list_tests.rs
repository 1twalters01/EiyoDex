use nutrients::{
    nutrient::{link_parent_child, Nutrient}, nutrient_amount::NutrientAmount, nutrient_list::NutrientAmountList, units::NutrientUnit
};
use units::mass::MassUnit;

#[test]
fn test_sum_descendants_vec() {
    let id = None;

    // Create iron, heme iron and non-heme iron
    let value_1 = 15f64;
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

    let value_2 = 7.5;
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

    let value_3 = 2.5;
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

    let value_4 = 10f64;
    let non_heme_iron_a = Nutrient::new_rc_refcell(
        id,
        String::from("Non-heme Iron A"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_amount_a: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_4,
        Some(non_heme_iron.clone()),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_5 = 25f64;
    let non_heme_iron_b = Nutrient::new_rc_refcell(
        id,
        String::from("Non-heme Iron"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_amount_b: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_5,
        Some(non_heme_iron.clone()),
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

    let mineral_vec = Vec::from([iron_amount, heme_iron_amount, non_heme_iron_amount, non_heme_iron_amount_a, non_heme_iron_amount_b, potassium_amount]);
    let minerals = NutrientAmountList::from_vec(mineral_vec);

    let iron_sum = minerals.sum_amounts_from_descendants_rc_refcell(iron);
    println!("iron_sum: {:#?}", iron_sum);
    assert_eq!(iron_sum.get_value(), value_2 + value_3 + value_4 + value_5);
}
