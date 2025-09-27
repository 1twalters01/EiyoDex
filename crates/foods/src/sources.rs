use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct DataSource {
    id: Uuid,
    name: String,
    description: String,
}
