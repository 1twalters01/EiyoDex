use nutrients::{
    nutrient::{link_parent_child, unlink_parent_child, Nutrient},
    nutrient_units::NutrientUnit,
    schema::{
        nutrient_classes::{ChemicalType, EssentialityType, QuantityType},
        nutrient_type::NutrientType,
    },
};
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};
use units::{mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::dsa::node::GraphNode;
use uuid::Uuid;

#[test]
fn test_nutrient_new_nutrient_unit() {
    let id = None;
    let name = String::from("Potassium");
    let description = "Test description".to_string();
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let mut nutrient = Nutrient::new(id, name, nutrient_type, main_unit);
    let mut nutrient_2 = nutrient.clone();
    assert_eq!(nutrient, nutrient_2);

    nutrient_2.set_description(description.clone());
    assert_ne!(nutrient, nutrient_2);

    nutrient.set_description(description);
    assert_eq!(nutrient, nutrient_2);
}

#[test]
fn test_nutrient_new_rc_refcell_nutrient_unit() {
    let id = None;
    let name = String::from("Potassium");
    let description = "Test description".to_string();
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let nutrient = Nutrient::new_rc_refcell(id, name, nutrient_type, main_unit);
    let nutrient_2 = nutrient.clone();
    assert_eq!(nutrient, nutrient_2);

    nutrient.borrow_mut().set_description(description);
    assert_eq!(nutrient, nutrient_2);
}

#[test]
fn test_nutrient_unit_id() {
    let id: Uuid = Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128);
    let id2: Uuid = Uuid::from_u128(0xa1a2a3a4b1b2c1c2c3c4d1d2d3d4e1e2u128);
    let name = String::from("Potassium");
    let nutrient_type = NutrientType {
        essentiality_type: Some(EssentialityType::Essential),
        quantity_type: QuantityType::Micronutrient,
        chemical_type: ChemicalType::Mineral,
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let mut nutrient = Nutrient::new(Some(id), name, nutrient_type, main_unit);

    assert_eq!(nutrient.get_id(), id);
    nutrient.set_id(id2);
    assert_eq!(nutrient.get_id(), id2);
}

#[test]
fn test_nutrient_unit_name() {
    let id: Uuid = Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128);
    let name = String::from("Potassium");
    let name2 = String::from("Calcium");
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let mut nutrient = Nutrient::new(Some(id), name.clone(), nutrient_type, main_unit);

    assert_eq!(nutrient.get_name(), name);
    nutrient.set_name(name2.clone());
    assert_eq!(nutrient.get_name(), name2);
}

#[test]
fn test_nutrient_unit_description() {
    let id = None;
    let name = String::from("Potassium");
    let description = "Test description".to_string();
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let mut nutrient = Nutrient::new(id, name, nutrient_type, main_unit);

    assert_eq!(nutrient.get_description(), String::new());
    nutrient.set_description(description.clone());
    assert_eq!(nutrient.get_description(), description);
}

#[test]
fn test_nutrient_unit_categories() {
    let id = None;
    let name = String::from("Potassium");
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let nutrient = Nutrient::new_rc_refcell(id, name, nutrient_type, main_unit);

    assert_eq!(
        nutrient.borrow().get_nutrient_type(),
        NutrientType {
            chemical_type: ChemicalType::Mineral,
            quantity_type: QuantityType::Micronutrient,
            essentiality_type: Some(EssentialityType::Essential)
        }
    );

    nutrient.borrow_mut().set_nutrient_type(NutrientType {
        chemical_type: ChemicalType::Other,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: None,
    });

    assert_eq!(
        nutrient.borrow().get_nutrient_type(),
        NutrientType {
            chemical_type: ChemicalType::Other,
            quantity_type: QuantityType::Micronutrient,
            essentiality_type: None
        }
    );
}

#[test]
fn test_accepted_units() {
    let id = None;
    let name = String::from("Potassium");
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let milligram_unit = NutrientUnit::Mass(MassUnit::Milligram);
    let mass_accepted_units: BTreeSet<NutrientUnit> = MassUnit::get_enumerations()
        .iter()
        .map(|mass| NutrientUnit::Mass(*mass))
        .collect();

    let main_unit = milligram_unit;
    let nutrient = Nutrient::new_rc_refcell(id, name.clone(), nutrient_type, main_unit);

    assert_eq!(nutrient.borrow().get_accepted_units(), mass_accepted_units);

    let milliliter_unit = NutrientUnit::Volume(VolumeUnit::Milliliter);
    let volume_accepted_units: BTreeSet<NutrientUnit> = VolumeUnit::get_enumerations()
        .iter()
        .map(|volume| NutrientUnit::Volume(*volume))
        .collect();
    let mut mass_and_volume_accepted_units = mass_accepted_units.clone();
    mass_and_volume_accepted_units.extend(volume_accepted_units.iter());
    assert_ne!(
        nutrient.borrow().get_accepted_units(),
        volume_accepted_units
    );

    // 1ml = 5mg
    let from_unit = milliliter_unit;
    let factor = 5f64;
    let _ = nutrient
        .borrow_mut()
        .insert_unit_conversion(from_unit, factor);
    assert_eq!(
        nutrient.borrow().get_accepted_units(),
        mass_and_volume_accepted_units
    );
}

#[test]
fn test_convert() {
    let id = None;
    let name = String::from("Potassium");
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let microgram_unit = NutrientUnit::Mass(MassUnit::Microgram);
    let milligram_unit = NutrientUnit::Mass(MassUnit::Milligram);
    let kilogram_unit = NutrientUnit::Mass(MassUnit::Kilogram);
    let ounce_unit = NutrientUnit::Mass(MassUnit::Ounce);
    let liter_unit = NutrientUnit::Volume(VolumeUnit::Liter);
    let milliliter_unit = NutrientUnit::Volume(VolumeUnit::Milliliter);

    // First set of tests
    let main_unit = milligram_unit;
    let nutrient = Nutrient::new_rc_refcell(id, name.clone(), nutrient_type, main_unit);
    assert_eq!(nutrient.borrow().get_main_unit(), Some(milligram_unit));

    let mut res = nutrient
        .borrow()
        .get_conversion_factor(kilogram_unit, ounce_unit);
    assert_eq!(
        res,
        Ok(MassUnit::Kilogram.si_factor() / MassUnit::Ounce.si_factor())
    );

    let expected = MassUnit::Kilogram.si_factor() / MassUnit::Milligram.si_factor();
    let actual = nutrient
        .borrow()
        .get_conversion_factor(kilogram_unit, milligram_unit)
        .unwrap();
    let epsilon = 1e-10;
    assert!((actual - expected).abs() < epsilon);

    let expected = MassUnit::Milligram.si_factor() / MassUnit::Microgram.si_factor();
    let actual = nutrient
        .borrow()
        .get_conversion_factor(milligram_unit, microgram_unit)
        .unwrap();
    let epsilon = 1e-10;
    assert!((actual - expected).abs() < epsilon);

    res = nutrient
        .borrow()
        .get_conversion_factor(milligram_unit, liter_unit);
    assert!(res.is_err());

    // Change the main unit and repeat conversions
    let _ = nutrient.borrow_mut().set_main_unit(kilogram_unit);
    assert_eq!(nutrient.borrow().get_main_unit(), Some(kilogram_unit));

    let mut res = nutrient
        .borrow()
        .get_conversion_factor(kilogram_unit, ounce_unit);
    assert_eq!(
        res,
        Ok(MassUnit::Kilogram.si_factor() / MassUnit::Ounce.si_factor())
    );

    res = nutrient
        .borrow()
        .get_conversion_factor(kilogram_unit, milligram_unit);
    assert_eq!(
        res,
        Ok(MassUnit::Kilogram.si_factor() / MassUnit::Milligram.si_factor())
    );

    res = nutrient
        .borrow()
        .get_conversion_factor(milligram_unit, microgram_unit);
    assert_eq!(
        res,
        Ok(MassUnit::Milligram.si_factor() / MassUnit::Microgram.si_factor())
    );

    res = nutrient
        .borrow()
        .get_conversion_factor(milligram_unit, liter_unit);
    assert!(res.is_err());

    // Add a different conversion, change to it and test conversions
    // 1kg = 5ml in this example
    let new_unit = milliliter_unit;
    let factor = 5f64;
    nutrient
        .borrow_mut()
        .insert_unit_conversion(new_unit, factor);

    let mut res = nutrient
        .borrow()
        .get_conversion_factor(kilogram_unit, ounce_unit);
    assert_eq!(
        res,
        Ok(MassUnit::Kilogram.si_factor() / MassUnit::Ounce.si_factor())
    );

    res = nutrient
        .borrow()
        .get_conversion_factor(kilogram_unit, milligram_unit);
    assert_eq!(
        res,
        Ok(MassUnit::Kilogram.si_factor() / MassUnit::Milligram.si_factor())
    );

    res = nutrient
        .borrow()
        .get_conversion_factor(milligram_unit, microgram_unit);
    assert_eq!(
        res,
        Ok(MassUnit::Milligram.si_factor() / MassUnit::Microgram.si_factor())
    );

    res = nutrient
        .borrow()
        .get_conversion_factor(milligram_unit, liter_unit);
    assert!(res.is_ok());

    println!("current {:#?}", nutrient.borrow().get_main_unit());
    res = nutrient
        .borrow()
        .get_conversion_factor(kilogram_unit, milliliter_unit);
    assert_eq!(
        res,
        Ok(MassUnit::Kilogram.si_factor() / (factor * MassUnit::Kilogram.si_factor()))
    );

    res = nutrient
        .borrow()
        .get_conversion_factor(milliliter_unit, liter_unit);
    assert_eq!(
        res,
        Ok(VolumeUnit::Milliliter.si_factor() / VolumeUnit::Liter.si_factor())
    );
}

#[test]
fn test_nutrient_unit_main_unit() {
    let id = None;
    let name = String::from("Potassium");
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let milligram_unit = NutrientUnit::Mass(MassUnit::Milligram);
    let microgram_unit = NutrientUnit::Mass(MassUnit::Microgram);
    let milliliter_unit = NutrientUnit::Volume(VolumeUnit::Milliliter);

    let nutrient = Nutrient::new_rc_refcell(id, name.clone(), nutrient_type, milligram_unit);
    assert_eq!(nutrient.borrow().get_main_unit(), Some(milligram_unit));

    let mut from_unit = milligram_unit;
    let mut to_unit = microgram_unit;
    assert_eq!(
        nutrient.borrow().get_conversion_factor(from_unit, to_unit),
        Ok(1_000f64)
    );

    // Test setting a value that is in the conversions
    let mut res = nutrient.borrow_mut().set_main_unit(microgram_unit);
    assert!(res.is_ok());
    assert_eq!(nutrient.borrow().get_main_unit(), Some(microgram_unit));
    assert!(nutrient
        .borrow()
        .get_accepted_units()
        .contains(&milligram_unit));
    assert!(nutrient
        .borrow()
        .get_accepted_units()
        .contains(&microgram_unit));
    assert!(!nutrient
        .borrow()
        .get_accepted_units()
        .contains(&milliliter_unit));

    assert_eq!(
        nutrient.borrow().get_conversion_factor(from_unit, to_unit),
        Ok(1_000f64)
    );

    // Test setting a value that is not in the conversions
    res = nutrient.borrow_mut().set_main_unit(milliliter_unit);
    assert!(res.is_err());

    // Add to conversions
    // 1mg = 5ml in this example
    let _ = nutrient.borrow_mut().set_main_unit(milligram_unit);
    from_unit = milligram_unit;
    to_unit = milliliter_unit;
    let factor = 5f64;
    nutrient
        .borrow_mut()
        .insert_unit_conversion(to_unit, factor);

    // let _ = nutrient.borrow_mut().set_main_unit(milliliter_unit);
    println!("{:#?}", nutrient.borrow().get_main_unit());
    println!("{:#?}", nutrient.borrow().get_unit_conversions());
    assert_eq!(
        nutrient
            .borrow()
            .get_conversion_factor(from_unit, to_unit)
            .unwrap(),
        1f64 / factor
    );
}

#[test]
fn test_link() {
    let id = None;
    let name = String::from("Iron");
    let name_2 = String::from("Heme Iron");
    let name_3 = String::from("Non-heme Iron");
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);
    let iron = Nutrient::new_rc_refcell(id, name, nutrient_type.clone(), main_unit);
    let heme_iron = Nutrient::new_rc_refcell(id, name_2, nutrient_type.clone(), main_unit);
    let non_heme_iron = Nutrient::new_rc_refcell(id, name_3, nutrient_type.clone(), main_unit);

    let res = link_parent_child(&iron, &heme_iron);
    assert!(res.is_ok());

    assert_eq!(iron.borrow().get_children(), Vec::from([heme_iron.clone()]));
    assert_eq!(
        heme_iron
            .borrow()
            .get_parents()
            .iter()
            .filter_map(|p| p.upgrade())
            .collect::<Vec<Rc<RefCell<Nutrient>>>>(),
        Vec::from([iron.clone()])
    );

    let _ = link_parent_child(&iron.clone(), &non_heme_iron);
    assert_eq!(
        non_heme_iron
            .borrow()
            .get_parents()
            .iter()
            .filter_map(|p| p.upgrade())
            .collect::<Vec<Rc<RefCell<Nutrient>>>>(),
        Vec::from([iron.clone()])
    );

    assert_eq!(
        iron.borrow().get_children(),
        Vec::from([heme_iron.clone(), non_heme_iron.clone()])
    );
    assert_eq!(
        heme_iron
            .borrow()
            .get_parents()
            .iter()
            .filter_map(|p| p.upgrade())
            .collect::<Vec<Rc<RefCell<Nutrient>>>>(),
        Vec::from([iron.clone()])
    );
}

#[test]
fn test_unlink() {
    let id = None;
    let name = String::from("Iron");
    let name_2 = String::from("Heme Iron");
    let name_3 = String::from("Non-heme Iron");
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);
    let iron = Nutrient::new_rc_refcell(id, name, nutrient_type.clone(), main_unit);
    let heme_iron = Nutrient::new_rc_refcell(id, name_2, nutrient_type.clone(), main_unit);
    let non_heme_iron = Nutrient::new_rc_refcell(id, name_3, nutrient_type.clone(), main_unit);

    let _ = link_parent_child(&iron, &heme_iron);
    let _ = link_parent_child(&iron, &non_heme_iron);
    assert_eq!(
        iron.borrow().get_children(),
        Vec::from([heme_iron.clone(), non_heme_iron.clone()])
    );
    assert_eq!(
        heme_iron
            .borrow()
            .get_parents()
            .iter()
            .filter_map(|p| p.upgrade())
            .collect::<Vec<Rc<RefCell<Nutrient>>>>(),
        Vec::from([iron.clone()])
    );
    assert_eq!(
        non_heme_iron
            .borrow()
            .get_parents()
            .iter()
            .filter_map(|p| p.upgrade())
            .collect::<Vec<Rc<RefCell<Nutrient>>>>(),
        Vec::from([iron.clone()])
    );

    unlink_parent_child(&iron, &heme_iron);
    assert_eq!(
        iron.borrow().get_children(),
        Vec::from([non_heme_iron.clone()])
    );
    assert_eq!(
        non_heme_iron
            .borrow()
            .get_parents()
            .iter()
            .filter_map(|p| p.upgrade())
            .collect::<Vec<Rc<RefCell<Nutrient>>>>(),
        Vec::from([iron.clone()])
    );
    assert_eq!(
        heme_iron
            .borrow()
            .get_parents()
            .iter()
            .filter_map(|p| p.upgrade())
            .collect::<Vec<Rc<RefCell<Nutrient>>>>(),
        Vec::new()
    );
}
