use uuid::Uuid;

pub struct Nutrient {
    id: Uuid,
    name: String,
    categories: HashSet<NutrientTypes>,
    parent: Vec<Uuid>,
    main_unit: Unit,
    accepted_units: Unit,
    unit_conversions: Vec<UnitConversion>
}

pub enum Unit { // Make this just the unit rather than the 
    Mass(MassUnit),
    Volume(VolumeUnit),
    Energy(EnergyUnit),
    IU, // International Unit
    DFE, // Dietary Folate Equivalent
    NE, // Niacin Equivalent
    RAE, // Retinol Activity Equivalent
    PDCAAS, // Protein Digestibility Corrected Amino Acid Score
    DIAAS1, // Digestible Indispensable Amino Acid Score 0 to 6 months
    DIAAS2, // Digestible Indispensable Amino Acid Score 6 months to 3 years
    DIAAS3, // Digestible Indispensable Amino Acid Score Over 3 years
}

pub enum MassUnit {
    Gram,
    Milligram,
    Kilogram,
    Microgram,
    Ounce,
}

pub enum VolumeUnit {
    Liter,
    Milliliter,
}

pub enum EnergyUnit {
    Kcal,
    KJ,
}

pub struct UnitConversion {
    from: Unit,
    to: Unit,
    factor: f64,
}