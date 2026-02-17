extern crate proc_macro;

use glob::glob;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::Path};
use syn::{LitStr, parse::Parser};

#[derive(Debug, Deserialize)]
pub struct CurrencyJson {
    symbol: String,
    code: String,
    unit_type: String,
    unit_type_plural: String,
}

pub fn populate_currencies(input: TokenStream) -> HashMap<String, CurrencyJson> {
    let mut currencies: HashMap<String, CurrencyJson> = HashMap::new();

    // let file_paths = syn::parse_macro_input!(input with
    // syn::punctuated::Punctuated::<LitStr, syn::Token![,]>::parse_terminated);
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

            let json_results: HashMap<String, CurrencyJson> =
                serde_json::from_str(&file_content).expect("Invalid JSON format");

            for (key, value) in json_results {
                currencies.insert(key, value);
            }
        }
    }

    return currencies;
}

pub fn generate(input: TokenStream) -> TokenStream {
    let currencies = populate_currencies(input);

    let variants = currencies.iter().map(|(key, data)| {
        let from_fn_name = format_ident!("from_{}", data.code.to_lowercase());
        let as_fn_name = format_ident!("as_{}", data.code.to_lowercase());
        let to_fn_name = format_ident!("to_{}", data.code.to_lowercase());
        let variant = format_ident!("{}", key);

        quote! {
            #variant => {
                from_fn_name: #from_fn_name,
                as_fn_name: #as_fn_name,
                to_fn_name: #to_fn_name,
            }
        }
    });

    let expanded = quote! {
        define_currencies! {
            #(#variants),*
        }
    };

    expanded.into()
}

pub fn generate_units(input: TokenStream) -> TokenStream {
    let mut currencies: HashMap<String, CurrencyJson> = HashMap::new();

    let file_paths = syn::parse_macro_input!(input with syn::punctuated::Punctuated::<LitStr, syn::Token![,]>::parse_terminated);
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

            let json_results: HashMap<String, CurrencyJson> =
                serde_json::from_str(&file_content).expect("Invalid JSON format");

            for (key, value) in json_results {
                currencies.insert(key, value);
            }
        }
    }

    let variants = currencies.iter().map(|(key, data)| {
        let variant = format_ident!("{}", key);
        let symbol = &data.symbol;
        let code = &data.code;
        let unit_type = &data.unit_type;
        let unit_type_plural = &data.unit_type_plural;

        quote! {
            #variant => {
                symbol: #symbol,
                code: #code,
                unit_type: #unit_type,
                unit_type_plural: #unit_type_plural
            }
        }
    });

    let expanded = quote! {
        define_currency_units! {
            #(#variants),*
        }
    };

    expanded.into()
}
