use chrono::NaiveDate;
use units::{
    distance::{Distance, DistanceUnit},
    mass::{Mass, MassUnit},
    volume::VolumeUnit,
};
use utils::base_types::Percentage;
use uuid::Uuid;

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
    waist_circumference: Distance,
    hip_circumference: Distance,
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
    mass: MassUnit,
    volume: VolumeUnit,
    height: DistanceUnit,
    distance: DistanceUnit,
}
