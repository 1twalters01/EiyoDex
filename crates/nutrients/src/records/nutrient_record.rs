use std::collections::BTreeMap;

use utils::dsa::node::GraphNode;
use uuid::Uuid;

use crate::{nutrient::Nutrient, schema::nutrient_type::NutrientType, nutrient_units::NutrientUnit};

pub struct NutrientRecord {
    id: Uuid,
    name: String,
    description: String,
    nutrient_type: NutrientType,
    unit_conversions: BTreeMap<NutrientUnit, f64>, // 1 unit = factor * main_unit
    main_unit: NutrientUnit,
}

impl NutrientRecord {
    pub fn from_nutrient(nutrient: Nutrient) -> Self {
        Self {
            id: nutrient.get_id(),
            name: nutrient.get_name(),
            description: nutrient.get_description(),
            nutrient_type: nutrient.get_nutrient_type(),
            unit_conversions: nutrient.get_unit_conversions(),
            main_unit: nutrient.get_main_unit(),
        }
    }

    pub fn from_nutrient_vec(nutrient_vec: Vec<Nutrient>) -> Vec<Self> {
        nutrient_vec
            .iter()
            .map(|nutrient| Self {
                id: nutrient.get_id(),
                name: nutrient.get_name(),
                description: nutrient.get_description(),
                nutrient_type: nutrient.get_nutrient_type(),
                unit_conversions: nutrient.get_unit_conversions(),
                main_unit: nutrient.get_main_unit(),
            })
            .collect()
    }

    // pub fn load_from_sqlite() -> Vec<Self> {}
    // pub fn save_to_sqlite() {}
}

pub struct NutrientLinkRecord {
    parent_ids: Vec<Uuid>,
    child_ids: Vec<Uuid>,
}

impl NutrientLinkRecord {
    pub fn from_nutrient(nutrient: Nutrient) -> Result<Self, &'static str> {
        let parent_ids: Vec<Uuid> = nutrient
            .get_parents()
            .iter()
            .map(|parent| {
                let parent_rc = parent
                    .upgrade()
                    .ok_or("Failed to upgrade weak reference to parent")?;
                let parent_id = parent_rc.borrow().get_id();
                Ok(parent_id)
            })
            .collect::<Result<Vec<Uuid>, &'static str>>()?;
        let child_ids = nutrient
            .get_children()
            .iter()
            .map(|child| child.borrow().get_id())
            .collect();
        Ok(Self {
            parent_ids,
            child_ids,
        })
    }

    pub fn from_nutrient_vec(nutrient_vec: Vec<Nutrient>) -> Result<Vec<Self>, &'static str> {
        nutrient_vec
            .iter()
            .map(|nutrient| {
                let parent_ids: Vec<Uuid> = nutrient
                    .get_parents()
                    .iter()
                    .map(|parent| {
                        let parent_rc = parent
                            .upgrade()
                            .ok_or("Failed to upgrade weak reference to parent")?;
                        let parent_id = parent_rc.borrow().get_id();
                        Ok(parent_id)
                    })
                    .collect::<Result<Vec<Uuid>, &'static str>>()?;
                let child_ids = nutrient
                    .get_children()
                    .iter()
                    .map(|child| child.borrow().get_id())
                    .collect();
                Ok(Self {
                    parent_ids,
                    child_ids,
                })
            })
            .collect()
    }

    // pub fn load_from_sqlite() -> Vec<Self> {}
    // pub fn save_to_sqlite() {}
}
