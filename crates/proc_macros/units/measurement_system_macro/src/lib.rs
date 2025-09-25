extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{collections::{HashMap, HashSet}, env, fs, path::Path};
use syn::{Ident, LitStr, Token};
use syn::parse::{Parse, ParseStream, Result as ParseResult};

struct EnumSourceGroup {
    items: Vec<(Ident, Vec<LitStr>)>,
}

impl Parse for EnumSourceGroup {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut items = Vec::new();

        while !input.is_empty() {
            let enum_ident: Ident = input.parse()?;
            input.parse::<Token![=>]>()?;

            let mut paths = Vec::new();

            while input.peek(LitStr) {
                paths.push(input.parse()?);

                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }

            items.push((enum_ident, paths));
        }

        Ok(EnumSourceGroup { items })
    }
}

enum MeasurementContainer {
    MassJson(HashMap<String, MassJson>),
    VolumeJson(HashMap<String, VolumeJson>),
    EnergyJson(HashMap<String, EnergyJson>),
    DistanceJson(HashMap<String, DistanceJson>),
}

#[derive(Debug, Deserialize)]
struct MassJson {
    measurement_system: String,
}

#[derive(Debug, Deserialize)]
struct VolumeJson {
    measurement_system: String,
}

#[derive(Debug, Deserialize)]
struct EnergyJson {
    measurement_system: String,
}

#[derive(Debug, Deserialize)]
struct DistanceJson {
    measurement_system: String,
}

#[proc_macro]
pub fn include_measurement_systems_from_json(input: TokenStream) -> TokenStream {
    let mut measurement_systems = HashSet::new(); 

    let parsed_input = syn::parse_macro_input!(input as EnumSourceGroup);

    for (enum_name, paths) in parsed_input.items {
        for file_path_lit in paths {

            let rel_path = file_path_lit.value();
            let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
            let full_path = Path::new(&manifest_dir).join(rel_path);

            let file_content =
            fs::read_to_string(&full_path).unwrap_or_else(|_| panic!("Unable to read file: {}", full_path.display()));

            let measurement_container: MeasurementContainer = match enum_name.to_string().as_str() {
                "MassUnit" => {
                    let serde_res = serde_json::from_str::<HashMap<String, MassJson>>(&file_content).expect("Invalid JSON format");
                    MeasurementContainer::MassJson(serde_res)
                },
                "VolumeUnit" => {
                    let serde_res = serde_json::from_str(&file_content).expect("Invalid JSON format");
                    MeasurementContainer::VolumeJson(serde_res)
                },
                "EnergyUnit" => {
                    let serde_res = serde_json::from_str(&file_content).expect("Invalid JSON format");
                    MeasurementContainer::EnergyJson(serde_res)
                },
                "DistanceUnit" => {
                    let serde_res = serde_json::from_str(&file_content).expect("Invalid JSON format");
                    MeasurementContainer::DistanceJson(serde_res)
                },
                _ => panic!("Incorrect unit")
            };

            match measurement_container {
                MeasurementContainer::MassJson(json_results) => {
                    for data in json_results.values() {
                        measurement_systems.insert(data.measurement_system.clone());
                    }
                },
                MeasurementContainer::VolumeJson(json_results) => {
                    for data in json_results.values() {
                        measurement_systems.insert(data.measurement_system.clone());
                    }
                },
                MeasurementContainer::EnergyJson(json_results) => {
                    for data in json_results.values() {
                        measurement_systems.insert(data.measurement_system.clone());
                    }
                },
                MeasurementContainer::DistanceJson(json_results) => {
                    for data in json_results.values() {
                        measurement_systems.insert(data.measurement_system.clone());
                    }
                },
            }
        }
    }

    let variants: Vec<_> = measurement_systems.iter().map(|measurement_system| {
        let variant = format_ident!("{}", measurement_system);

        quote! {
            #variant
        }
    }).collect();

    let expanded = quote! {
        define_measurement_systems! {
            #(#variants),*
        }
    };

    expanded.into()
}

