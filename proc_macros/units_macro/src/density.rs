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

pub struct EnumSourceGroup {
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

#[derive(Clone, Debug, Deserialize)]
struct Density {
    identifier: String,
    mass_unit: String,
    volume_unit: String,
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

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct DensityJson {
    mass_unit: String,
    volume_unit: String,
}

#[derive(Debug, Deserialize)]
pub struct MassJson {
    identifier: String,
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
    measurement_system: String,
    si_factor: f64,
}

#[derive(Debug, Deserialize)]
pub struct VolumeJson {
    identifier: String,
    symbol: String,
    unit_type: String,
    measurement_system: String,
    si_factor: f64,
}

pub struct JsonHashes {
    density_data: HashSet<DensityJson>,
    mass_data: HashMap<String, MassJson>,
    volume_data: HashMap<String, VolumeJson>,
}

pub struct DensityHashMaps {
    density: HashMap<String, Density>,
    density_all: HashMap<String, Density>,
}

pub fn populate_densities_masses_and_volumes(parsed_input: EnumSourceGroup) -> JsonHashes {
    let mut density_data: HashSet<DensityJson> = HashSet::new();
    let mut mass_data: HashMap<String, MassJson> = HashMap::new();
    let mut volume_data: HashMap<String, VolumeJson> = HashMap::new();

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
                        let serde_res: HashMap<String, MassJson> =
                            serde_json::from_str(&file_content).expect("Invalid JSON format");
                        for (key, data) in serde_res {
                            mass_data.insert(key, data);
                        }
                    }
                    "VolumeUnit" => {
                        let serde_res =
                            serde_json::from_str::<HashMap<String, VolumeJson>>(&file_content)
                                .expect("Invalid JSON format");
                        for (key, data) in serde_res {
                            volume_data.insert(key, data);
                        }
                    }
                    "DensityUnit" => {
                        let serde_res: Vec<DensityJson> =
                            serde_json::from_str(&file_content).expect("Invalid JSON format");
                        density_data.extend(serde_res);
                    }
                    _ => panic!("Incorrect unit"),
                };
            }
        }
    }

    return JsonHashes {
        density_data,
        mass_data,
        volume_data,
    };
}

pub fn fill_density_hashmaps(
    mass_data: HashMap<String, MassJson>,
    volume_data: HashMap<String, VolumeJson>,
    density_data: HashSet<DensityJson>,
) -> DensityHashMaps {
    let mut density: HashMap<String, Density> = HashMap::new();
    let mut density_all: HashMap<String, Density> = HashMap::new();

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

            let density_object = Density {
                identifier: density_identifier.clone(),
                mass_unit: mass_key.clone(),
                volume_unit: volume_key.clone(),
                symbol: density_symbol.clone(),
                unit_type: density_unit_type.clone(),
                unit_type_plural: density_unit_type_plural.clone(),
                measurement_system: density_measurement_system.clone(),
                si_factor: density_si_factor,
            };

            density_all.insert(density_variant.clone(), density_object.clone());

            if density_data.contains(&DensityJson {
                mass_unit: mass_key.clone(),
                volume_unit: volume_key.clone(),
            }) {
                density.insert(density_variant, density_object);
            }
        }
    }

    return DensityHashMaps {
        density,
        density_all,
    };
}

pub fn generate(input: TokenStream) -> TokenStream {
    let parsed_input = syn::parse_macro_input!(input as EnumSourceGroup);

    let json_data = populate_densities_masses_and_volumes(parsed_input);
    let density_data: HashSet<DensityJson> = json_data.density_data;
    let mass_data: HashMap<String, MassJson> = json_data.mass_data;
    let volume_data: HashMap<String, VolumeJson> = json_data.volume_data;

    let density_hashmaps = fill_density_hashmaps(mass_data, volume_data, density_data);
    let density: HashMap<String, Density> = density_hashmaps.density;
    let density_all: HashMap<String, Density> = density_hashmaps.density_all;

    let variants: Vec<_> = density
        .iter()
        .map(|(key, data)| {
            let json_variant = format_ident!("{}", key);
            let from_fn_name = format_ident!("from_{}", data.identifier);
            let as_fn_name = format_ident!("as_{}", data.identifier);
            let to_fn_name = format_ident!("to_{}", data.identifier);
            let si_factor = &data.si_factor;

            quote! {
                #json_variant => {
                    from_fn_name: #from_fn_name,
                    as_fn_name: #as_fn_name,
                    to_fn_name: #to_fn_name,
                    si_factor: #si_factor
                }
            }
        })
        .collect();

    let variants_all: Vec<_> = density_all
        .iter()
        .map(|(key, data)| {
            let all_variant = format_ident!("{}", key);
            let from_fn_name = format_ident!("from_{}", data.identifier);
            let as_fn_name = format_ident!("as_{}", data.identifier);
            let to_fn_name = format_ident!("to_{}", data.identifier);
            let si_factor = &data.si_factor;

            quote! {
                #all_variant => {
                    from_fn_name: #from_fn_name,
                    as_fn_name: #as_fn_name,
                    to_fn_name: #to_fn_name,
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

pub fn generate_units(input: TokenStream) -> TokenStream {
    let parsed_input = syn::parse_macro_input!(input as EnumSourceGroup);

    let json_data = populate_densities_masses_and_volumes(parsed_input);
    let density_data: HashSet<DensityJson> = json_data.density_data;
    let mass_data: HashMap<String, MassJson> = json_data.mass_data;
    let volume_data: HashMap<String, VolumeJson> = json_data.volume_data;

    let density_hashmaps = fill_density_hashmaps(mass_data, volume_data, density_data);
    let density: HashMap<String, Density> = density_hashmaps.density;
    let density_all: HashMap<String, Density> = density_hashmaps.density_all;

    let variants: Vec<_> = density
        .iter()
        .map(|(key, data)| {
            let json_variant = format_ident!("{}", key);
            let mass_unit_variant = format_ident!("{}", &data.mass_unit);
            let volume_unit_variant = format_ident!("{}", &data.volume_unit);
            let mass_measurement_system =
                format_ident!("{}", &data.measurement_system.mass_measurement_system);
            let volume_measurement_system =
                format_ident!("{}", &data.measurement_system.volume_measurement_system);
            let symbol = &data.symbol;
            let symbol_lc = &data.symbol.to_lowercase();
            let unit_type = &data.unit_type;
            let unit_type_lc = &data.unit_type.to_lowercase();
            let unit_type_plural = &data.unit_type_plural;
            let unit_type_plural_lc = &data.unit_type_plural.to_lowercase();
            let identifier_lc = &data.identifier.to_lowercase();
            let si_factor = &data.si_factor;

            quote! {
                #json_variant => {
                    mass_unit_variant: #mass_unit_variant,
                    volume_unit_variant: #volume_unit_variant,
                    mass_measurement_system: #mass_measurement_system,
                    volume_measurement_system: #volume_measurement_system,
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
            let all_variant = format_ident!("{}", key);
            let mass_unit_variant = format_ident!("{}", &data.mass_unit);
            let volume_unit_variant = format_ident!("{}", &data.volume_unit);
            let mass_measurement_system =
                format_ident!("{}", &data.measurement_system.mass_measurement_system);
            let volume_measurement_system =
                format_ident!("{}", &data.measurement_system.volume_measurement_system);
            let symbol = &data.symbol;
            let symbol_lc = data.symbol.to_lowercase();
            let unit_type = &data.unit_type;
            let unit_type_lc = data.unit_type.to_lowercase();
            let unit_type_plural = &data.unit_type_plural;
            let unit_type_plural_lc = data.unit_type_plural.to_lowercase();
            let identifier_lc = data.identifier.to_lowercase();
            let si_factor = &data.si_factor;

            quote! {
                #all_variant => {
                    mass_unit_variant: #mass_unit_variant,
                    volume_unit_variant: #volume_unit_variant,
                    mass_measurement_system: #mass_measurement_system,
                    volume_measurement_system: #volume_measurement_system,
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
        define_density_units! {
            all: { #(#variants_all),* },
            json: { #(#variants),* },
        }
    };

    expanded.into()
}
