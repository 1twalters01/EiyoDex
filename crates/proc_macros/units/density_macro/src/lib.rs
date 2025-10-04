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
    DensityJson(HashMap<String, DensityJson>),
    MassJson(HashMap<String, MassJson>),
    VolumeJson(HashMap<String, VolumeJson>),
}

#[derive(Debug, Deserialize)]
struct Density {
    identifier: String,
    mass_unit_varient: String,
    volume_unit_varient: String,
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
    measurement_system: String,
    si_factor: f64,
}

#[derive(Debug, Deserialize)]
struct DensityJson {
    mass_unit_varient: String,
    volume_unit_varient: String,
}

#[derive(Debug, Deserialize)]
struct MassJson {
    identifier: String,
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
    measurement_system: String,
    si_factor: f64,
}

#[derive(Debug, Deserialize)]
struct VolumeJson {
    identifier: String,
    symbol: String,
    unit_type: String,
    measurement_system: String,
    si_factor: f64,
}

#[proc_macro]
pub fn include_densities_from_json(input: TokenStream) -> TokenStream {
    let mut density_all: HashMap<String, Density> = HashMap::new();
    let mut mass_data: HashMap<String, MassJson> = HashMap::new();
    let mut volume_data: HashMap<String, VolumeJson> = HashMap::new();

    let mut density: HashMap<String, Density> = HashMap::new(); 
    let mut density_data: HashMap<String, DensityJson> = HashMap::new();

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
                    let serde_res = serde_json::from_str::<HashMap<String, VolumeJson>>(&file_content).expect("Invalid JSON format");
                    MeasurementContainer::VolumeJson(serde_res)
                },
                "DensityUnit" => {
                    let serde_res = serde_json::from_str::<HashMap<String, DensityJson>>(&file_content).expect("Invalid JSON format");
                    MeasurementContainer::DensityJson(serde_res)
                },
                _ => panic!("Incorrect unit")
            };

            match measurement_container {
                MeasurementContainer::MassJson(json_results) => {
                    for (key, data) in json_results {
                        mass_data.insert(key, data);
                    }
                },
                MeasurementContainer::VolumeJson(json_results) => {
                    for (key, data) in json_results {
                        volume_data.insert(key, data);
                    }
                },
                MeasurementContainer::DensityJson(json_results) => {
                    for (key, data) in json_results {
                        density_data.insert(key, data);
                    }
                },
            }

            for (mass_key, mass_value) in mass_data {
                for (volume_key, volume_data) in volume_data {
                }
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
