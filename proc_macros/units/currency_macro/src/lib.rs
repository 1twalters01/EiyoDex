extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::Path};
use syn::LitStr;

#[derive(Debug, Deserialize)]
struct CurrencyJson {
    symbol: String,
    code: String,
    unit_type: String,
    unit_type_plural: String,
}

#[proc_macro]
pub fn include_currencies_from_json(input: TokenStream) -> TokenStream {
    let mut currencies: HashMap<String, CurrencyJson> = HashMap::new();

    let file_paths = syn::parse_macro_input!(input with syn::punctuated::Punctuated::<LitStr, syn::Token![,]>::parse_terminated);
    for file_path_lit in file_paths.iter() {
        let rel_path = file_path_lit.value();

        let manifest_dir = env::var("WORKSPACE_ROOT").expect("WORKSPACE_ROOT not set");
        let full_path = Path::new(&manifest_dir).join(rel_path);

        let file_content = fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Unable to read file: {}", full_path.display()));

        let json_results: HashMap<String, CurrencyJson> =
            serde_json::from_str(&file_content).expect("Invalid JSON format");

        for (key, value) in json_results {
            currencies.insert(key, value);
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
        define_currencies! {
            #(#variants),*
        }
    };

    expanded.into()
}
