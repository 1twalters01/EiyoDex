use nutrients::nutrient_units::NutrientUnit;
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};

#[test]
pub fn test_get_enumerations() {
    let mut enumerations = NutrientUnit::get_enumerations();
    let mut manual_enumerations = Vec::from([
        NutrientUnit::Mass(MassUnit::Microgram),
        NutrientUnit::Mass(MassUnit::Milligram),
        NutrientUnit::Mass(MassUnit::Gram),
        NutrientUnit::Mass(MassUnit::Kilogram),
        NutrientUnit::Mass(MassUnit::Ounce),
        NutrientUnit::Volume(VolumeUnit::Liter),
        NutrientUnit::Volume(VolumeUnit::Milliliter),
        NutrientUnit::Volume(VolumeUnit::Pint),
        NutrientUnit::Volume(VolumeUnit::Gallon),
        NutrientUnit::Volume(VolumeUnit::FluidOunce),
        NutrientUnit::Volume(VolumeUnit::Tablespoon),
        NutrientUnit::Volume(VolumeUnit::Teaspoon),
        NutrientUnit::Energy(EnergyUnit::Kilojoule),
        NutrientUnit::Energy(EnergyUnit::Kilocalorie),
        NutrientUnit::IU,
        NutrientUnit::DFE,
        NutrientUnit::NE,
        NutrientUnit::RAE,
        NutrientUnit::PDCAAS,
        NutrientUnit::DIAAS1,
        NutrientUnit::DIAAS2,
        NutrientUnit::DIAAS3,
    ]);

    assert_eq!(enumerations.sort(), manual_enumerations.sort());
}

#[test]
pub fn test_get_si_factor() {
    let mass_g = MassUnit::Gram;
    let volume_pt = VolumeUnit::Pint;
    let energy_kj = EnergyUnit::Kilojoule;

    assert_eq!(NutrientUnit::Mass(mass_g).si_factor(), Some(mass_g.si_factor()));
    assert_eq!(NutrientUnit::Volume(volume_pt).si_factor(), Some(volume_pt.si_factor()));
    assert_eq!(NutrientUnit::Energy(energy_kj).si_factor(), Some(energy_kj.si_factor()));
    assert!(NutrientUnit::IU.si_factor().is_none());
    assert!(NutrientUnit::DFE.si_factor().is_none());
    assert!(NutrientUnit::NE.si_factor().is_none());
    assert!(NutrientUnit::RAE.si_factor().is_none());
    assert!(NutrientUnit::PDCAAS.si_factor().is_none());
    assert!(NutrientUnit::DIAAS1.si_factor().is_none());
    assert!(NutrientUnit::DIAAS2.si_factor().is_none());
    assert!(NutrientUnit::DIAAS3.si_factor().is_none());
}
