use chrono::NaiveDate;
use units::{
    distance::{Distance, DistanceUnit},
    mass::{Mass, MassUnit},
    volume::VolumeUnit,
};
use utils::base_types::Percentage;
use uuid::Uuid;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Name {
    first_name: String,
    last_name: String,
}

#[derive(Clone)]
pub enum Language {
    English,
    French,
    Spanish,
}

#[derive(Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone)]
pub enum Ethnicity {
    White,
    Black,
    EastAsian,
    WestAsian,
}

#[derive(Clone)]
pub struct PreferredUnits {
    mass: MassUnit,
    volume: VolumeUnit,
    height: DistanceUnit,
    distance: DistanceUnit,
}
