extern crate proc_macro;

use glob::glob;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::Path,
};
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Ident, LitStr, Token};

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

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
struct MassJson {
    measurement_system: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
struct VolumeJson {
    measurement_system: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
struct EnergyJson {
    measurement_system: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
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
            let manifest_dir = env::var("WORKSPACE_ROOT").expect("WORKSPACE_ROOT not set");
            let full_path = Path::new(&manifest_dir).join(rel_path);

            let pattern = if full_path.is_dir() {
                format!("{}/**/*.json", full_path.display())
            } else {
                full_path.display().to_string()
            };

            for entry in glob(&pattern).expect("Error reading glob pattern") {
                let full_path = entry.expect("Invalid path");

                let file_content = fs::read_to_string(&full_path)
                    .unwrap_or_else(|_| panic!("Unable to read file: {}", full_path.display()));

                match enum_name.to_string().as_str() {
                    "MassUnit" => {
                        serde_json::from_str::<HashMap<String, MassJson>>(&file_content)
                            .expect("Invalid JSON format")
                            .values().into_iter().for_each(|mass| { measurement_systems.insert(mass.measurement_system.clone()); });
                    }
                    "VolumeUnit" => {
                        serde_json::from_str::<HashMap<String, VolumeJson>>(&file_content)
                            .expect("Invalid JSON format")
                            .values().into_iter().for_each(|volume| { measurement_systems.insert(volume.measurement_system.clone()); });
                    }
                    "EnergyUnit" => {
                        serde_json::from_str::<HashMap<String, EnergyJson>>(&file_content)
                            .expect("Invalid JSON format")
                            .values().into_iter().for_each(|energy| { measurement_systems.insert(energy.measurement_system.clone()); });
                    }
                    "DistanceUnit" => {
                        serde_json::from_str::<HashMap<String, DistanceJson>>(&file_content)
                            .expect("Invalid JSON format")
                            .values().into_iter().for_each(|distance| { measurement_systems.insert(distance.measurement_system.clone()); });
                    }
                    _ => panic!("Incorrect unit"),
                };
            }
        }
    }

    let variants: Vec<_> = measurement_systems
        .iter()
        .map(|measurement_system| {
            let variant = format_ident!("{}", measurement_system);

            quote! {
                #variant
            }
        })
        .collect();

    let expanded = quote! {
        define_measurement_systems! {
            #(#variants),*
        }
    };

    expanded.into()
}
