pub struct Nutrient {
    id: UUID,
    name: String,
    unit: Unit,
    categories: HashSet<NutrientTypes>
}

pub enum Unit {
    Mass(Mass),
    Volume(Volume),
}

pub struct Mass {
    value: f64,
    unit: MassUnit,
}

pub enum MassUnit {
    Gram,
    Milligram,
    Kilogram,
    Microgram,
    Ounce,
}

pub struct Volume {
    value: f64,
    unit: VolumeUnit,
}

pub enum VolumeUnit {
    Liter,
    Milliliter,
}