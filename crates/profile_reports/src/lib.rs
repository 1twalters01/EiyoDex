use chrono::NaiveDateTime;
use profiles::profile::Profile;
use nutrients::nutrient::Nutrient;

pub fn get_current_streak(profile: Profile) {}

pub fn get_longest_streak(profile: Profile) {}

pub fn get_weight_change(profile: Profile, from: NaiveDateTime, to: NaiveDateTime) {}

pub fn get_body_fat_percentage_change(profile: Profile, from: NaiveDateTime, to: NaiveDateTime) {}

pub fn get_full_nutrient_breakdown(profile: Profile, from: NaiveDateTime, to: NaiveDateTime) {}

pub fn get_nutrient_breakdown(profile: Profile, from: NaiveDateTime, to: NaiveDateTime, nutrients: Vec<Nutrient>) {}

