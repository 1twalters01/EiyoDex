extern crate proc_macro;

use glob::glob;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::Path};
use syn::{LitStr, parse::Parser};

#[derive(Debug, Deserialize)]
struct NutrientUnitJson {
    identifier: String,
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
}

fn populate_nutrient_units(input: TokenStream) -> HashMap<String, NutrientUnitJson> {
    let mut nutrient_unit: HashMap<String, NutrientUnitJson> = HashMap::new();

    let parser = syn::punctuated::Punctuated::<LitStr, syn::Token![,]>::parse_terminated;
    let file_paths = parser
        .parse(input)
        .expect("Failed to parse input as comma-separated string literals");

    for file_path_lit in file_paths.iter() {
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

            let json_results: HashMap<String, NutrientUnitJson> =
                serde_json::from_str(&file_content).expect("Invalid JSON format");

            for (key, value) in json_results {
                nutrient_unit.insert(key, value);
            }
        }
    }

    return nutrient_unit;
}

pub fn generate_nutrient_units(input: TokenStream) -> TokenStream {
    let nutrient_units = populate_nutrient_units(input);
    // if nutrient_units.is_empty() {
    //     panic!("include_nutrient_units_from_json! found no JSON nutrient unit
    // files"); }

    let variants = nutrient_units.iter().map(|(key, data)| {
        let variant = format_ident!("{}", key);
        let symbol = &data.symbol;
        let symbol_lc = &data.symbol.to_lowercase();
        let unit_type = &data.unit_type;
        let unit_type_lc = &data.unit_type.to_lowercase();
        let unit_type_plural = &data.unit_type_plural;
        let unit_type_plural_lc = &data.unit_type_plural.to_lowercase();
        let identifier_lc = &data.identifier.to_lowercase();

        quote! {
            #variant => {
                symbol: #symbol,
                symbol_lc: #symbol_lc,
                unit_type: #unit_type,
                unit_type_lc: #unit_type_lc,
                unit_type_plural: #unit_type_plural,
                unit_type_plural_lc: #unit_type_plural_lc,
                identifier_lc: #identifier_lc,
            }
        }
    });

    let expanded = quote! {
        define_nutrient_units! {
            #(#variants),*
        }
    };

    expanded.into()
}

pub fn generate_nutrient_units_c(input: TokenStream) -> TokenStream {
    let masses = populate_nutrient_units(input);

    let variants = masses.iter().map(|(key, _)| {
        let variant = format_ident!("{}", key);

        quote! {
            #variant
        }
    });

    let expanded = quote! {
        define_nutrient_units_c! {
            #(#variants),*
        }
    };

    expanded.into()
}
