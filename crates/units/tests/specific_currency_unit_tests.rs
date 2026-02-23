use std::{collections::BTreeSet, str::FromStr};
use units::{
    currency::unit::CurrencyUnit,
    mass::unit::MassUnit,
    measurement_system::MeasurementSystem,
    specific_currency::unit::{Denominator, DenominatorType, SpecificCurrencyUnit},
    volume::unit::VolumeUnit,
};

#[test]
fn test_from_variants() {
    let usd = CurrencyUnit::USD;
    let gbp = CurrencyUnit::GBP;
    let eur = CurrencyUnit::EUR;
    let jpy = CurrencyUnit::JPY;

    let gram = Denominator::MassDenominator(MassUnit::Gram);
    let kilogram = Denominator::MassDenominator(MassUnit::Kilogram);
    let milligram = Denominator::MassDenominator(MassUnit::Milligram);
    let microgram = Denominator::MassDenominator(MassUnit::Microgram);
    let ounce = Denominator::MassDenominator(MassUnit::Ounce);

    let liter = Denominator::VolumeDenominator(VolumeUnit::Liter);
    let milliliter = Denominator::VolumeDenominator(VolumeUnit::Milliliter);
    let pint = Denominator::VolumeDenominator(VolumeUnit::Pint);
    let gallon = Denominator::VolumeDenominator(VolumeUnit::Gallon);
    let fluid_ounce = Denominator::VolumeDenominator(VolumeUnit::FluidOunce);
    let tablespoon = Denominator::VolumeDenominator(VolumeUnit::Tablespoon);
    let teaspoon = Denominator::VolumeDenominator(VolumeUnit::Teaspoon);

    assert_eq!(
        SpecificCurrencyUnit::from_variants(usd, gram),
        SpecificCurrencyUnit::USDPerGram
    );
    assert_eq!(
        SpecificCurrencyUnit::from_variants(usd, kilogram),
        SpecificCurrencyUnit::USDPerKilogram
    );
    assert_eq!(
        SpecificCurrencyUnit::from_variants(gbp, milligram),
        SpecificCurrencyUnit::GBPPerMilligram
    );
    assert_eq!(
        SpecificCurrencyUnit::from_variants(gbp, microgram),
        SpecificCurrencyUnit::GBPPerMicrogram
    );
    assert_eq!(
        SpecificCurrencyUnit::from_variants(eur, ounce),
        SpecificCurrencyUnit::EURPerOunce
    );
    assert_eq!(
        SpecificCurrencyUnit::from_variants(eur, liter),
        SpecificCurrencyUnit::EURPerLiter
    );
    assert_eq!(
        SpecificCurrencyUnit::from_variants(jpy, milliliter),
        SpecificCurrencyUnit::JPYPerMilliliter
    );

    assert_eq!(
        SpecificCurrencyUnit::from_variants(jpy, pint),
        SpecificCurrencyUnit::JPYPerPint
    );
    assert_eq!(
        SpecificCurrencyUnit::from_variants(usd, gallon),
        SpecificCurrencyUnit::USDPerGallon
    );
    assert_eq!(
        SpecificCurrencyUnit::from_variants(usd, fluid_ounce),
        SpecificCurrencyUnit::USDPerFluidOunce
    );
    assert_eq!(
        SpecificCurrencyUnit::from_variants(gbp, tablespoon),
        SpecificCurrencyUnit::GBPPerTablespoon
    );
    assert_eq!(
        SpecificCurrencyUnit::from_variants(gbp, teaspoon),
        SpecificCurrencyUnit::GBPPerTeaspoon
    );
}

#[test]
fn test_get_all_specific_currency_unit_enumerations() {
    let function_enumerations = SpecificCurrencyUnit::get_all_enumerations();
    let manual_enumerations = &vec![
        SpecificCurrencyUnit::USDPerGram,
        SpecificCurrencyUnit::USDPerKilogram,
        SpecificCurrencyUnit::USDPerMilligram,
        SpecificCurrencyUnit::USDPerMicrogram,
        SpecificCurrencyUnit::USDPerOunce,
        SpecificCurrencyUnit::USDPerLiter,
        SpecificCurrencyUnit::USDPerMilliliter,
        SpecificCurrencyUnit::USDPerPint,
        SpecificCurrencyUnit::USDPerGallon,
        SpecificCurrencyUnit::USDPerFluidOunce,
        SpecificCurrencyUnit::USDPerTablespoon,
        SpecificCurrencyUnit::USDPerTeaspoon,
        SpecificCurrencyUnit::GBPPerGram,
        SpecificCurrencyUnit::GBPPerKilogram,
        SpecificCurrencyUnit::GBPPerMilligram,
        SpecificCurrencyUnit::GBPPerMicrogram,
        SpecificCurrencyUnit::GBPPerOunce,
        SpecificCurrencyUnit::GBPPerLiter,
        SpecificCurrencyUnit::GBPPerMilliliter,
        SpecificCurrencyUnit::GBPPerPint,
        SpecificCurrencyUnit::GBPPerGallon,
        SpecificCurrencyUnit::GBPPerFluidOunce,
        SpecificCurrencyUnit::GBPPerTablespoon,
        SpecificCurrencyUnit::GBPPerTeaspoon,
        SpecificCurrencyUnit::EURPerGram,
        SpecificCurrencyUnit::EURPerKilogram,
        SpecificCurrencyUnit::EURPerMilligram,
        SpecificCurrencyUnit::EURPerMicrogram,
        SpecificCurrencyUnit::EURPerOunce,
        SpecificCurrencyUnit::EURPerLiter,
        SpecificCurrencyUnit::EURPerMilliliter,
        SpecificCurrencyUnit::EURPerPint,
        SpecificCurrencyUnit::EURPerGallon,
        SpecificCurrencyUnit::EURPerFluidOunce,
        SpecificCurrencyUnit::EURPerTablespoon,
        SpecificCurrencyUnit::EURPerTeaspoon,
        SpecificCurrencyUnit::JPYPerGram,
        SpecificCurrencyUnit::JPYPerKilogram,
        SpecificCurrencyUnit::JPYPerMilligram,
        SpecificCurrencyUnit::JPYPerMicrogram,
        SpecificCurrencyUnit::JPYPerOunce,
        SpecificCurrencyUnit::JPYPerLiter,
        SpecificCurrencyUnit::JPYPerMilliliter,
        SpecificCurrencyUnit::JPYPerPint,
        SpecificCurrencyUnit::JPYPerGallon,
        SpecificCurrencyUnit::JPYPerFluidOunce,
        SpecificCurrencyUnit::JPYPerTablespoon,
        SpecificCurrencyUnit::JPYPerTeaspoon,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_selected_density_unit_enumerations() {
    let function_enumerations = SpecificCurrencyUnit::get_selected_enumerations();
    let manual_enumerations = vec![
        &SpecificCurrencyUnit::USDPerOunce,
        &SpecificCurrencyUnit::USDPerPint,
        &SpecificCurrencyUnit::GBPPerKilogram,
        &SpecificCurrencyUnit::GBPPerLiter,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(SpecificCurrencyUnit::USDPerGram.as_symbol(), "$/g");
    assert_eq!(SpecificCurrencyUnit::GBPPerKilogram.as_symbol(), "£/kg");
    assert_eq!(SpecificCurrencyUnit::EURPerMilligram.as_symbol(), "€/mg");
    assert_eq!(SpecificCurrencyUnit::JPYPerMicrogram.as_symbol(), "¥/ug");
    assert_eq!(SpecificCurrencyUnit::USDPerOunce.as_symbol(), "$/oz");
    assert_eq!(SpecificCurrencyUnit::GBPPerLiter.as_symbol(), "£/L");
    assert_eq!(SpecificCurrencyUnit::EURPerMilliliter.as_symbol(), "€/mL");
    assert_eq!(SpecificCurrencyUnit::JPYPerPint.as_symbol(), "¥/pt");
    assert_eq!(SpecificCurrencyUnit::USDPerGallon.as_symbol(), "$/gal");
    assert_eq!(
        SpecificCurrencyUnit::GBPPerFluidOunce.as_symbol(),
        "£/fl oz"
    );
    assert_eq!(SpecificCurrencyUnit::EURPerTablespoon.as_symbol(), "€/tbsp");
    assert_eq!(SpecificCurrencyUnit::JPYPerTeaspoon.as_symbol(), "¥/tsp");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(
        SpecificCurrencyUnit::USDPerGram.as_unit_type(),
        "dollar per gram"
    );
    assert_eq!(
        SpecificCurrencyUnit::GBPPerKilogram.as_unit_type(),
        "pound per kilogram"
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerMilligram.as_unit_type(),
        "euro per milligram"
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerMicrogram.as_unit_type(),
        "yen per microgram"
    );
    assert_eq!(
        SpecificCurrencyUnit::USDPerOunce.as_unit_type(),
        "dollar per ounce"
    );
    assert_eq!(
        SpecificCurrencyUnit::GBPPerLiter.as_unit_type(),
        "pound per liter"
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerMilliliter.as_unit_type(),
        "euro per milliliter"
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerPint.as_unit_type(),
        "yen per pint"
    );
    assert_eq!(
        SpecificCurrencyUnit::USDPerGallon.as_unit_type(),
        "dollar per gallon"
    );
    assert_eq!(
        SpecificCurrencyUnit::GBPPerFluidOunce.as_unit_type(),
        "pound per fluid ounce"
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerTablespoon.as_unit_type(),
        "euro per tablespoon"
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerTeaspoon.as_unit_type(),
        "yen per teaspoon"
    );
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(
        SpecificCurrencyUnit::USDPerGram.as_unit_type_plural(),
        "dollars per gram"
    );
    assert_eq!(
        SpecificCurrencyUnit::GBPPerKilogram.as_unit_type_plural(),
        "pounds per kilogram"
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerMilligram.as_unit_type_plural(),
        "euros per milligram"
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerMicrogram.as_unit_type_plural(),
        "yen per microgram"
    );
    assert_eq!(
        SpecificCurrencyUnit::USDPerOunce.as_unit_type_plural(),
        "dollars per ounce"
    );
    assert_eq!(
        SpecificCurrencyUnit::GBPPerLiter.as_unit_type_plural(),
        "pounds per liter"
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerMilliliter.as_unit_type_plural(),
        "euros per milliliter"
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerPint.as_unit_type_plural(),
        "yen per pint"
    );
    assert_eq!(
        SpecificCurrencyUnit::USDPerGallon.as_unit_type_plural(),
        "dollars per gallon"
    );
    assert_eq!(
        SpecificCurrencyUnit::GBPPerFluidOunce.as_unit_type_plural(),
        "pounds per fluid ounce"
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerTablespoon.as_unit_type_plural(),
        "euros per tablespoon"
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerTeaspoon.as_unit_type_plural(),
        "yen per teaspoon"
    );
}

#[test]
fn test_get_measurement_system() {
    let metric = MeasurementSystem::Metric;
    let imperial = MeasurementSystem::Imperial;

    assert_eq!(
        SpecificCurrencyUnit::GBPPerKilogram.get_measurement_system(),
        metric
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerOunce.get_measurement_system(),
        imperial
    );
    assert_eq!(
        SpecificCurrencyUnit::USDPerPint.get_measurement_system(),
        imperial
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerLiter.get_measurement_system(),
        metric
    );
}

#[test]
fn test_get_denominator_type() {
    let volume_denominator = DenominatorType::VolumeDenominator;
    let mass_denominator = DenominatorType::MassDenominator;

    assert_eq!(
        SpecificCurrencyUnit::USDPerGram.get_denominator_type(),
        mass_denominator
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerLiter.get_denominator_type(),
        volume_denominator
    )
}

#[test]
fn test_get_currency_unit() {
    assert_eq!(
        SpecificCurrencyUnit::USDPerGram.get_currency_unit(),
        CurrencyUnit::USD
    );
    assert_eq!(
        SpecificCurrencyUnit::GBPPerOunce.get_currency_unit(),
        CurrencyUnit::GBP
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerLiter.get_currency_unit(),
        CurrencyUnit::EUR
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerFluidOunce.get_currency_unit(),
        CurrencyUnit::JPY
    );
}

#[test]
fn test_get_denominator_unit() {
    assert_eq!(
        SpecificCurrencyUnit::USDPerGram.get_denominator(),
        Denominator::MassDenominator(MassUnit::Gram)
    );
    assert_eq!(
        SpecificCurrencyUnit::GBPPerKilogram.get_denominator(),
        Denominator::MassDenominator(MassUnit::Kilogram)
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerMilligram.get_denominator(),
        Denominator::MassDenominator(MassUnit::Milligram)
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerMicrogram.get_denominator(),
        Denominator::MassDenominator(MassUnit::Microgram)
    );
    assert_eq!(
        SpecificCurrencyUnit::USDPerOunce.get_denominator(),
        Denominator::MassDenominator(MassUnit::Ounce)
    );
    assert_eq!(
        SpecificCurrencyUnit::GBPPerLiter.get_denominator(),
        Denominator::VolumeDenominator(VolumeUnit::Liter)
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerMilliliter.get_denominator(),
        Denominator::VolumeDenominator(VolumeUnit::Milliliter)
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerPint.get_denominator(),
        Denominator::VolumeDenominator(VolumeUnit::Pint)
    );
    assert_eq!(
        SpecificCurrencyUnit::USDPerGallon.get_denominator(),
        Denominator::VolumeDenominator(VolumeUnit::Gallon)
    );
    assert_eq!(
        SpecificCurrencyUnit::GBPPerFluidOunce.get_denominator(),
        Denominator::VolumeDenominator(VolumeUnit::FluidOunce)
    );
    assert_eq!(
        SpecificCurrencyUnit::EURPerTablespoon.get_denominator(),
        Denominator::VolumeDenominator(VolumeUnit::Tablespoon)
    );
    assert_eq!(
        SpecificCurrencyUnit::JPYPerTeaspoon.get_denominator(),
        Denominator::VolumeDenominator(VolumeUnit::Teaspoon)
    );
}

#[test]
fn get_si_factor() {
    let percentage_error = 0.005;

    assert!((SpecificCurrencyUnit::USDPerKilogram.si_factor() - 1f64) / 1f64 < percentage_error);
    assert!(
        (SpecificCurrencyUnit::GBPPerKilogram.si_factor() - 1f64 * 1.35) / 1f64 < percentage_error
    );
    assert!(
        (SpecificCurrencyUnit::USDPerMilliliter.si_factor() - 1_000_000f64) / 1_000_000f64
            < percentage_error
    );
}

#[test]
fn test_from_str() {
    assert_eq!(
        SpecificCurrencyUnit::from_str("dollars per gram").unwrap(),
        SpecificCurrencyUnit::USDPerGram
    );
    assert_eq!(
        SpecificCurrencyUnit::from_str("euro per liter").unwrap(),
        SpecificCurrencyUnit::EURPerLiter
    );
    assert_eq!(
        SpecificCurrencyUnit::from_str("¥/pt").unwrap(),
        SpecificCurrencyUnit::JPYPerPint
    );
    assert_eq!(
        SpecificCurrencyUnit::from_str("gbp/pt").unwrap(),
        SpecificCurrencyUnit::GBPPerPint
    );
}
