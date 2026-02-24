use uuid::Uuid;

enum FoodCategoryChild {
    FoodCategory(Rc<RefCell<FoodCategory>>),
    FoodTaxonomy(Rc<RefCell<FoodTaxonomy>>),
}

struct FoodCategory {
    id: Uuid,
    name: String,
    description: String,
    parent: Weak<RefCell<FoodCategory>>,
    children: Vec<Rc<RefCell<FoodCategoryChild>>>,
}

impl FoodCategory {
    pub fn new(food_category: Weak<RefCell<FoodCategory>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            description: String::new(),
            parent: food_category,
            children: Vec::new(),
        }
    }

    pub fn get_id(&self) -> {
        Self.id.clone()
    }

    pub fn set_id(&mut self, id: Uuid) {
        Self.id = id;
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

    pub fn get_parent(&self) -> {
        self.parent.clone()
    }

    pub fn set_parent(&mut self, food_category: Weak<RefCell<FoodCategory>>) {
        self.parent = food_category;
    }

    pub fn get_children(&mut self) -> Vec<FoodCategoryChild> {
        self.children.clone()
    }

    pub fn set_children(&mut self, children: Vec<FoodCategoryChild>) {
        self.children = children;
    }

    pub fn push_child(&mut self, child: FoodCategoryChild) {
        self.children.push(child)
    }

    pub fn remove_child(&mut self, child: FoodCategoryChild) {
        self.children.iter().retain(|c| c != child)
    }
}
