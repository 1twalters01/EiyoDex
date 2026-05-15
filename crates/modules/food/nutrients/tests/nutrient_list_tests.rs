use std::{cell::RefCell, rc::Rc};

use nutrients::{
    nutrient::{link_parent_child, Nutrient}, nutrient_list::NutrientList, nutrient_units::NutrientUnit, schema::{
        nutrient_classes::{ChemicalType, EssentialityType, QuantityType},
        nutrient_type::NutrientType,
    }
};
use units::mass::unit::MassUnit;
use uuid::Uuid;

#[test]
fn test_id_funcs() {
    let nutrient_list_id = Uuid::from_u128(15u128);
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };

    let value = 15f64;

    let iron: Rc<RefCell<Nutrient>> = Nutrient::new_rc_refcell(
        String::from("Iron"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let potassium: Rc<RefCell<Nutrient>> = Nutrient::new_rc_refcell(
        String::from("Potassium"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let mut nutrient_list = NutrientList::from_vec(Vec::from([iron, potassium]));

    assert_ne!(nutrient_list.get_id(), nutrient_list_id);
    nutrient_list.set_id(nutrient_list_id);
    assert_eq!(nutrient_list.get_id(), nutrient_list_id);
}

#[test]
fn test_push_and_remove_from_nutrient_list() {
    let nutrient_list_id = Uuid::from_u128(15u128);
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };

    let value = 15f64;

    let iron: Rc<RefCell<Nutrient>> = Nutrient::new_rc_refcell(
        String::from("Iron"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let potassium: Rc<RefCell<Nutrient>> = Nutrient::new_rc_refcell(
        String::from("Potassium"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let mut nutrient_list_iron = NutrientList::from_vec(Vec::from([iron.clone()]));
    nutrient_list_iron.set_id(nutrient_list_id);

    let mut nutrient_list_potassium =
        NutrientList::from_vec(Vec::from([potassium.clone()]));
    nutrient_list_potassium.set_id(nutrient_list_id);

    let mut nutrient_list_iron_and_potassium =
        NutrientList::from_vec(Vec::from([iron.clone(), potassium.clone()]));
    nutrient_list_iron_and_potassium.set_id(nutrient_list_id);

    let mut nutrient_list_potassium_and_iron =
        NutrientList::from_vec(Vec::from([potassium.clone(), iron.clone()]));
    nutrient_list_potassium_and_iron.set_id(nutrient_list_id);

    let mut nutrient_list = NutrientList::from_vec(Vec::from([iron.clone()]));
    nutrient_list.set_id(nutrient_list_id);
    assert_eq!(nutrient_list, nutrient_list_iron);

    nutrient_list.push(potassium);

    nutrient_list.sort_by_name();
    nutrient_list_iron_and_potassium.sort_by_name();
    nutrient_list_potassium_and_iron.sort_by_name();
    assert_eq!(
        nutrient_list,
        nutrient_list_iron_and_potassium
    );
    assert_eq!(
        nutrient_list,
        nutrient_list_potassium_and_iron
    );

    println!("{:#?}", nutrient_list);
    nutrient_list.remove(iron);
    println!("{:#?}", nutrient_list);
    assert_eq!(nutrient_list, nutrient_list_potassium);
}

