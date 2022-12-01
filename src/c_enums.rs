use crate::c_enums::Distance::{Feet, Meters, Miles};

/// A type representing distances in multiple units
#[derive(Clone, Copy)]
pub enum Distance {
	Meters(f32),
	Feet(f32),
	Miles(f32),
}

/// There are roughly 1609 meters in one mile
const METERS_PER_MILE: f32 = 1609.344;

/// There are roughly 3.3 feet in one meter
const FEET_PER_METER: f32 = 3.281;

// There are roughly 5280 feet in one mile
const FEET_PER_MILE: f32 = 5280.000;

impl Distance {
	/// Convert the given distance to meters
	pub fn to_meters(&self) -> Self {
		match self {
			Meters(n) => { Meters(*n) }
			Feet(n) => { Meters(*n / FEET_PER_METER) }
			Miles(n) => { Meters(*n * METERS_PER_MILE) }
		}
	}

	/// Convert the given distance to feet
	pub fn to_feet(&self) -> Self {
		match self {
			Feet(n) => { Feet(*n) }
			Meters(n) => { Feet(*n * FEET_PER_METER) }
			Miles(n) => { Feet(*n * FEET_PER_MILE) }
		}
	}

	/// Convert the given distance to miles
	pub fn to_miles(&self) -> Self {
		match self {
			Miles(n) => { Miles(*n) }
			Feet(n) => { Miles(*n / FEET_PER_MILE) }
			Meters(n) => { Miles(*n / METERS_PER_MILE) }
		}
	}
}