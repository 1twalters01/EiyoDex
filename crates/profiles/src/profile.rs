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
    // id
    // name
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
    // language
    pub fn get_gender(&self) -> Gender {
        self.gender.clone()
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }

    // ethnicity
    // preferred_units

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

    // target weight
    // target weight deadline

    pub fn get_waist_circumference(&self) -> Distance {
        self.waist_circumference
    }

    pub fn set_waist_circumference(&mut self, waist_circumference: Distance) {
        self.waist_circumference = waist_circumference;
    }

    // hip circumference
    // body fat percentage
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
