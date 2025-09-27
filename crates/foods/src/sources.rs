use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataSource {
    id: Uuid,
    name: String,
    description: String,
}
