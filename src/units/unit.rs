use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnitAtom {
    // Mass
    Tonne, // t
    Kilogram, // kg
    Gram, // g
    Milligram, // mg

    // Length
    Kilometer, // km
    Meter, // m
    Centimeter, // cm
    Millimeter, // mm
    Micrometer, // μm
    Nanometer, // nm

    // Time
    Hour, // h
    Minute, // min
    Second, // s
    Millisecond, // ms
    Microsecond, // μs
    Nanosecond, // ns

    // Temperature
    Kelvin, // k
    Celsius, // °C

    // Electricity
    Ampere, // A
    Volt, // V
    Ohm, // Ω
    Watt, // W

    // Mechanics
    Newton, // N
    Joule, // J
    Pascal, // Pa

    // Geometry
    SquareMeter, // m²
    SquareKilometer, // km²,
    CubicMeter, // m³
    Liter, // L
    Milliliter, // mL

    // Chemistry
    Mole, // ml
    Percent, // %

    // Optics
    Lumen, // lm
    Lux, // lx
    Becquerel, // Bq
}

impl fmt::Display for UnitAtom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            UnitAtom::Tonne => "t",
            UnitAtom::Kilogram => "kg",
            UnitAtom::Gram => "g",
            UnitAtom::Milligram => "mg",

            UnitAtom::Kilometer => "km",
            UnitAtom::Meter => "m",
            UnitAtom::Centimeter => "cm",
            UnitAtom::Millimeter => "mm",
            UnitAtom::Micrometer => "μm",
            UnitAtom::Nanometer => "nm",

            UnitAtom::Hour => "h",
            UnitAtom::Minute => "min",
            UnitAtom::Second => "s",
            UnitAtom::Millisecond => "ms",
            UnitAtom::Microsecond => "μs",
            UnitAtom::Nanosecond => "ns",

            UnitAtom::Kelvin => "k",
            UnitAtom::Celsius => "°C",

            UnitAtom::Ampere => "A",
            UnitAtom::Volt => "V",
            UnitAtom::Ohm => "Ω",
            UnitAtom::Watt => "W",

            UnitAtom::Newton => "N",
            UnitAtom::Joule => "J",
            UnitAtom::Pascal => "Pa",

            UnitAtom::SquareMeter => "m²",
            UnitAtom::SquareKilometer => "km²",
            UnitAtom::CubicMeter => "m³",
            UnitAtom::Liter => "L",
            UnitAtom::Milliliter => "mL",

            UnitAtom::Mole => "mol",
            UnitAtom::Percent => "%",

            UnitAtom::Lumen => "lm",
            UnitAtom::Lux => "lx",
            UnitAtom::Becquerel => "Bq",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub numerators: Vec<UnitAtom>,
    pub denominators: Vec<UnitAtom>,
}

impl Unit {
    pub fn div(&self, rhs: &Unit) -> Unit {
        let mut numerators = self.numerators.clone();
        let mut denominators = self.denominators.clone();

        numerators.extend(rhs.denominators.clone());
        denominators.extend(rhs.numerators.clone());

        Unit { numerators, denominators }
    }
}