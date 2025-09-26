extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::Path};
use syn::LitStr;

#[derive(Debug, Deserialize)]
struct DistanceJson {
    identifier: String,
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
    measurement_system: String,
    si_factor: f64,
}

#[proc_macro]
pub fn include_distances_from_json(input: TokenStream) -> TokenStream {
    let mut distances: HashMap<String, DistanceJson> = HashMap::new();

    let file_paths = syn::parse_macro_input!(input with syn::punctuated::Punctuated::<LitStr, syn::Token![,]>::parse_terminated);
    for file_path_lit in file_paths.iter() {
        let rel_path = file_path_lit.value();

        let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
        let full_path = Path::new(&manifest_dir).join(rel_path);

        let file_content =
        fs::read_to_string(&full_path).unwrap_or_else(|_| panic!("Unable to read file: {}", full_path.display()));

        let json_results: HashMap<String, DistanceJson> = serde_json::from_str(&file_content).expect("Invalid JSON format");

        for (key, value) in json_results {
            distances.insert(key, value);
        }
    }

    let variants = distances.iter().map(|(key, data)| {
        let variant = format_ident!("{}", key);
        let from_fn_name = format_ident!("from_{}", data.symbol);
        let as_fn_name = format_ident!("as_{}", data.symbol);
        let to_fn_name = format_ident!("to_{}", data.symbol);
        let measurement_system = format_ident!("{}", data.measurement_system);
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
                measurement_system: #measurement_system,
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
    });

    let expanded = quote! {
        define_distances! {
            #(#variants),*
        }
    };

    expanded.into()
}

