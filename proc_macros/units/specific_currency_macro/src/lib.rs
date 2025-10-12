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
struct SpecificCurrency {
    identifier: String,
    currency_unit: String,
    denominator_unit: String,
    denominator_unit_type: String,
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
    denominator_measurement_system: String,
    si_factor: f64,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
struct SpecificCurrencyJson {
    currency_unit: String,
    denominator_unit: String,
}

#[derive(Debug, Deserialize)]
struct CurrencyJson {
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
}

#[derive(Debug, Deserialize)]
struct DenominatorJson {
    identifier: String,
    symbol: String,
    unit_type: String,
    measurement_system: String,
    si_factor: f64,
}

#[proc_macro]
pub fn include_specific_currencies_from_json(input: TokenStream) -> TokenStream {
    let mut specific_currency_all: HashMap<String, SpecificCurrency> = HashMap::new();
    let mut specific_currency: HashMap<String, SpecificCurrency> = HashMap::new();
    let mut specific_currency_mass: HashMap<String, SpecificCurrency> = HashMap::new();
    let mut specific_currency_volume: HashMap<String, SpecificCurrency> = HashMap::new();

    let mut currency_data: HashMap<String, CurrencyJson> = HashMap::new();
    let mut denominator_data: HashMap<String, (DenominatorJson, String)> = HashMap::new();
    let mut specific_currency_data: HashSet<SpecificCurrencyJson> = HashSet::new();

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
                    "SpecificCurrencyUnit" => {
                        let serde_res: Vec<SpecificCurrencyJson> =
                            serde_json::from_str(&file_content).expect("Invalid JSON format");
                        specific_currency_data.extend(serde_res);
                    }
                    "CurrencyUnit" => {
                        let serde_res: HashMap<String, CurrencyJson> =
                            serde_json::from_str(&file_content).expect("Invalid JSON format");
                        for (key, data) in serde_res {
                            currency_data.insert(key, data);
                        }
                    }
                    "MassUnit" => {
                        let serde_res: HashMap<String, DenominatorJson> =
                            serde_json::from_str(&file_content).expect("Invalid JSON format");
                        for (key, data) in serde_res {
                            denominator_data
                                .insert(key.clone(), (data, "MassDenominator".to_string()));
                        }
                    }
                    "VolumeUnit" => {
                        let serde_res: HashMap<String, DenominatorJson> =
                            serde_json::from_str(&file_content).expect("Invalid JSON format");
                        for (key, data) in serde_res {
                            denominator_data
                                .insert(key.clone(), (data, "VolumeDenominator".to_string()));
                        }
                    }
                    _ => panic!("Incorrect unit"),
                };
            }
        }
    }

    for (currency_key, currency_value) in &currency_data {
        for (denominator_key, (denominator_value, denominator_unit_type)) in &denominator_data {
            let specific_currency_variant = format!("{}Per{}", currency_key, denominator_key);

            let specific_currency_identifier = format!(
                "{}_per_{}",
                currency_key.to_lowercase(),
                denominator_value.identifier
            );
            let specific_currency_symbol =
                format!("{}/{}", currency_value.symbol, denominator_value.symbol);
            let specific_unit_type = format!(
                "{} per {}",
                currency_value.unit_type, denominator_value.unit_type
            );
            let specific_currency_unit_type_plural = format!(
                "{} per {}",
                currency_value.unit_type_plural, denominator_value.unit_type
            );

            let denominator_measurement_system = denominator_value.measurement_system.clone();
            let specific_currency_si_factor = 1f64 / denominator_value.si_factor;

            let specific_currency_object = SpecificCurrency {
                identifier: specific_currency_identifier.clone(),
                currency_unit: currency_key.clone(),
                denominator_unit: denominator_key.clone(),
                denominator_unit_type: denominator_unit_type.clone(),
                symbol: specific_currency_symbol.clone(),
                unit_type: specific_unit_type.clone(),
                unit_type_plural: specific_currency_unit_type_plural.clone(),
                denominator_measurement_system: denominator_measurement_system.clone(),
                si_factor: specific_currency_si_factor,
            };

            specific_currency_all.insert(
                specific_currency_variant.clone(),
                specific_currency_object.clone(),
            );

            if specific_currency_data.contains(&SpecificCurrencyJson {
                currency_unit: currency_key.clone(),
                denominator_unit: denominator_key.clone(),
            }) {
                specific_currency.insert(
                    specific_currency_variant.clone(),
                    specific_currency_object.clone(),
                );
            }

            if denominator_unit_type == "MassDenominator" {
                specific_currency_mass.insert(specific_currency_variant, specific_currency_object);
            } else if denominator_unit_type == "VolumeDenominator" {
                specific_currency_volume
                    .insert(specific_currency_variant, specific_currency_object);
            }
        }
    }

    let variants: Vec<_> = specific_currency
        .iter()
        .map(|(key, data)| {
            let json_variant = format_ident!("{}", key);
            let from_fn_name = format_ident!("from_{}", data.identifier);
            let as_fn_name = format_ident!("as_{}", data.identifier);
            let to_fn_name = format_ident!("to_{}", data.identifier);
            let currency_unit_variant = format_ident!("{}", &data.currency_unit);
            let denominator_unit_variant = format_ident!("{}", &data.denominator_unit);
            let denominator_unit_type = format_ident!("{}", &data.denominator_unit_type);
            let denominator_measurement_system =
                format_ident!("{}", &data.denominator_measurement_system);
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
                    from_fn_name: #from_fn_name,
                    as_fn_name: #as_fn_name,
                    to_fn_name: #to_fn_name,
                    currency_unit_variant: #currency_unit_variant,
                    denominator_unit_variant: #denominator_unit_variant,
                    denominator_unit_type: #denominator_unit_type,
                    denominator_measurement_system: #denominator_measurement_system,
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

    let variants_all: Vec<_> = specific_currency_all
        .iter()
        .map(|(key, data)| {
            let all_variant = format_ident!("{}", key);
            let from_fn_name = format_ident!("from_{}", data.identifier);
            let as_fn_name = format_ident!("as_{}", data.identifier);
            let to_fn_name = format_ident!("to_{}", data.identifier);
            let currency_unit_variant = format_ident!("{}", &data.currency_unit);
            let denominator_unit_variant = format_ident!("{}", &data.denominator_unit);
            let denominator_unit_type = format_ident!("{}", &data.denominator_unit_type);
            let denominator_measurement_system =
                format_ident!("{}", &data.denominator_measurement_system);
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
                    from_fn_name: #from_fn_name,
                    as_fn_name: #as_fn_name,
                    to_fn_name: #to_fn_name,
                    currency_unit_variant: #currency_unit_variant,
                    denominator_unit_variant: #denominator_unit_variant,
                    denominator_unit_type: #denominator_unit_type,
                    denominator_measurement_system: #denominator_measurement_system,
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

    let variants_mass: Vec<_> = specific_currency_mass
        .iter()
        .map(|(key, data)| {
            let mass_variant = format_ident!("{}", key);
            let from_fn_name = format_ident!("from_{}", data.identifier);
            let as_fn_name = format_ident!("as_{}", data.identifier);
            let to_fn_name = format_ident!("to_{}", data.identifier);
            let currency_unit_variant = format_ident!("{}", &data.currency_unit);
            let denominator_unit_variant = format_ident!("{}", &data.denominator_unit);
            let denominator_unit_type = format_ident!("{}", &data.denominator_unit_type);
            let denominator_measurement_system =
                format_ident!("{}", &data.denominator_measurement_system);
            let symbol = &data.symbol;
            let symbol_lc = data.symbol.to_lowercase();
            let unit_type = &data.unit_type;
            let unit_type_lc = data.unit_type.to_lowercase();
            let unit_type_plural = &data.unit_type_plural;
            let unit_type_plural_lc = data.unit_type_plural.to_lowercase();
            let identifier_lc = data.identifier.to_lowercase();
            let si_factor = &data.si_factor;

            quote! {
                #mass_variant => {
                    from_fn_name: #from_fn_name,
                    as_fn_name: #as_fn_name,
                    to_fn_name: #to_fn_name,
                    currency_unit_variant: #currency_unit_variant,
                    denominator_unit_variant: #denominator_unit_variant,
                    denominator_unit_type: #denominator_unit_type,
                    denominator_measurement_system: #denominator_measurement_system,
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

    let variants_volume: Vec<_> = specific_currency_volume
        .iter()
        .map(|(key, data)| {
            let volume_variant = format_ident!("{}", key);
            let from_fn_name = format_ident!("from_{}", data.identifier);
            let as_fn_name = format_ident!("as_{}", data.identifier);
            let to_fn_name = format_ident!("to_{}", data.identifier);
            let currency_unit_variant = format_ident!("{}", &data.currency_unit);
            let denominator_unit_variant = format_ident!("{}", &data.denominator_unit);
            let denominator_unit_type = format_ident!("{}", &data.denominator_unit_type);
            let denominator_measurement_system =
                format_ident!("{}", &data.denominator_measurement_system);
            let symbol = &data.symbol;
            let symbol_lc = data.symbol.to_lowercase();
            let unit_type = &data.unit_type;
            let unit_type_lc = data.unit_type.to_lowercase();
            let unit_type_plural = &data.unit_type_plural;
            let unit_type_plural_lc = data.unit_type_plural.to_lowercase();
            let identifier_lc = data.identifier.to_lowercase();
            let si_factor = &data.si_factor;

            quote! {
                #volume_variant => {
                    from_fn_name: #from_fn_name,
                    as_fn_name: #as_fn_name,
                    to_fn_name: #to_fn_name,
                    currency_unit_variant: #currency_unit_variant,
                    denominator_unit_variant: #denominator_unit_variant,
                    denominator_unit_type: #denominator_unit_type,
                    denominator_measurement_system: #denominator_measurement_system,
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
        define_specific_currencies! {
            all: { #(#variants_all),* },
            json: { #(#variants),* },
            mass: { #(#variants_mass),* },
            volume: { #(#variants_volume),* },
        }
    };

    expanded.into()
}
