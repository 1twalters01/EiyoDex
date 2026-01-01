use crate::{data_sources::DataSource, food_nutrition_data::FoodNutritionData};
use nutrients::{
    nutrient_amount::NutrientAmount, nutrient_list::NutrientAmountList, units::NutrientUnit,
};
use std::{
    cell::RefCell,
    collections::BTreeSet,
    rc::{Rc, Weak},
};
use units::energy::Energy;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoodCategory {
    parents: Vec<Rc<RefCell<FoodCategory>>>,
    children: Vec<Rc<RefCell<FoodCategory>>>,
    food_instance: Option<FoodInstance>,
}

impl FoodCategory {
    pub fn new() -> FoodCategory {
        FoodCategory {
            parents: Vec::new(),
            children: Vec::new(),
            food_instance: None,
        }
    }

    pub fn get_parents(&self) -> Vec<Rc<RefCell<FoodCategory>>> {
        self.parents.clone()
    }

    pub fn set_parents(&mut self, food_categories: Vec<Rc<RefCell<FoodCategory>>>) {
        self.parents = food_categories;
    }

    pub fn add_parent(&mut self, food_category: FoodCategory) {
        self.parents.push(Rc::new(RefCell::new(food_category)));
    }

    pub fn add_parent_rc_refcell(&mut self, food_category: Rc<RefCell<FoodCategory>>) {
        self.parents.push(food_category);
    }

    pub fn remove_parent(&mut self, food_category: FoodCategory) {
        if let Some(pos) = self
            .parents
            .iter()
            .position(|x| *x.borrow() == food_category)
        {
            self.parents.remove(pos);
        }
    }

    pub fn remove_parent_rc_refcell(&mut self, food_category: Rc<RefCell<FoodCategory>>) {
        if let Some(pos) = self
            .parents
            .iter()
            .position(|x| Rc::ptr_eq(x, &food_category))
        {
            self.parents.remove(pos);
        }
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<FoodCategory>>> {
        self.children.clone()
    }

    pub fn set_children(&mut self, food_categories: Vec<Rc<RefCell<FoodCategory>>>) {
        self.children = food_categories;
    }

    pub fn add_child(&mut self, food_category: FoodCategory) {
        self.children.push(Rc::new(RefCell::new(food_category)));
    }

    pub fn add_child_rc_refcell(&mut self, food_category: Rc<RefCell<FoodCategory>>) {
        self.children.push(food_category);
    }

    pub fn remove_child(&mut self, food_category: FoodCategory) {
        if let Some(pos) = self
            .children
            .iter()
            .position(|x| *x.borrow() == food_category)
        {
            self.children.remove(pos);
        }
    }

    pub fn remove_child_rc_refcell(&mut self, food_category: Rc<RefCell<FoodCategory>>) {
        if let Some(pos) = self
            .children
            .iter()
            .position(|x| Rc::ptr_eq(x, &food_category))
        {
            self.children.remove(pos);
        }
    }

    pub fn get_food_instance(&self) -> Option<FoodInstance> {
        self.food_instance.clone()
    }

    pub fn set_food_instance(&mut self, food_instance: Option<FoodInstance>) {
        self.food_instance = food_instance;
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoodInstance {
    id: Uuid,
    name: String,
    description: String,
    tags: BTreeSet<FoodTag>,
    food_data: BTreeSet<FoodNutritionData>,
}

impl FoodInstance {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        Self {
            id,
            name,
            description: String::new(),
            tags: BTreeSet::new(),
            food_data: BTreeSet::new(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_tags(&self) -> BTreeSet<FoodTag> {
        self.tags.clone()
    }

    pub fn set_tags(&mut self, tags: BTreeSet<FoodTag>) {
        self.tags = tags;
    }

    pub fn add_tag(&mut self, tag: FoodTag) {
        self.tags.insert(tag);
    }

    pub fn remove_tag(&mut self, tag: FoodTag) {
        self.tags.remove(&tag);
    }

    pub fn get_food_data(&self) -> BTreeSet<FoodNutritionData> {
        self.food_data.clone()
    }

    pub fn set_food_data(&mut self, food_data: BTreeSet<FoodNutritionData>) {
        self.food_data = food_data;
    }

    pub fn add_food_data(&mut self, food_data: FoodNutritionData) {
        self.food_data.insert(food_data);
    }

    pub fn remove_food_data(&mut self, food_data: FoodNutritionData) {
        self.food_data.remove(&food_data);
    }

    pub fn get_calories(&self, food_data_uuid: Uuid) -> Energy {
        let mut food_data: Option<FoodNutritionData> = None;
        for data in self.food_data.clone() {
            if data.get_data_source().get_id() == food_data_uuid {
                food_data = Some(data)
            }
        }

        let mut energy: Energy = Energy::new(0f64, units::energy::EnergyUnit::Kilocalorie);
        match food_data {
            Some(data) => {
                for nutrient_amount in data.get_nutrient_amount_list().get_nutrient_amounts() {
                    match nutrient_amount.get_nutrient() {
                        Some(nutrient) => {
                            if nutrient.borrow().get_name() == "Calories" {
                                let unit = nutrient.borrow().get_main_unit();
                                match unit {
                                    NutrientUnit::Energy(energy_unit) => {
                                        energy =
                                            Energy::new(nutrient_amount.get_value(), energy_unit);
                                        break;
                                    }
                                    _ => {}
                                };
                            }
                        }
                        None => {}
                    }
                }
            }
            None => energy = Energy::new(0f64, units::energy::EnergyUnit::Kilocalorie),
        };
        return energy;
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoodTag {
    id: Uuid,
    name: String,
    description: String,
    applicable_categories: Vec<FoodCategory>,
}

impl FoodTag {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        Self {
            id,
            name,
            description: String::new(),
            applicable_categories: Vec::new(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_applicable_categories(&self) -> Vec<FoodCategory> {
        self.applicable_categories.clone()
    }

    pub fn set_applicable_categories(&mut self, food_categories: Vec<FoodCategory>) {
        self.applicable_categories = food_categories;
    }

    pub fn add_applicable_category(&mut self, food_category: FoodCategory) {
        self.applicable_categories.push(food_category);
    }

    pub fn remove_applicable_category(&mut self, food_category: FoodCategory) {
        if let Some(pos) = self
            .applicable_categories
            .iter()
            .position(|x| *x == food_category)
        {
            self.applicable_categories.remove(pos);
        }
    }
}
