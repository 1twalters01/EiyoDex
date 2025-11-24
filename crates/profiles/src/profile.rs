use chrono::{Datelike, Local, NaiveDate};
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

impl Profile {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_name(&self) -> Name {
        self.name
    }

    pub fn set_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn get_date_of_birth(&self) -> NaiveDate {
        self.date_of_birth
    }

    pub fn set_date_of_birth(&mut self, date_of_birth: NaiveDate) {
        self.date_of_birth = date_of_birth;
    }

    pub fn get_age(&self) -> i32 {
        let dob = self.get_date_of_birth();
        let today = Local::now().naive_local();

        if (today.month(), today.day()) < (dob.month(), dob.day()) {
            return today.year() - dob.year() - 1;
        } else {
            return today.year() - dob.year();
        }
    }

    pub fn get_language(&self) -> Language {
        self.language
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
    
    pub fn get_gender(&self) -> Gender {
        self.gender.clone()
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }

    pub fn get_ethnicity(&self) -> Option<Ethnicity> {
        self.ethnicity
    }

    pub fn set_ethnicity(&mut self, ethnicity: Option<Ethnicity> ) {
        self.ethnicity = ethnicity;
    }

    pub fn get_preferred_units(&self) -> PreferredUnits {
        self.preferred_units
    }

    pub fn set_preferred_units(&mut self, preferred_units: PreferredUnits) {
        self.preferred_units = preferred_units;
    }

    pub fn get_height(&self) -> Distance {
        self.height
    }

    pub fn set_height(&mut self, height: Distance) {
        self.height = height;
    }

    pub fn get_weight(&self) -> Mass {
        self.weight
    }

    pub fn set_weight(&mut self, weight: Mass) {
        self.weight = weight;
    }

    pub fn get_target_weight(&self) -> Mass {
        self.target_weight
    }

    pub fn set_target_weight(&self, target_weight: Mass) {
        self.target_weight = target_weight;
    }

    pub fn get_target_weight_deadline(&self) -> Option<NaiveDate> {
        self.target_weight_deadline
    }

    pub fn set_target_weight_deadline(&mut self, target_weight_deadline: Option<NaiveDate>) {
        self.target_weight_deadline = target_weight_deadline;
    }

    pub fn get_waist_circumference(&self) -> Distance {
        self.waist_circumference
    }

    pub fn set_waist_circumference(&mut self, waist_circumference: Distance) {
        self.waist_circumference = waist_circumference;
    }

    pub fn get_hip_circumference(&self) -> Distance {
        self.hip_circumference
    }

    pub fn set_hip_circumference(&mut self, hip_circumference: Distance) {
        self.hip_circumference = hip_circumference;
    }

    pub fn get_body_fat_percentage(&self) -> Percentage {
        self.body_fat_percentage
    }

    pub fn set_body_fat_percentage(&mut self, body_fat_percentage: Percentage) {
        self.body_fat_percentage = body_fat_percentage;
    }
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
    mass_unit: MassUnit,
    volume_unit: VolumeUnit,
    height_unit: DistanceUnit,
    distance_unit: DistanceUnit,
}

impl PreferredUnits {
    pub fn new(
        mass_unit: MassUnit,
        volume_unit: VolumeUnit,
        height_unit: DistanceUnit,
        distance_unit: DistanceUnit,
    ) -> Self {
        Self {
            mass_unit,
            volume_unit,
            height_unit,
            distance_unit,
        }
    }

    pub fn get_mass_unit(&self) -> MassUnit {
        self.mass_unit.clone()
    }

    pub fn set_mass(&mut self, mass_unit: MassUnit) {
        self.mass_unit = mass_unit;
    }

    pub fn get_volume(&self) -> VolumeUnit {
        self.volume_unit.clone()
    }

    pub fn set_volume(&mut self, volume_unit: VolumeUnit) {
        self.volume_unit = volume_unit;
    }

    pub fn get_height(&self) -> DistanceUnit {
        self.height_unit.clone()
    }

    pub fn set_height(&mut self, height_unit: DistanceUnit) {
        self.height_unit = height_unit;
    }

    pub fn get_distance(&self) -> DistanceUnit {
        self.distance_unit.clone()
    }

    pub fn set_distance(&mut self, distance_unit: DistanceUnit) {
        self.distance_unit = distance_unit;
    }
}
