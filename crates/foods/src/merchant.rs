use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Merchant {
    id: Uuid,
    name: String,
    description: String,
    website: Option<String>,
}

impl Merchant {
    pub fn new(name: String, id: Option<Uuid>) -> Self {
        let id: Uuid = match id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        Merchant {
            id,
            name,
            description: String::new(),
            website: None,
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

    pub fn get_website(&self) -> Option<String> {
        self.website.clone()
    }

    pub fn set_website(&mut self, website: Option<String>) {
        // add validation?
        self.website = website;
    }
}
