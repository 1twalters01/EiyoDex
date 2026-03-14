mod unit;

use proc_macro::TokenStream;

#[proc_macro]
pub fn include_nutrient_units_from_json(input: TokenStream) -> TokenStream {
    unit::generate_nutrient_units(input)
}

