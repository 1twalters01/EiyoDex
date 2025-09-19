use uuid::Uuid;
use chrono::NaiveDate;

pub struct Profile {
    id: Uuid,
    name: Name,
    date_of_birth: NaiveDate,
    language: Language,
    gender: Gender,
    ethnicity: Option<Ethnicity>,
    preferred_units: PreferredUnits,
    height: Distance,
    weight: Mass,
    target_weight: Mass,
    target_weight_deadline: Option<NaiveDate>,
    // waist
    // hip
    body_fat_percentage: Percentage,
}

pub struct Name {
    first_name: String,
    last_name: String,
}

pub enum Language {
    English,
    French,
    Spanish,
}

pub enum Gender {
    Male,
    Female,
}

pub enum Ethnicity {
    White,
    Black,
    EastAsian,
    WestAsian,
}

pub struct PreferredUnits {
    mass: UnitType,
    volume: UnitType,
    height: UnitType,
    distance: UnitType,
}
