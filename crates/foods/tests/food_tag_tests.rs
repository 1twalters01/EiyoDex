use std::collections::BTreeSet;

use foods::food_tag::FoodTag;
use uuid::Uuid;

#[test]
pub fn test_food_tag_id() {
    let id = Uuid::from_u128(9u128);
    let name = String::from("Fermented");
    let mut food_tag = FoodTag::new(None, name);

    assert_ne!(food_tag.get_id(), id);
    food_tag.set_id(id);
    assert_eq!(food_tag.get_id(), id);
}

#[test]
pub fn test_food_tag_name() {
    let id = Uuid::from_u128(12u128);
    let name_1 = String::from("Fermented");
    let name_2 = String::from("Raw");
    let mut food_tag = FoodTag::new(Some(id), name_1.clone());

    assert_eq!(food_tag.get_name(), name_1);
    assert_ne!(food_tag.get_name(), name_2);
    food_tag.set_name(name_2.clone());
    assert_ne!(food_tag.get_name(), name_1);
    assert_eq!(food_tag.get_name(), name_2);
}

#[test]
pub fn test_food_tag_description() {
    let id = Uuid::from_u128(9u128);
    let name = String::from("Fermented");
    let description = String::from("Food that is fermented");
    let mut food_tag = FoodTag::new(Some(id), name);
    
    assert_eq!(food_tag.get_description(), String::new());
    food_tag.set_description(description.clone());
    assert_eq!(food_tag.get_description(), description);
}

#[test]
pub fn test_food_tag_applicable_categories_get_and_set() {
    let id = Uuid::from_u128(9u128);
    let name = String::from("Fermented");
    let mut food_tag = FoodTag::new(Some(id), name);

    assert_eq!(food_tag.get_applicable_categories(), BTreeSet::new());
}

#[test]
pub fn test_food_tag_applicable_categories_add_remove() {
    let id = Uuid::from_u128(9u128);
    let name = String::from("Fermented");
    let mut food_tag = FoodTag::new(Some(id), name);

    assert_eq!(food_tag.get_applicable_categories(), BTreeSet::new());
}

#[test]
pub fn test_food_tag_applicable_categories_extend() {
    let id = Uuid::from_u128(9u128);
    let name = String::from("Fermented");
    let mut food_tag = FoodTag::new(Some(id), name);

    assert_eq!(food_tag.get_applicable_categories(), BTreeSet::new());
}
