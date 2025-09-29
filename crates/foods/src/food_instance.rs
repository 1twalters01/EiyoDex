use crate::sources::DataSource;
use nutrients::nutrient::{NutrientAmount, Unit};
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};
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
    favourite: bool,
    tags: BTreeSet<FoodTag>,
    food_data: BTreeSet<FoodData>,
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
            favourite: false,
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

    pub fn get_favourite_status(&self) -> bool {
        self.favourite
    }

    pub fn set_favourite_status(&mut self, favourite_status: bool) {
        self.favourite = favourite_status;
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

    pub fn get_food_data(&self) -> BTreeSet<FoodData> {
        self.food_data.clone()
    }

    pub fn set_food_data(&mut self, food_data: BTreeSet<FoodData>) {
        self.food_data = food_data;
    }

    pub fn add_food_data(&mut self, food_data: FoodData) {
        self.food_data.insert(food_data);
    }

    pub fn remove_food_data(&mut self, food_data: FoodData) {
        self.food_data.remove(&food_data);
    }

    pub fn get_calories(&self, food_data_uuid: Uuid) -> Energy {
        let mut food_data: Option<FoodData> = None;
        for data in self.food_data.clone() {
            if data.get_data_source().get_id() == food_data_uuid {
                food_data = Some(data)
            }
        }

        let mut energy: Energy = Energy::new(0f64, units::energy::EnergyUnit::Kilocalorie);
        match food_data {
            Some(data) => {
                for nutrient in data.get_nutrients() {
                    if nutrient.get_nutrient().borrow().get_name() == "Calories" {
                        let unit = nutrient.get_nutrient().borrow().get_main_unit();
                        match unit {
                            Unit::Energy(energy_unit) => {
                                energy = Energy::new(nutrient.get_value(), energy_unit);
                                break;
                            }
                            _ => {}
                        };
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoodData {
    data_source: DataSource,
    nutrients: BTreeSet<NutrientAmount>,
}

impl FoodData {
    pub fn new(data_source: DataSource, nutrients: BTreeSet<NutrientAmount>) -> Self {
        Self {
            data_source,
            nutrients,
        }
    }

    pub fn get_data_source(&self) -> DataSource {
        self.data_source.clone()
    }

    pub fn set_data_source(&mut self, data_source: DataSource) {
        self.data_source = data_source;
    }

    pub fn get_nutrients(&self) -> BTreeSet<NutrientAmount> {
        self.nutrients.clone()
    }

    pub fn set_nutrients(&mut self, nutrients: BTreeSet<NutrientAmount>) {
        self.nutrients = nutrients;
    }

    pub fn add_nutrient(&mut self, nutrient: NutrientAmount) -> Result<(), &'static str> {
        for self_nutrient in self.nutrients.iter() {
            if Rc::ptr_eq(&self_nutrient.get_nutrient(), &nutrient.get_nutrient()) {
                return Err("Nutrient already in nutrients");
            }

            if self_nutrient.get_nutrient().borrow().get_id()
                == nutrient.get_nutrient().borrow().get_id()
            {
                return Err("Nutrient already in nutrients");
            }
        }
        self.nutrients.insert(nutrient);

        return Ok(());
    }

    pub fn remove_nutrient(&mut self, nutrient: &NutrientAmount) {
        self.nutrients.remove(nutrient);
    }
}
