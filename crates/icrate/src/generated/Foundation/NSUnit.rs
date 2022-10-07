use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitConverter;
    unsafe impl ClassType for NSUnitConverter {
        type Super = NSObject;
    }
);
impl NSUnitConverter {
    pub unsafe fn baseUnitValueFromValue(&self, value: c_double) -> c_double {
        msg_send![self, baseUnitValueFromValue: value]
    }
    pub unsafe fn valueFromBaseUnitValue(&self, baseUnitValue: c_double) -> c_double {
        msg_send![self, valueFromBaseUnitValue: baseUnitValue]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitConverterLinear;
    unsafe impl ClassType for NSUnitConverterLinear {
        type Super = NSUnitConverter;
    }
);
impl NSUnitConverterLinear {
    pub unsafe fn coefficient(&self) -> c_double {
        msg_send![self, coefficient]
    }
    pub unsafe fn constant(&self) -> c_double {
        msg_send![self, constant]
    }
    pub unsafe fn initWithCoefficient(&self, coefficient: c_double) -> Id<Self, Shared> {
        msg_send_id![self, initWithCoefficient: coefficient]
    }
    pub unsafe fn initWithCoefficient_constant(
        &self,
        coefficient: c_double,
        constant: c_double,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithCoefficient: coefficient, constant: constant]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnit;
    unsafe impl ClassType for NSUnit {
        type Super = NSObject;
    }
);
impl NSUnit {
    pub unsafe fn symbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, symbol]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
    pub unsafe fn initWithSymbol(&self, symbol: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithSymbol: symbol]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSDimension;
    unsafe impl ClassType for NSDimension {
        type Super = NSUnit;
    }
);
impl NSDimension {
    pub unsafe fn converter(&self) -> Id<NSUnitConverter, Shared> {
        msg_send_id![self, converter]
    }
    pub unsafe fn initWithSymbol_converter(
        &self,
        symbol: &NSString,
        converter: &NSUnitConverter,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithSymbol: symbol, converter: converter]
    }
    pub unsafe fn baseUnit() -> Id<Self, Shared> {
        msg_send_id![Self::class(), baseUnit]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitAcceleration;
    unsafe impl ClassType for NSUnitAcceleration {
        type Super = NSDimension;
    }
);
impl NSUnitAcceleration {
    pub unsafe fn metersPerSecondSquared() -> Id<NSUnitAcceleration, Shared> {
        msg_send_id![Self::class(), metersPerSecondSquared]
    }
    pub unsafe fn gravity() -> Id<NSUnitAcceleration, Shared> {
        msg_send_id![Self::class(), gravity]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitAngle;
    unsafe impl ClassType for NSUnitAngle {
        type Super = NSDimension;
    }
);
impl NSUnitAngle {
    pub unsafe fn degrees() -> Id<NSUnitAngle, Shared> {
        msg_send_id![Self::class(), degrees]
    }
    pub unsafe fn arcMinutes() -> Id<NSUnitAngle, Shared> {
        msg_send_id![Self::class(), arcMinutes]
    }
    pub unsafe fn arcSeconds() -> Id<NSUnitAngle, Shared> {
        msg_send_id![Self::class(), arcSeconds]
    }
    pub unsafe fn radians() -> Id<NSUnitAngle, Shared> {
        msg_send_id![Self::class(), radians]
    }
    pub unsafe fn gradians() -> Id<NSUnitAngle, Shared> {
        msg_send_id![Self::class(), gradians]
    }
    pub unsafe fn revolutions() -> Id<NSUnitAngle, Shared> {
        msg_send_id![Self::class(), revolutions]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitArea;
    unsafe impl ClassType for NSUnitArea {
        type Super = NSDimension;
    }
);
impl NSUnitArea {
    pub unsafe fn squareMegameters() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareMegameters]
    }
    pub unsafe fn squareKilometers() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareKilometers]
    }
    pub unsafe fn squareMeters() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareMeters]
    }
    pub unsafe fn squareCentimeters() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareCentimeters]
    }
    pub unsafe fn squareMillimeters() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareMillimeters]
    }
    pub unsafe fn squareMicrometers() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareMicrometers]
    }
    pub unsafe fn squareNanometers() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareNanometers]
    }
    pub unsafe fn squareInches() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareInches]
    }
    pub unsafe fn squareFeet() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareFeet]
    }
    pub unsafe fn squareYards() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareYards]
    }
    pub unsafe fn squareMiles() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), squareMiles]
    }
    pub unsafe fn acres() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), acres]
    }
    pub unsafe fn ares() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), ares]
    }
    pub unsafe fn hectares() -> Id<NSUnitArea, Shared> {
        msg_send_id![Self::class(), hectares]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitConcentrationMass;
    unsafe impl ClassType for NSUnitConcentrationMass {
        type Super = NSDimension;
    }
);
impl NSUnitConcentrationMass {
    pub unsafe fn gramsPerLiter() -> Id<NSUnitConcentrationMass, Shared> {
        msg_send_id![Self::class(), gramsPerLiter]
    }
    pub unsafe fn milligramsPerDeciliter() -> Id<NSUnitConcentrationMass, Shared> {
        msg_send_id![Self::class(), milligramsPerDeciliter]
    }
    pub unsafe fn millimolesPerLiterWithGramsPerMole(
        gramsPerMole: c_double,
    ) -> Id<NSUnitConcentrationMass, Shared> {
        msg_send_id![
            Self::class(),
            millimolesPerLiterWithGramsPerMole: gramsPerMole
        ]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitDispersion;
    unsafe impl ClassType for NSUnitDispersion {
        type Super = NSDimension;
    }
);
impl NSUnitDispersion {
    pub unsafe fn partsPerMillion() -> Id<NSUnitDispersion, Shared> {
        msg_send_id![Self::class(), partsPerMillion]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitDuration;
    unsafe impl ClassType for NSUnitDuration {
        type Super = NSDimension;
    }
);
impl NSUnitDuration {
    pub unsafe fn hours() -> Id<NSUnitDuration, Shared> {
        msg_send_id![Self::class(), hours]
    }
    pub unsafe fn minutes() -> Id<NSUnitDuration, Shared> {
        msg_send_id![Self::class(), minutes]
    }
    pub unsafe fn seconds() -> Id<NSUnitDuration, Shared> {
        msg_send_id![Self::class(), seconds]
    }
    pub unsafe fn milliseconds() -> Id<NSUnitDuration, Shared> {
        msg_send_id![Self::class(), milliseconds]
    }
    pub unsafe fn microseconds() -> Id<NSUnitDuration, Shared> {
        msg_send_id![Self::class(), microseconds]
    }
    pub unsafe fn nanoseconds() -> Id<NSUnitDuration, Shared> {
        msg_send_id![Self::class(), nanoseconds]
    }
    pub unsafe fn picoseconds() -> Id<NSUnitDuration, Shared> {
        msg_send_id![Self::class(), picoseconds]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitElectricCharge;
    unsafe impl ClassType for NSUnitElectricCharge {
        type Super = NSDimension;
    }
);
impl NSUnitElectricCharge {
    pub unsafe fn coulombs() -> Id<NSUnitElectricCharge, Shared> {
        msg_send_id![Self::class(), coulombs]
    }
    pub unsafe fn megaampereHours() -> Id<NSUnitElectricCharge, Shared> {
        msg_send_id![Self::class(), megaampereHours]
    }
    pub unsafe fn kiloampereHours() -> Id<NSUnitElectricCharge, Shared> {
        msg_send_id![Self::class(), kiloampereHours]
    }
    pub unsafe fn ampereHours() -> Id<NSUnitElectricCharge, Shared> {
        msg_send_id![Self::class(), ampereHours]
    }
    pub unsafe fn milliampereHours() -> Id<NSUnitElectricCharge, Shared> {
        msg_send_id![Self::class(), milliampereHours]
    }
    pub unsafe fn microampereHours() -> Id<NSUnitElectricCharge, Shared> {
        msg_send_id![Self::class(), microampereHours]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitElectricCurrent;
    unsafe impl ClassType for NSUnitElectricCurrent {
        type Super = NSDimension;
    }
);
impl NSUnitElectricCurrent {
    pub unsafe fn megaamperes() -> Id<NSUnitElectricCurrent, Shared> {
        msg_send_id![Self::class(), megaamperes]
    }
    pub unsafe fn kiloamperes() -> Id<NSUnitElectricCurrent, Shared> {
        msg_send_id![Self::class(), kiloamperes]
    }
    pub unsafe fn amperes() -> Id<NSUnitElectricCurrent, Shared> {
        msg_send_id![Self::class(), amperes]
    }
    pub unsafe fn milliamperes() -> Id<NSUnitElectricCurrent, Shared> {
        msg_send_id![Self::class(), milliamperes]
    }
    pub unsafe fn microamperes() -> Id<NSUnitElectricCurrent, Shared> {
        msg_send_id![Self::class(), microamperes]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitElectricPotentialDifference;
    unsafe impl ClassType for NSUnitElectricPotentialDifference {
        type Super = NSDimension;
    }
);
impl NSUnitElectricPotentialDifference {
    pub unsafe fn megavolts() -> Id<NSUnitElectricPotentialDifference, Shared> {
        msg_send_id![Self::class(), megavolts]
    }
    pub unsafe fn kilovolts() -> Id<NSUnitElectricPotentialDifference, Shared> {
        msg_send_id![Self::class(), kilovolts]
    }
    pub unsafe fn volts() -> Id<NSUnitElectricPotentialDifference, Shared> {
        msg_send_id![Self::class(), volts]
    }
    pub unsafe fn millivolts() -> Id<NSUnitElectricPotentialDifference, Shared> {
        msg_send_id![Self::class(), millivolts]
    }
    pub unsafe fn microvolts() -> Id<NSUnitElectricPotentialDifference, Shared> {
        msg_send_id![Self::class(), microvolts]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitElectricResistance;
    unsafe impl ClassType for NSUnitElectricResistance {
        type Super = NSDimension;
    }
);
impl NSUnitElectricResistance {
    pub unsafe fn megaohms() -> Id<NSUnitElectricResistance, Shared> {
        msg_send_id![Self::class(), megaohms]
    }
    pub unsafe fn kiloohms() -> Id<NSUnitElectricResistance, Shared> {
        msg_send_id![Self::class(), kiloohms]
    }
    pub unsafe fn ohms() -> Id<NSUnitElectricResistance, Shared> {
        msg_send_id![Self::class(), ohms]
    }
    pub unsafe fn milliohms() -> Id<NSUnitElectricResistance, Shared> {
        msg_send_id![Self::class(), milliohms]
    }
    pub unsafe fn microohms() -> Id<NSUnitElectricResistance, Shared> {
        msg_send_id![Self::class(), microohms]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitEnergy;
    unsafe impl ClassType for NSUnitEnergy {
        type Super = NSDimension;
    }
);
impl NSUnitEnergy {
    pub unsafe fn kilojoules() -> Id<NSUnitEnergy, Shared> {
        msg_send_id![Self::class(), kilojoules]
    }
    pub unsafe fn joules() -> Id<NSUnitEnergy, Shared> {
        msg_send_id![Self::class(), joules]
    }
    pub unsafe fn kilocalories() -> Id<NSUnitEnergy, Shared> {
        msg_send_id![Self::class(), kilocalories]
    }
    pub unsafe fn calories() -> Id<NSUnitEnergy, Shared> {
        msg_send_id![Self::class(), calories]
    }
    pub unsafe fn kilowattHours() -> Id<NSUnitEnergy, Shared> {
        msg_send_id![Self::class(), kilowattHours]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitFrequency;
    unsafe impl ClassType for NSUnitFrequency {
        type Super = NSDimension;
    }
);
impl NSUnitFrequency {
    pub unsafe fn terahertz() -> Id<NSUnitFrequency, Shared> {
        msg_send_id![Self::class(), terahertz]
    }
    pub unsafe fn gigahertz() -> Id<NSUnitFrequency, Shared> {
        msg_send_id![Self::class(), gigahertz]
    }
    pub unsafe fn megahertz() -> Id<NSUnitFrequency, Shared> {
        msg_send_id![Self::class(), megahertz]
    }
    pub unsafe fn kilohertz() -> Id<NSUnitFrequency, Shared> {
        msg_send_id![Self::class(), kilohertz]
    }
    pub unsafe fn hertz() -> Id<NSUnitFrequency, Shared> {
        msg_send_id![Self::class(), hertz]
    }
    pub unsafe fn millihertz() -> Id<NSUnitFrequency, Shared> {
        msg_send_id![Self::class(), millihertz]
    }
    pub unsafe fn microhertz() -> Id<NSUnitFrequency, Shared> {
        msg_send_id![Self::class(), microhertz]
    }
    pub unsafe fn nanohertz() -> Id<NSUnitFrequency, Shared> {
        msg_send_id![Self::class(), nanohertz]
    }
    pub unsafe fn framesPerSecond() -> Id<NSUnitFrequency, Shared> {
        msg_send_id![Self::class(), framesPerSecond]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitFuelEfficiency;
    unsafe impl ClassType for NSUnitFuelEfficiency {
        type Super = NSDimension;
    }
);
impl NSUnitFuelEfficiency {
    pub unsafe fn litersPer100Kilometers() -> Id<NSUnitFuelEfficiency, Shared> {
        msg_send_id![Self::class(), litersPer100Kilometers]
    }
    pub unsafe fn milesPerImperialGallon() -> Id<NSUnitFuelEfficiency, Shared> {
        msg_send_id![Self::class(), milesPerImperialGallon]
    }
    pub unsafe fn milesPerGallon() -> Id<NSUnitFuelEfficiency, Shared> {
        msg_send_id![Self::class(), milesPerGallon]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitInformationStorage;
    unsafe impl ClassType for NSUnitInformationStorage {
        type Super = NSDimension;
    }
);
impl NSUnitInformationStorage {
    pub unsafe fn bytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), bytes]
    }
    pub unsafe fn bits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), bits]
    }
    pub unsafe fn nibbles() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), nibbles]
    }
    pub unsafe fn yottabytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), yottabytes]
    }
    pub unsafe fn zettabytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), zettabytes]
    }
    pub unsafe fn exabytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), exabytes]
    }
    pub unsafe fn petabytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), petabytes]
    }
    pub unsafe fn terabytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), terabytes]
    }
    pub unsafe fn gigabytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), gigabytes]
    }
    pub unsafe fn megabytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), megabytes]
    }
    pub unsafe fn kilobytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), kilobytes]
    }
    pub unsafe fn yottabits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), yottabits]
    }
    pub unsafe fn zettabits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), zettabits]
    }
    pub unsafe fn exabits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), exabits]
    }
    pub unsafe fn petabits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), petabits]
    }
    pub unsafe fn terabits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), terabits]
    }
    pub unsafe fn gigabits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), gigabits]
    }
    pub unsafe fn megabits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), megabits]
    }
    pub unsafe fn kilobits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), kilobits]
    }
    pub unsafe fn yobibytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), yobibytes]
    }
    pub unsafe fn zebibytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), zebibytes]
    }
    pub unsafe fn exbibytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), exbibytes]
    }
    pub unsafe fn pebibytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), pebibytes]
    }
    pub unsafe fn tebibytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), tebibytes]
    }
    pub unsafe fn gibibytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), gibibytes]
    }
    pub unsafe fn mebibytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), mebibytes]
    }
    pub unsafe fn kibibytes() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), kibibytes]
    }
    pub unsafe fn yobibits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), yobibits]
    }
    pub unsafe fn zebibits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), zebibits]
    }
    pub unsafe fn exbibits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), exbibits]
    }
    pub unsafe fn pebibits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), pebibits]
    }
    pub unsafe fn tebibits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), tebibits]
    }
    pub unsafe fn gibibits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), gibibits]
    }
    pub unsafe fn mebibits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), mebibits]
    }
    pub unsafe fn kibibits() -> Id<NSUnitInformationStorage, Shared> {
        msg_send_id![Self::class(), kibibits]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitLength;
    unsafe impl ClassType for NSUnitLength {
        type Super = NSDimension;
    }
);
impl NSUnitLength {
    pub unsafe fn megameters() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), megameters]
    }
    pub unsafe fn kilometers() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), kilometers]
    }
    pub unsafe fn hectometers() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), hectometers]
    }
    pub unsafe fn decameters() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), decameters]
    }
    pub unsafe fn meters() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), meters]
    }
    pub unsafe fn decimeters() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), decimeters]
    }
    pub unsafe fn centimeters() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), centimeters]
    }
    pub unsafe fn millimeters() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), millimeters]
    }
    pub unsafe fn micrometers() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), micrometers]
    }
    pub unsafe fn nanometers() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), nanometers]
    }
    pub unsafe fn picometers() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), picometers]
    }
    pub unsafe fn inches() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), inches]
    }
    pub unsafe fn feet() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), feet]
    }
    pub unsafe fn yards() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), yards]
    }
    pub unsafe fn miles() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), miles]
    }
    pub unsafe fn scandinavianMiles() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), scandinavianMiles]
    }
    pub unsafe fn lightyears() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), lightyears]
    }
    pub unsafe fn nauticalMiles() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), nauticalMiles]
    }
    pub unsafe fn fathoms() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), fathoms]
    }
    pub unsafe fn furlongs() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), furlongs]
    }
    pub unsafe fn astronomicalUnits() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), astronomicalUnits]
    }
    pub unsafe fn parsecs() -> Id<NSUnitLength, Shared> {
        msg_send_id![Self::class(), parsecs]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitIlluminance;
    unsafe impl ClassType for NSUnitIlluminance {
        type Super = NSDimension;
    }
);
impl NSUnitIlluminance {
    pub unsafe fn lux() -> Id<NSUnitIlluminance, Shared> {
        msg_send_id![Self::class(), lux]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitMass;
    unsafe impl ClassType for NSUnitMass {
        type Super = NSDimension;
    }
);
impl NSUnitMass {
    pub unsafe fn kilograms() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), kilograms]
    }
    pub unsafe fn grams() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), grams]
    }
    pub unsafe fn decigrams() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), decigrams]
    }
    pub unsafe fn centigrams() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), centigrams]
    }
    pub unsafe fn milligrams() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), milligrams]
    }
    pub unsafe fn micrograms() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), micrograms]
    }
    pub unsafe fn nanograms() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), nanograms]
    }
    pub unsafe fn picograms() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), picograms]
    }
    pub unsafe fn ounces() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), ounces]
    }
    pub unsafe fn poundsMass() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), poundsMass]
    }
    pub unsafe fn stones() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), stones]
    }
    pub unsafe fn metricTons() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), metricTons]
    }
    pub unsafe fn shortTons() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), shortTons]
    }
    pub unsafe fn carats() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), carats]
    }
    pub unsafe fn ouncesTroy() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), ouncesTroy]
    }
    pub unsafe fn slugs() -> Id<NSUnitMass, Shared> {
        msg_send_id![Self::class(), slugs]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitPower;
    unsafe impl ClassType for NSUnitPower {
        type Super = NSDimension;
    }
);
impl NSUnitPower {
    pub unsafe fn terawatts() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), terawatts]
    }
    pub unsafe fn gigawatts() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), gigawatts]
    }
    pub unsafe fn megawatts() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), megawatts]
    }
    pub unsafe fn kilowatts() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), kilowatts]
    }
    pub unsafe fn watts() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), watts]
    }
    pub unsafe fn milliwatts() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), milliwatts]
    }
    pub unsafe fn microwatts() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), microwatts]
    }
    pub unsafe fn nanowatts() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), nanowatts]
    }
    pub unsafe fn picowatts() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), picowatts]
    }
    pub unsafe fn femtowatts() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), femtowatts]
    }
    pub unsafe fn horsepower() -> Id<NSUnitPower, Shared> {
        msg_send_id![Self::class(), horsepower]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitPressure;
    unsafe impl ClassType for NSUnitPressure {
        type Super = NSDimension;
    }
);
impl NSUnitPressure {
    pub unsafe fn newtonsPerMetersSquared() -> Id<NSUnitPressure, Shared> {
        msg_send_id![Self::class(), newtonsPerMetersSquared]
    }
    pub unsafe fn gigapascals() -> Id<NSUnitPressure, Shared> {
        msg_send_id![Self::class(), gigapascals]
    }
    pub unsafe fn megapascals() -> Id<NSUnitPressure, Shared> {
        msg_send_id![Self::class(), megapascals]
    }
    pub unsafe fn kilopascals() -> Id<NSUnitPressure, Shared> {
        msg_send_id![Self::class(), kilopascals]
    }
    pub unsafe fn hectopascals() -> Id<NSUnitPressure, Shared> {
        msg_send_id![Self::class(), hectopascals]
    }
    pub unsafe fn inchesOfMercury() -> Id<NSUnitPressure, Shared> {
        msg_send_id![Self::class(), inchesOfMercury]
    }
    pub unsafe fn bars() -> Id<NSUnitPressure, Shared> {
        msg_send_id![Self::class(), bars]
    }
    pub unsafe fn millibars() -> Id<NSUnitPressure, Shared> {
        msg_send_id![Self::class(), millibars]
    }
    pub unsafe fn millimetersOfMercury() -> Id<NSUnitPressure, Shared> {
        msg_send_id![Self::class(), millimetersOfMercury]
    }
    pub unsafe fn poundsForcePerSquareInch() -> Id<NSUnitPressure, Shared> {
        msg_send_id![Self::class(), poundsForcePerSquareInch]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitSpeed;
    unsafe impl ClassType for NSUnitSpeed {
        type Super = NSDimension;
    }
);
impl NSUnitSpeed {
    pub unsafe fn metersPerSecond() -> Id<NSUnitSpeed, Shared> {
        msg_send_id![Self::class(), metersPerSecond]
    }
    pub unsafe fn kilometersPerHour() -> Id<NSUnitSpeed, Shared> {
        msg_send_id![Self::class(), kilometersPerHour]
    }
    pub unsafe fn milesPerHour() -> Id<NSUnitSpeed, Shared> {
        msg_send_id![Self::class(), milesPerHour]
    }
    pub unsafe fn knots() -> Id<NSUnitSpeed, Shared> {
        msg_send_id![Self::class(), knots]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitTemperature;
    unsafe impl ClassType for NSUnitTemperature {
        type Super = NSDimension;
    }
);
impl NSUnitTemperature {
    pub unsafe fn kelvin() -> Id<NSUnitTemperature, Shared> {
        msg_send_id![Self::class(), kelvin]
    }
    pub unsafe fn celsius() -> Id<NSUnitTemperature, Shared> {
        msg_send_id![Self::class(), celsius]
    }
    pub unsafe fn fahrenheit() -> Id<NSUnitTemperature, Shared> {
        msg_send_id![Self::class(), fahrenheit]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitVolume;
    unsafe impl ClassType for NSUnitVolume {
        type Super = NSDimension;
    }
);
impl NSUnitVolume {
    pub unsafe fn megaliters() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), megaliters]
    }
    pub unsafe fn kiloliters() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), kiloliters]
    }
    pub unsafe fn liters() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), liters]
    }
    pub unsafe fn deciliters() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), deciliters]
    }
    pub unsafe fn centiliters() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), centiliters]
    }
    pub unsafe fn milliliters() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), milliliters]
    }
    pub unsafe fn cubicKilometers() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), cubicKilometers]
    }
    pub unsafe fn cubicMeters() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), cubicMeters]
    }
    pub unsafe fn cubicDecimeters() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), cubicDecimeters]
    }
    pub unsafe fn cubicCentimeters() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), cubicCentimeters]
    }
    pub unsafe fn cubicMillimeters() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), cubicMillimeters]
    }
    pub unsafe fn cubicInches() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), cubicInches]
    }
    pub unsafe fn cubicFeet() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), cubicFeet]
    }
    pub unsafe fn cubicYards() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), cubicYards]
    }
    pub unsafe fn cubicMiles() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), cubicMiles]
    }
    pub unsafe fn acreFeet() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), acreFeet]
    }
    pub unsafe fn bushels() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), bushels]
    }
    pub unsafe fn teaspoons() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), teaspoons]
    }
    pub unsafe fn tablespoons() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), tablespoons]
    }
    pub unsafe fn fluidOunces() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), fluidOunces]
    }
    pub unsafe fn cups() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), cups]
    }
    pub unsafe fn pints() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), pints]
    }
    pub unsafe fn quarts() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), quarts]
    }
    pub unsafe fn gallons() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), gallons]
    }
    pub unsafe fn imperialTeaspoons() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), imperialTeaspoons]
    }
    pub unsafe fn imperialTablespoons() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), imperialTablespoons]
    }
    pub unsafe fn imperialFluidOunces() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), imperialFluidOunces]
    }
    pub unsafe fn imperialPints() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), imperialPints]
    }
    pub unsafe fn imperialQuarts() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), imperialQuarts]
    }
    pub unsafe fn imperialGallons() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), imperialGallons]
    }
    pub unsafe fn metricCups() -> Id<NSUnitVolume, Shared> {
        msg_send_id![Self::class(), metricCups]
    }
}
