mod currency;
mod density;
mod distance;
mod duration;
mod energy;
mod mass;
mod measurement_system;
mod power;
mod specific_currency;
mod volume;

use proc_macro::TokenStream;

#[proc_macro]
pub fn include_measurement_systems_from_json(input: TokenStream) -> TokenStream {
    measurement_system::generate(input)
}

#[proc_macro]
pub fn include_masses_from_json(input: TokenStream) -> TokenStream {
    mass::generate(input)
}

#[proc_macro]
pub fn include_mass_units_from_json(input: TokenStream) -> TokenStream {
    mass::generate_units(input)
}

#[proc_macro]
pub fn include_mass_units_c_from_json(input: TokenStream) -> TokenStream {
    mass::generate_units_c(input)
}

#[proc_macro]
pub fn include_volumes_from_json(input: TokenStream) -> TokenStream {
    volume::generate(input)
}

#[proc_macro]
pub fn include_volume_units_from_json(input: TokenStream) -> TokenStream {
    volume::generate_units(input)
}
#[proc_macro]
pub fn include_volume_units_c_from_json(input: TokenStream) -> TokenStream {
    volume::generate_units_c(input)
}

#[proc_macro]
pub fn include_energies_from_json(input: TokenStream) -> TokenStream {
    energy::generate(input)
}

#[proc_macro]
pub fn include_energy_units_from_json(input: TokenStream) -> TokenStream {
    energy::generate_units(input)
}

#[proc_macro]
pub fn include_energy_units_c_from_json(input: TokenStream) -> TokenStream {
    energy::generate_units_c(input)
}

#[proc_macro]
pub fn include_distances_from_json(input: TokenStream) -> TokenStream {
    distance::generate(input)
}

#[proc_macro]
pub fn include_distance_units_from_json(input: TokenStream) -> TokenStream {
    distance::generate_units(input)
}

#[proc_macro]
pub fn include_durations_from_json(input: TokenStream) -> TokenStream {
    duration::generate(input)
}

#[proc_macro]
pub fn include_duration_units_from_json(input: TokenStream) -> TokenStream {
    duration::generate_units(input)
}

#[proc_macro]
pub fn include_currencies_from_json(input: TokenStream) -> TokenStream {
    currency::generate(input)
}

#[proc_macro]
pub fn include_currency_units_from_json(input: TokenStream) -> TokenStream {
    currency::generate_units(input)
}

#[proc_macro]
pub fn include_densities_from_json(input: TokenStream) -> TokenStream {
    density::generate(input)
}

#[proc_macro]
pub fn include_density_units_from_json(input: TokenStream) -> TokenStream {
    density::generate_units(input)
}

#[proc_macro]
pub fn include_powers_from_json(input: TokenStream) -> TokenStream {
    power::generate(input)
}

#[proc_macro]
pub fn include_power_units_from_json(input: TokenStream) -> TokenStream {
    power::generate_units(input)
}

#[proc_macro]
pub fn include_specific_currencies_from_json(input: TokenStream) -> TokenStream {
    specific_currency::generate(input)
}

#[proc_macro]
pub fn include_specific_currency_units_from_json(input: TokenStream) -> TokenStream {
    specific_currency::generate_units(input)
}
