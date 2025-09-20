extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::Path};
use syn::LitStr;

#[derive(Debug, Deserialize)]
struct MassJson {
    symbol: String,
    unit_type: String,
    unit_type_plural: String,
    factor: f64,
}

#[proc_macro]
pub fn include_masses_from_json(input: TokenStream) -> TokenStream {
    let file_path_lit = syn::parse_macro_input!(input as LitStr);
    let rel_path = file_path_lit.value();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let full_path = Path::new(&manifest_dir).join(rel_path);

    let file_content =
        fs::read_to_string(&full_path).unwrap_or_else(|_| panic!("Unable to read file: {}", full_path.display()));

    let currencies: HashMap<String, MassJson> =
        serde_json::from_str(&file_content).expect("Invalid JSON format");

    let variants = currencies.iter().map(|(key, data)| {
        let variant = format_ident!("{}", key);
        let from_fn_name = format_ident!("from_{}", data.symbol);
        let as_fn_name = format_ident!("as_{}", data.symbol);
        let symbol = &data.symbol;
        let unit_type = &data.unit_type;
        let unit_type_plural = &data.unit_type_plural;
        let factor = &data.factor;

        quote! {
            #variant => {
                from_fn_name: #from_fn_name,
                as_fn_name: #as_fn_name,
                symbol: #symbol,
                unit_type: #unit_type,
                unit_type_plural: #unit_type_plural,
                factor: #factor
            }
        }
    });

    let expanded = quote! {
        define_masses! {
            #(#variants),*
        }
    };

    expanded.into()
}

