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

#[derive(Clone, Debug, Deserialize)]
struct Power {
    identifier: String,
    energy_unit: String,
    duration_unit: String,
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
    measurement_system: PowerMeasurementSystem,
    si_factor: f64,
}

#[derive(Clone, Debug, Deserialize)]
struct PowerMeasurementSystem {
    energy_measurement_system: String,
    duration_measurement_system: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
struct PowerJson {
    energy_unit: String,
    duration_unit: String,
}

#[derive(Debug, Deserialize)]
struct EnergyJson {
    identifier: String,
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
    measurement_system: String,
    si_factor: f64,
}

#[derive(Debug, Deserialize)]
struct DurationJson {
    unit_type: String,
    symbol: String,
    measurement_system: String,
    si_factor: f64,
}

struct JsonHashes {
    power_data: HashSet<PowerJson>,
    energy_data: HashMap<String, EnergyJson>,
    duration_data: HashMap<String, DurationJson>,
}

struct PowerHashmaps {
    power_all: HashMap<String, Power>,
    power: HashMap<String, Power>,
}


fn populate_powers_energies_durations(parsed_input: EnumSourceGroup) -> JsonHashes {
    let mut power_data: HashSet<PowerJson> = HashSet::new();
    let mut energy_data: HashMap<String, EnergyJson> = HashMap::new();
    let mut duration_data: HashMap<String, DurationJson> = HashMap::new();

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
                    "EnergyUnit" => {
                        let serde_res: HashMap<String, EnergyJson> =
                            serde_json::from_str(&file_content).expect("Invalid JSON format");
                        for (key, data) in serde_res {
                            energy_data.insert(key, data);
                        }
                    }
                    "DurationUnit" => {
                        let serde_res: HashMap<String, DurationJson> =
                            serde_json::from_str(&file_content).expect("Invalid JSON format");
                        for (key, data) in serde_res {
                            duration_data.insert(key, data);
                        }
                    }
                    "PowerUnit" => {
                        let serde_res: Vec<PowerJson> =
                            serde_json::from_str(&file_content).expect("Invalid JSON format");
                        power_data.extend(serde_res);
                    }
                    _ => panic!("Incorrect unit"),
                };
            }
        }
    }

    return JsonHashes {
        power_data,
        energy_data,
        duration_data,
    }
}

fn fill_power_hashmaps(
    power_data: HashSet<PowerJson>,
    energy_data: HashMap<String, EnergyJson>,
    duration_data: HashMap<String, DurationJson>,
) -> PowerHashmaps {
    let mut power_all: HashMap<String, Power> = HashMap::new();
    let mut power: HashMap<String, Power> = HashMap::new();

    for (energy_key, energy_value) in &energy_data {
        for (duration_key, duration_value) in &duration_data {
            let power_variant = format!("{}Per{}", energy_key, duration_key);

            let power_identifier =
                format!("{}_per_{}", energy_value.identifier, duration_value.symbol);
            let power_symbol = format!("{}/{}", energy_value.symbol, duration_value.symbol);
            let power_unit_type = format!(
                "{} per {}",
                energy_value.unit_type, duration_value.unit_type
            );
            let power_unit_type_plural = format!(
                "{} per {}",
                energy_value.unit_type_plural, duration_value.unit_type
            );
            let power_measurement_system = PowerMeasurementSystem {
                energy_measurement_system: energy_value.measurement_system.clone(),
                duration_measurement_system: duration_value.measurement_system.clone(),
            };
            let power_si_factor = energy_value.si_factor / duration_value.si_factor;

            let power_object = Power {
                identifier: power_identifier.clone(),
                energy_unit: energy_key.clone(),
                duration_unit: duration_key.clone(),
                symbol: power_symbol.clone(),
                unit_type: power_unit_type.clone(),
                unit_type_plural: power_unit_type_plural.clone(),
                measurement_system: power_measurement_system.clone(),
                si_factor: power_si_factor,
            };

            power_all.insert(power_variant.clone(), power_object.clone());

            if power_data.contains(&PowerJson {
                energy_unit: energy_key.clone(),
                duration_unit: duration_key.clone(),
            }) {
                power.insert(power_variant, power_object);
            }
        }
    }

    return PowerHashmaps {
        power_all,
        power,
    }
}

pub fn generate(input: TokenStream) -> TokenStream {
    let parsed_input = syn::parse_macro_input!(input as EnumSourceGroup);

    let json_hashes = populate_powers_energies_durations(parsed_input);
    let power_data = json_hashes.power_data;
    let energy_data = json_hashes.energy_data;
    let duration_data = json_hashes.duration_data;

    let power_hashmaps = fill_power_hashmaps(power_data, energy_data, duration_data);
    let power_all = power_hashmaps.power_all;
    let power = power_hashmaps.power;

    let variants: Vec<_> = power
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

    let variants_all: Vec<_> = power_all
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
        define_powers! {
            all: { #(#variants_all),* },
            json: { #(#variants),* },
        }
    };

    expanded.into()
}

pub fn generate_units(input: TokenStream) -> TokenStream {
    let parsed_input = syn::parse_macro_input!(input as EnumSourceGroup);

    let json_hashes = populate_powers_energies_durations(parsed_input);
    let power_data = json_hashes.power_data;
    let energy_data = json_hashes.energy_data;
    let duration_data = json_hashes.duration_data;

    let power_hashmaps = fill_power_hashmaps(power_data, energy_data, duration_data);
    let power_all = power_hashmaps.power_all;
    let power = power_hashmaps.power;

    let variants: Vec<_> = power
        .iter()
        .map(|(key, data)| {
            let json_variant = format_ident!("{}", key);
            let energy_unit_variant = format_ident!("{}", &data.energy_unit);
            let duration_unit_variant = format_ident!("{}", &data.duration_unit);
            let energy_measurement_system =
                format_ident!("{}", &data.measurement_system.energy_measurement_system);
            let duration_measurement_system =
                format_ident!("{}", &data.measurement_system.duration_measurement_system);
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
                    energy_unit_variant: #energy_unit_variant,
                    duration_unit_variant: #duration_unit_variant,
                    energy_measurement_system: #energy_measurement_system,
                    duration_measurement_system: #duration_measurement_system,
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

    let variants_all: Vec<_> = power_all
        .iter()
        .map(|(key, data)| {
            let all_variant = format_ident!("{}", key);
            let energy_unit_variant = format_ident!("{}", &data.energy_unit);
            let duration_unit_variant = format_ident!("{}", &data.duration_unit);
            let energy_measurement_system =
                format_ident!("{}", &data.measurement_system.energy_measurement_system);
            let duration_measurement_system =
                format_ident!("{}", &data.measurement_system.duration_measurement_system);
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
                    energy_unit_variant: #energy_unit_variant,
                    duration_unit_variant: #duration_unit_variant,
                    energy_measurement_system: #energy_measurement_system,
                    duration_measurement_system: #duration_measurement_system,
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
        define_power_units! {
            all: { #(#variants_all),* },
            json: { #(#variants),* },
        }
    };

    expanded.into()
}
