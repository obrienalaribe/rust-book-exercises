// First, we are going to introduce some units of energy. For whatever reason, we prefer BTU above
// Joules and Calories, but we want to support all 3 of these in this module. Double check the
// conversion methods, and make sure you fully understand them.

use std::cell::RefCell;
use std::marker::PhantomData;

// You may uncomment and use the following import if you need it. You may also read its
// documentation at https://doc.rust-lang.org/std/cell/struct.RefCell
// use std::cell::RefCell;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Joule(u32);
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Calorie(u32);

pub type BTU = u32;

impl From<Joule> for BTU {
    fn from(j: Joule) -> Self {
        j.0 / 1055
    }
}

impl From<BTU> for Joule {
    fn from(b: BTU) -> Self {
        Self(b * 1055)
    }
}

impl From<Calorie> for BTU {
    fn from(c: Calorie) -> Self {
        c.0 / 251
    }
}

impl From<BTU> for Calorie {
    fn from(b: BTU) -> Self {
        Calorie(b * 251)
    }
}

// Now, we start defining some types of fuel.

/// A technology for storing energy for later consumption.
pub trait Fuel {
    /// The output unit of the energy density.
    ///
    /// Think about this: why did we chose this to be an associated type rather than a generic?
    type Output: Into<BTU> + From<BTU>;

    /// The amount of energy contained in a single unit of fuel.
    fn energy_density() -> Self::Output;
}

pub struct Diesel;
impl Fuel for Diesel {
    type Output = Joule;
    fn energy_density() -> Self::Output {
        Joule::from(100)
    }
}

pub struct LithiumBattery;
impl Fuel for LithiumBattery {
    type Output = Calorie;
    fn energy_density() -> Self::Output {
        Calorie::from(200)
    }
}

pub struct Uranium;
impl Fuel for Uranium {
    type Output = Joule;
    fn energy_density() -> Self::Output {
        Joule::from(1000)
    }
}

/// A container for any fuel type.
pub struct FuelContainer<F: Fuel> {
    /// the amount of fuel.
    amount: u32,
    /// Note: Why do we need this? Fuel doesn't really have any methods that require `&self` on it,
    /// so any information that we can get, we can get from `F` as **TYPE**, we don't really need
    /// to store an instance of `F`, like `fuel: F` as a struct field. But to satisfy the compiler,
    /// we muse use `F` somewhere. This is the perfect use case of `PhantomData`.
    _marker: PhantomData<F>,
}

impl<F: Fuel> FuelContainer<F> {
    pub fn new(amount: u32) -> Self {
        Self { amount, _marker: Default::default() }
    }
}

/// Something that can provide energy from a given `F` fuel type, like a power-plant.
pub trait ProvideEnergy<F: Fuel> {
    /// consume the fuel container and return the created energy, based on the power density of the
    /// fuel and potentially other factors.
    ///
    /// Some fuel providers might have some kind of decay or inefficiency, which should be reflected
    /// here. Otherwise, [`provide_energy_with_efficiency`] or [`provide_energy_ideal`] might be
    /// good enough.
    ///
    /// Not all `ProvideEnergy` implementations need to have internal state. Therefore, this
    /// interface accepts `&self`, not `&mut self`. You might need to use special language features
    /// to overcome this.
    fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output;

    /// Convert the amount of fuel in `f` with an exact efficiency of `e`.
    ///
    /// NOTE: all efficiencies are interpreted as u8 values that can be at most 100, and represent a
    /// percent.
    ///
    /// This method must be provided as it will be the same in all implementations.
    fn provide_energy_with_efficiency(&self, f: FuelContainer<F>, e: u8) -> <F as Fuel>::Output {
        todo!();
    }

    /// Same as above, but with an efficiency of 100.
    ///
    /// This method must be provided as it will be the same in all implementations.
    fn provide_energy_ideal(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
        let joule = Uranium::energy_density();
        let energy = (f.amount * joule.0) * (e/100) as u32 ;
        energy
    }
}

/// A nuclear reactor that can only consume `Uranium` and provide energy with 99% efficiency.
pub struct NuclearReactor;
impl ProvideEnergy<Uranium> for NuclearReactor {
    fn provide_energy(&self, f: FuelContainer<Uranium>) -> <Uranium as Fuel>::Output {
        self.provide_energy_with_efficiency(f, 99)
    }

    fn provide_energy_with_efficiency(
        &self,
        f: FuelContainer<Uranium>,
        e: u8,
    ) -> <Uranium as Fuel>::Output {
        Joule::from(BTU::from(Uranium::energy_density()) * f.amount * 99 / 100)
    }

    fn provide_energy_ideal(&self, f: FuelContainer<Uranium>) -> <Uranium as Fuel>::Output {
        Joule::from(BTU::from(Uranium::energy_density()) * f.amount)
    }
}
/// A combustion engine that can only consume `Diesel`.
///
/// The `DECAY` const must be interpreted as such: per every `DECAY` times `provide_energy` is
/// called on an instance of this type, the efficiency should reduce by one. The initial efficiency
/// must be configurable with a `fn new(efficiency: u8) -> Self`.
pub struct InternalCombustion<const DECAY: u32> {
    times_called: RefCell<u32>,
    efficiency: RefCell<u8>,
}

impl<const DECAY: u32> InternalCombustion<DECAY> {
    pub fn new(efficiency: u8) -> Self {
        if efficiency > 100 {
            Self { efficiency: RefCell::new(100), times_called: RefCell::new(0) }
        } else {
            Self { efficiency: RefCell::new(efficiency), times_called: RefCell::new(0) }
        }
    }
}


impl<const DECAY: u32> ProvideEnergy<Diesel> for InternalCombustion<DECAY> {
    fn provide_energy(&self, f: FuelContainer<Diesel>) -> <Diesel as Fuel>::Output {
        *self.times_called.borrow_mut() += 1;
        if self.times_called >= (DECAY + 1).into() {
            *self.efficiency.borrow_mut() -= 1;
            self.provide_energy_with_efficiency(f, *self.efficiency.borrow())
        } else {
            self.provide_energy_ideal(f)
        }
    }
    fn provide_energy_with_efficiency(
        &self,
        f: FuelContainer<Diesel>,
        e: u8,
    ) -> <Diesel as Fuel>::Output {
        Joule::from(BTU::from(Diesel::energy_density()) * f.amount * (e as u32) / 100)
    }

    fn provide_energy_ideal(&self, f: FuelContainer<Diesel>) -> <Diesel as Fuel>::Output {
        Joule::from(BTU::from(Diesel::energy_density()) * f.amount)
    }
}

/// A hypothetical device that can, unlike the `InternalCombustion`, consume **any fuel** that's of
/// type `trait Fuel`. It can provide a fixed efficiency regardless of fuel type. As before,
/// EFFICIENCY is a u8 whose value should not exceed 100 and is interpreted as a percent.
pub struct OmniGenerator<const EFFICIENCY: u8>;

// NOTE: implement `ProvideEnergy` for `OmniGenerator` using only one `impl` block.
// TODO: uncomment and complete:
impl ProvideEnergy<F> for OmniGenerator<{ EFFICIENCY }> {
    fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
        let joule = Fuel::energy_density();
        let energy = (f.amount * joule.0) as f64 * 0.99;
        Joule(energy as u32)
    }

    fn provide_energy_with_efficiency(&self, f: FuelContainer<F>, e: u8) -> <F as Fuel>::Output {
        let joule = Fuel::energy_density();
        let energy = (f.amount * joule.0) * (e/100) as u32 ;
        Joule(energy)
    }

    fn provide_energy_ideal(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
        let joule = Fuel::energy_density();
        let energy = (f.amount * joule.0) as f64;
        Joule(energy as u32)
    }
}

/// A type that can wrap two different fuel types and mix them together.
///
/// The energy density of the new fuel type is the average of the two given, once converted to BTU.
/// The output unit should also be BTU.
///
/// This can represent a new fuel type, thus it must implement `Fuel`.
pub struct Mixed<F1: Fuel, F2: Fuel>(PhantomData<(F1, F2)>);

// TODO: uncomment and complete:
// impl Fuel for Mixed<> {}

// Now think about how you can make the mixer configurable, such that it would produce a new fuel
// with an energy density that is more influences by one type than the other.
//
// For example, you have a mixer of F1, F2, and some coefficient C1, where the energy density of the
// mixture is `F1 * C1 + F2 * (1 - C1) )` where `C1` is a ratio (which you have to represent again
// with a u8 percent).
//
// The main trick is to overcome the fact that `fn energy_density` does not take in a `self`, so the
// coefficients need to be incorporated in some other way (you've already seen examples of that in
// this file ;)).
pub struct CustomMixed<const C: u8, F1, F2>(PhantomData<(F1, F2)>);
// TODO: uncomment and complete:
// impl Fuel for CustomMixed<> {}

// Now, any of our existing energy providers can be used with a mix fuel.
/// A function that returns the energy produced by the `OmniGenerator` with efficiency of 80%, when
/// the fuel type is a mix of `Diesel` as `LithiumBattery`;
pub fn omni_80_energy(amount: u32) -> BTU {
    todo!();
}

// Finally, let's consider marker traits, and some trait bounds.

/// Some traits are just markers. They don't bring any additional functionality anything, other than
/// marking a type with some trait.
pub trait IsRenewable {}
impl IsRenewable for LithiumBattery {}

/// Define the following struct such that it only provides energy if the fuel is `IsRenewable`.
///
/// It has perfect efficiency.
pub struct GreenEngine<F: Fuel>(PhantomData<F>);
// TODO: uncomment and complete:
// impl ProvideEnergy for GreenEngine {}

/// Define the following struct such that it only provides energy if the fuel's output type is
/// `BTU`.
///
/// It has perfect efficiency.
pub struct BritishEngine<F: Fuel>(PhantomData<F>);
// TODO: uncomment and complete:
// impl ProvideEnergy for BritishEngine {}

// Congratulations! you have finished the advance trait section.
//
// Disclaimer: the types and traits that you are asked to implement in this module are by no means
// designed to be sensible. Instead, they are chosen to represent a typical, often difficult,
// pattern. Some are intentionally slightly convoluted to challenge you :). I am sure if we actually
// wanted to design a fuel system, we would do better.
