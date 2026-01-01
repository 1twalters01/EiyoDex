use foods::data_sources::DataSource;
use uuid::Uuid;

#[test]
pub fn test_id() {
    let data_source_id_1 = None;
    let data_source_id_2 = Uuid::from_u128(62u128);
    let data_source_name = String::from("NCCDB");
    let mut data_source = DataSource::new(data_source_id_1, data_source_name);

    assert_ne!(data_source.get_id(), data_source_id_2);

    data_source.set_id(data_source_id_2);
    assert_eq!(data_source.get_id(), data_source_id_2);
}

#[test]
pub fn test_name() {
    let data_source_id = Some(Uuid::from_u128(62u128));
    let data_source_name_1 = String::from("NCCDB");
    let data_source_name_2 = String::from("USDA");
    let mut data_source = DataSource::new(data_source_id, data_source_name_1.clone());

    assert_eq!(data_source.get_name(), data_source_name_1);
    assert_ne!(data_source.get_name(), data_source_name_2);

    data_source.set_name(data_source_name_2.clone());

    assert_ne!(data_source.get_name(), data_source_name_1);
    assert_eq!(data_source.get_name(), data_source_name_2);
}

#[test]
pub fn test_description() {
    let data_source_id = Some(Uuid::from_u128(62u128));
    let data_source_name = String::from("NCCDB");
    let data_source_description = String::from(
        "Nutrition data from the Nutrition Coordinating Center Food & Nutrient Database",
    );
    let mut data_source = DataSource::new(data_source_id, data_source_name.clone());

    assert_eq!(data_source.get_description(), String::new());

    data_source.set_description(data_source_description.clone());

    assert_eq!(data_source.get_description(), data_source_description);
}
