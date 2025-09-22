extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{collections::{HashMap, HashSet}, env, fs, path::Path};
use syn::LitStr;

#[derive(Debug, Deserialize)]
struct MassJson {
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
    measurement_system: String,
    grams_factor: f64,
}

#[proc_macro]
pub fn include_measurement_systems_from_json(input: TokenStream) -> TokenStream {
    let file_path_lit = syn::parse_macro_input!(input as LitStr);
    let rel_path = file_path_lit.value();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let full_path = Path::new(&manifest_dir).join(rel_path);

    let file_content =
        fs::read_to_string(&full_path).unwrap_or_else(|_| panic!("Unable to read file: {}", full_path.display()));

    let json_results: HashMap<String, MassJson> =
        serde_json::from_str(&file_content).expect("Invalid JSON format");

    let mut measurement_systems = HashSet::new(); 
    for data in json_results.values() {
        measurement_systems.insert(data.measurement_system.clone());
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

