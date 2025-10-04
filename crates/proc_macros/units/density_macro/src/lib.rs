extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{
    collections::HashMap,
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

enum MeasurementContainer {
    DensityJson(HashMap<String, DensityJson>),
    MassJson(HashMap<String, MassJson>),
    VolumeJson(HashMap<String, VolumeJson>),
}

#[derive(Debug, Deserialize)]
struct Density {
    identifier: String,
    mass_unit_variant: String,
    volume_unit_variant: String,
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
    measurement_system: DensityMeasurementSystem,
    si_factor: f64,
}

#[derive(Clone, Debug, Deserialize)]
struct DensityMeasurementSystem {
    mass_measurement_system: String,
    volume_measurement_system: String,
}

#[derive(Debug, Deserialize)]
struct DensityJson {
    // mass_unit_varient: String,
    // volume_unit_varient: String,
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

            let file_content = fs::read_to_string(&full_path)
                .unwrap_or_else(|_| panic!("Unable to read file: {}", full_path.display()));

            let measurement_container: MeasurementContainer = match enum_name.to_string().as_str() {
                "MassUnit" => {
                    let serde_res =
                        serde_json::from_str::<HashMap<String, MassJson>>(&file_content)
                            .expect("Invalid JSON format");
                    MeasurementContainer::MassJson(serde_res)
                }
                "VolumeUnit" => {
                    let serde_res =
                        serde_json::from_str::<HashMap<String, VolumeJson>>(&file_content)
                            .expect("Invalid JSON format");
                    MeasurementContainer::VolumeJson(serde_res)
                }
                "DensityUnit" => {
                    let serde_res =
                        serde_json::from_str::<HashMap<String, DensityJson>>(&file_content)
                            .expect("Invalid JSON format");
                    MeasurementContainer::DensityJson(serde_res)
                }
                _ => panic!("Incorrect unit"),
            };

            match measurement_container {
                MeasurementContainer::MassJson(json_results) => {
                    for (key, data) in json_results {
                        mass_data.insert(key, data);
                    }
                }
                MeasurementContainer::VolumeJson(json_results) => {
                    for (key, data) in json_results {
                        volume_data.insert(key, data);
                    }
                }
                MeasurementContainer::DensityJson(json_results) => {
                    for (key, data) in json_results {
                        density_data.insert(key, data);
                    }
                }
            }

        }
    }

    for (mass_key, mass_value) in &mass_data {
        for (volume_key, volume_value) in &volume_data {
            let density_variant = format!("{}Per{}", mass_key, volume_key);
            let density_identifier =
            format!("{}_per_{}", mass_value.identifier, volume_value.identifier);
            let density_symbol = format!("{}/{}", mass_value.symbol, volume_value.symbol);
            let density_unit_type =
            format!("{} per {}", mass_value.unit_type, volume_value.unit_type);
            let density_unit_type_plural = format!(
                "{} per {}",
                mass_value.unit_type_plural, volume_value.unit_type
            );
            let density_measurement_system = DensityMeasurementSystem {
                mass_measurement_system: mass_value.measurement_system.clone(),
                volume_measurement_system: volume_value.measurement_system.clone(),
            };
            let density_si_factor = mass_value.si_factor / volume_value.si_factor;

            density_all.insert(
                density_variant.clone(),
                Density {
                    identifier: density_identifier.clone(),
                    mass_unit_variant: mass_key.clone(),
                    volume_unit_variant: volume_key.clone(),
                    symbol: density_symbol.clone(),
                    unit_type: density_unit_type.clone(),
                    unit_type_plural: density_unit_type_plural.clone(),
                    measurement_system: density_measurement_system.clone(),
                    si_factor: density_si_factor.clone(),
                },
            );

            if density_data.contains_key(&density_variant) {
                density.insert(
                    density_variant,
                    Density {
                        identifier: density_identifier,
                        mass_unit_variant: mass_key.clone(),
                        volume_unit_variant: volume_key.clone(),
                        symbol: density_symbol,
                        unit_type: density_unit_type,
                        unit_type_plural: density_unit_type_plural,
                        measurement_system: density_measurement_system,
                        si_factor: density_si_factor,
                    },
                );
            }
        }
    }

    let variants: Vec<_> = density
        .iter()
        .map(|(key, data)| {
            let variant = format_ident!("{}", key);
            let from_fn_name = format_ident!("from_{}", data.identifier);
            let as_fn_name = format_ident!("as_{}", data.identifier);
            let to_fn_name = format_ident!("to_{}", data.identifier);
            let mass_unit_varient = &data.mass_unit_variant;
            let volume_unit_varient = &data.volume_unit_variant;
            let mass_measurement_system = &data.measurement_system.mass_measurement_system;
            let volume_measurement_system = &data.measurement_system.volume_measurement_system;
            let symbol = &data.symbol;
            let symbol_lc = &data.symbol.to_lowercase();
            let unit_type = &data.unit_type;
            let unit_type_lc = &data.unit_type.to_lowercase();
            let unit_type_plural = &data.unit_type_plural;
            let unit_type_plural_lc = &data.unit_type_plural.to_lowercase();
            let identifier_lc = &data.identifier.to_lowercase();
            let si_factor = &data.si_factor;

            quote! {
                #variant => {
                    from_fn_name: #from_fn_name,
                    as_fn_name: #as_fn_name,
                    to_fn_name: #to_fn_name,
                    mass_unit_varient: #mass_unit_varient,
                    volume_unit_varient: #volume_unit_varient,
                    measurement_system: DensityMeasurementSystem {
                        mass_measurement_system: #mass_measurement_system.to_string(),
                        volume_measurement_system: #volume_measurement_system.to_string(),
                    },
                    symbol: #symbol,
                    symbol_lc: #symbol_lc,
                    unit_type: #unit_type,
                    unit_type_lc: #unit_type_lc,
                    unit_type_plural: #unit_type_plural,
                    unit_type_plural_lc: #unit_type_plural_lc,
                    identifier_lc: #identifier_lc,
                    si_factor: #si_factor
                }
            }
        })
        .collect();

    let variants_all: Vec<_> = density_all
        .iter()
        .map(|(key, data)| {
            let variant = format_ident!("{}", key);
            let from_fn_name = format_ident!("from_{}", data.identifier);
            let as_fn_name = format_ident!("as_{}", data.identifier);
            let to_fn_name = format_ident!("to_{}", data.identifier);
            let mass_unit_variant = &data.mass_unit_variant;
            let volume_unit_variant = &data.volume_unit_variant;
            let mass_measurement_system = &data.measurement_system.mass_measurement_system;
            let volume_measurement_system = &data.measurement_system.volume_measurement_system;
            let symbol = &data.symbol;
            let symbol_lc = &data.symbol.to_lowercase();
            let unit_type = &data.unit_type;
            let unit_type_lc = &data.unit_type.to_lowercase();
            let unit_type_plural = &data.unit_type_plural;
            let unit_type_plural_lc = &data.unit_type_plural.to_lowercase();
            let identifier_lc = &data.identifier.to_lowercase();
            let si_factor = &data.si_factor;

            quote! {
                #variant => {
                    from_fn_name: #from_fn_name,
                    as_fn_name: #as_fn_name,
                    to_fn_name: #to_fn_name,
                    mass_unit_varient: #mass_unit_variant,
                    volume_unit_varient: #volume_unit_variant,
                    measurement_system: DensityMeasurementSystem {
                        mass_measurement_system: #mass_measurement_system.to_string(),
                        volume_measurement_system: #volume_measurement_system.to_string(),
                    },
                    symbol: #symbol,
                    symbol_lc: #symbol_lc,
                    unit_type: #unit_type,
                    unit_type_lc: #unit_type_lc,
                    unit_type_plural: #unit_type_plural,
                    unit_type_plural_lc: #unit_type_plural_lc,
                    identifier_lc: #identifier_lc,
                    si_factor: #si_factor
                }
            }
        })
        .collect();

    let expanded = quote! {
        define_densities! {
            all: { #(#variants_all),* },
            json: { #(#variants),* },
        }
    };

    expanded.into()
}
