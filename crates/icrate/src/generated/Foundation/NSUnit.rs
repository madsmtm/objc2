#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitConverter;
    unsafe impl ClassType for NSUnitConverter {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUnitConverter {
        #[method(baseUnitValueFromValue:)]
        pub unsafe fn baseUnitValueFromValue(&self, value: c_double) -> c_double;
        #[method(valueFromBaseUnitValue:)]
        pub unsafe fn valueFromBaseUnitValue(&self, baseUnitValue: c_double) -> c_double;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitConverterLinear;
    unsafe impl ClassType for NSUnitConverterLinear {
        type Super = NSUnitConverter;
    }
);
extern_methods!(
    unsafe impl NSUnitConverterLinear {
        #[method(coefficient)]
        pub unsafe fn coefficient(&self) -> c_double;
        #[method(constant)]
        pub unsafe fn constant(&self) -> c_double;
        #[method_id(initWithCoefficient:)]
        pub unsafe fn initWithCoefficient(&self, coefficient: c_double) -> Id<Self, Shared>;
        #[method_id(initWithCoefficient:constant:)]
        pub unsafe fn initWithCoefficient_constant(
            &self,
            coefficient: c_double,
            constant: c_double,
        ) -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnit;
    unsafe impl ClassType for NSUnit {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUnit {
        #[method_id(symbol)]
        pub unsafe fn symbol(&self) -> Id<NSString, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method_id(initWithSymbol:)]
        pub unsafe fn initWithSymbol(&self, symbol: &NSString) -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSDimension;
    unsafe impl ClassType for NSDimension {
        type Super = NSUnit;
    }
);
extern_methods!(
    unsafe impl NSDimension {
        #[method_id(converter)]
        pub unsafe fn converter(&self) -> Id<NSUnitConverter, Shared>;
        #[method_id(initWithSymbol:converter:)]
        pub unsafe fn initWithSymbol_converter(
            &self,
            symbol: &NSString,
            converter: &NSUnitConverter,
        ) -> Id<Self, Shared>;
        #[method_id(baseUnit)]
        pub unsafe fn baseUnit() -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitAcceleration;
    unsafe impl ClassType for NSUnitAcceleration {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitAcceleration {
        #[method_id(metersPerSecondSquared)]
        pub unsafe fn metersPerSecondSquared() -> Id<NSUnitAcceleration, Shared>;
        #[method_id(gravity)]
        pub unsafe fn gravity() -> Id<NSUnitAcceleration, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitAngle;
    unsafe impl ClassType for NSUnitAngle {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitAngle {
        #[method_id(degrees)]
        pub unsafe fn degrees() -> Id<NSUnitAngle, Shared>;
        #[method_id(arcMinutes)]
        pub unsafe fn arcMinutes() -> Id<NSUnitAngle, Shared>;
        #[method_id(arcSeconds)]
        pub unsafe fn arcSeconds() -> Id<NSUnitAngle, Shared>;
        #[method_id(radians)]
        pub unsafe fn radians() -> Id<NSUnitAngle, Shared>;
        #[method_id(gradians)]
        pub unsafe fn gradians() -> Id<NSUnitAngle, Shared>;
        #[method_id(revolutions)]
        pub unsafe fn revolutions() -> Id<NSUnitAngle, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitArea;
    unsafe impl ClassType for NSUnitArea {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitArea {
        #[method_id(squareMegameters)]
        pub unsafe fn squareMegameters() -> Id<NSUnitArea, Shared>;
        #[method_id(squareKilometers)]
        pub unsafe fn squareKilometers() -> Id<NSUnitArea, Shared>;
        #[method_id(squareMeters)]
        pub unsafe fn squareMeters() -> Id<NSUnitArea, Shared>;
        #[method_id(squareCentimeters)]
        pub unsafe fn squareCentimeters() -> Id<NSUnitArea, Shared>;
        #[method_id(squareMillimeters)]
        pub unsafe fn squareMillimeters() -> Id<NSUnitArea, Shared>;
        #[method_id(squareMicrometers)]
        pub unsafe fn squareMicrometers() -> Id<NSUnitArea, Shared>;
        #[method_id(squareNanometers)]
        pub unsafe fn squareNanometers() -> Id<NSUnitArea, Shared>;
        #[method_id(squareInches)]
        pub unsafe fn squareInches() -> Id<NSUnitArea, Shared>;
        #[method_id(squareFeet)]
        pub unsafe fn squareFeet() -> Id<NSUnitArea, Shared>;
        #[method_id(squareYards)]
        pub unsafe fn squareYards() -> Id<NSUnitArea, Shared>;
        #[method_id(squareMiles)]
        pub unsafe fn squareMiles() -> Id<NSUnitArea, Shared>;
        #[method_id(acres)]
        pub unsafe fn acres() -> Id<NSUnitArea, Shared>;
        #[method_id(ares)]
        pub unsafe fn ares() -> Id<NSUnitArea, Shared>;
        #[method_id(hectares)]
        pub unsafe fn hectares() -> Id<NSUnitArea, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitConcentrationMass;
    unsafe impl ClassType for NSUnitConcentrationMass {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitConcentrationMass {
        #[method_id(gramsPerLiter)]
        pub unsafe fn gramsPerLiter() -> Id<NSUnitConcentrationMass, Shared>;
        #[method_id(milligramsPerDeciliter)]
        pub unsafe fn milligramsPerDeciliter() -> Id<NSUnitConcentrationMass, Shared>;
        #[method_id(millimolesPerLiterWithGramsPerMole:)]
        pub unsafe fn millimolesPerLiterWithGramsPerMole(
            gramsPerMole: c_double,
        ) -> Id<NSUnitConcentrationMass, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitDispersion;
    unsafe impl ClassType for NSUnitDispersion {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitDispersion {
        #[method_id(partsPerMillion)]
        pub unsafe fn partsPerMillion() -> Id<NSUnitDispersion, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitDuration;
    unsafe impl ClassType for NSUnitDuration {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitDuration {
        #[method_id(hours)]
        pub unsafe fn hours() -> Id<NSUnitDuration, Shared>;
        #[method_id(minutes)]
        pub unsafe fn minutes() -> Id<NSUnitDuration, Shared>;
        #[method_id(seconds)]
        pub unsafe fn seconds() -> Id<NSUnitDuration, Shared>;
        #[method_id(milliseconds)]
        pub unsafe fn milliseconds() -> Id<NSUnitDuration, Shared>;
        #[method_id(microseconds)]
        pub unsafe fn microseconds() -> Id<NSUnitDuration, Shared>;
        #[method_id(nanoseconds)]
        pub unsafe fn nanoseconds() -> Id<NSUnitDuration, Shared>;
        #[method_id(picoseconds)]
        pub unsafe fn picoseconds() -> Id<NSUnitDuration, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitElectricCharge;
    unsafe impl ClassType for NSUnitElectricCharge {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitElectricCharge {
        #[method_id(coulombs)]
        pub unsafe fn coulombs() -> Id<NSUnitElectricCharge, Shared>;
        #[method_id(megaampereHours)]
        pub unsafe fn megaampereHours() -> Id<NSUnitElectricCharge, Shared>;
        #[method_id(kiloampereHours)]
        pub unsafe fn kiloampereHours() -> Id<NSUnitElectricCharge, Shared>;
        #[method_id(ampereHours)]
        pub unsafe fn ampereHours() -> Id<NSUnitElectricCharge, Shared>;
        #[method_id(milliampereHours)]
        pub unsafe fn milliampereHours() -> Id<NSUnitElectricCharge, Shared>;
        #[method_id(microampereHours)]
        pub unsafe fn microampereHours() -> Id<NSUnitElectricCharge, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitElectricCurrent;
    unsafe impl ClassType for NSUnitElectricCurrent {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitElectricCurrent {
        #[method_id(megaamperes)]
        pub unsafe fn megaamperes() -> Id<NSUnitElectricCurrent, Shared>;
        #[method_id(kiloamperes)]
        pub unsafe fn kiloamperes() -> Id<NSUnitElectricCurrent, Shared>;
        #[method_id(amperes)]
        pub unsafe fn amperes() -> Id<NSUnitElectricCurrent, Shared>;
        #[method_id(milliamperes)]
        pub unsafe fn milliamperes() -> Id<NSUnitElectricCurrent, Shared>;
        #[method_id(microamperes)]
        pub unsafe fn microamperes() -> Id<NSUnitElectricCurrent, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitElectricPotentialDifference;
    unsafe impl ClassType for NSUnitElectricPotentialDifference {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitElectricPotentialDifference {
        #[method_id(megavolts)]
        pub unsafe fn megavolts() -> Id<NSUnitElectricPotentialDifference, Shared>;
        #[method_id(kilovolts)]
        pub unsafe fn kilovolts() -> Id<NSUnitElectricPotentialDifference, Shared>;
        #[method_id(volts)]
        pub unsafe fn volts() -> Id<NSUnitElectricPotentialDifference, Shared>;
        #[method_id(millivolts)]
        pub unsafe fn millivolts() -> Id<NSUnitElectricPotentialDifference, Shared>;
        #[method_id(microvolts)]
        pub unsafe fn microvolts() -> Id<NSUnitElectricPotentialDifference, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitElectricResistance;
    unsafe impl ClassType for NSUnitElectricResistance {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitElectricResistance {
        #[method_id(megaohms)]
        pub unsafe fn megaohms() -> Id<NSUnitElectricResistance, Shared>;
        #[method_id(kiloohms)]
        pub unsafe fn kiloohms() -> Id<NSUnitElectricResistance, Shared>;
        #[method_id(ohms)]
        pub unsafe fn ohms() -> Id<NSUnitElectricResistance, Shared>;
        #[method_id(milliohms)]
        pub unsafe fn milliohms() -> Id<NSUnitElectricResistance, Shared>;
        #[method_id(microohms)]
        pub unsafe fn microohms() -> Id<NSUnitElectricResistance, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitEnergy;
    unsafe impl ClassType for NSUnitEnergy {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitEnergy {
        #[method_id(kilojoules)]
        pub unsafe fn kilojoules() -> Id<NSUnitEnergy, Shared>;
        #[method_id(joules)]
        pub unsafe fn joules() -> Id<NSUnitEnergy, Shared>;
        #[method_id(kilocalories)]
        pub unsafe fn kilocalories() -> Id<NSUnitEnergy, Shared>;
        #[method_id(calories)]
        pub unsafe fn calories() -> Id<NSUnitEnergy, Shared>;
        #[method_id(kilowattHours)]
        pub unsafe fn kilowattHours() -> Id<NSUnitEnergy, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitFrequency;
    unsafe impl ClassType for NSUnitFrequency {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitFrequency {
        #[method_id(terahertz)]
        pub unsafe fn terahertz() -> Id<NSUnitFrequency, Shared>;
        #[method_id(gigahertz)]
        pub unsafe fn gigahertz() -> Id<NSUnitFrequency, Shared>;
        #[method_id(megahertz)]
        pub unsafe fn megahertz() -> Id<NSUnitFrequency, Shared>;
        #[method_id(kilohertz)]
        pub unsafe fn kilohertz() -> Id<NSUnitFrequency, Shared>;
        #[method_id(hertz)]
        pub unsafe fn hertz() -> Id<NSUnitFrequency, Shared>;
        #[method_id(millihertz)]
        pub unsafe fn millihertz() -> Id<NSUnitFrequency, Shared>;
        #[method_id(microhertz)]
        pub unsafe fn microhertz() -> Id<NSUnitFrequency, Shared>;
        #[method_id(nanohertz)]
        pub unsafe fn nanohertz() -> Id<NSUnitFrequency, Shared>;
        #[method_id(framesPerSecond)]
        pub unsafe fn framesPerSecond() -> Id<NSUnitFrequency, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitFuelEfficiency;
    unsafe impl ClassType for NSUnitFuelEfficiency {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitFuelEfficiency {
        #[method_id(litersPer100Kilometers)]
        pub unsafe fn litersPer100Kilometers() -> Id<NSUnitFuelEfficiency, Shared>;
        #[method_id(milesPerImperialGallon)]
        pub unsafe fn milesPerImperialGallon() -> Id<NSUnitFuelEfficiency, Shared>;
        #[method_id(milesPerGallon)]
        pub unsafe fn milesPerGallon() -> Id<NSUnitFuelEfficiency, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitInformationStorage;
    unsafe impl ClassType for NSUnitInformationStorage {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitInformationStorage {
        #[method_id(bytes)]
        pub unsafe fn bytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(bits)]
        pub unsafe fn bits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(nibbles)]
        pub unsafe fn nibbles() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(yottabytes)]
        pub unsafe fn yottabytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(zettabytes)]
        pub unsafe fn zettabytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(exabytes)]
        pub unsafe fn exabytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(petabytes)]
        pub unsafe fn petabytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(terabytes)]
        pub unsafe fn terabytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(gigabytes)]
        pub unsafe fn gigabytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(megabytes)]
        pub unsafe fn megabytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(kilobytes)]
        pub unsafe fn kilobytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(yottabits)]
        pub unsafe fn yottabits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(zettabits)]
        pub unsafe fn zettabits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(exabits)]
        pub unsafe fn exabits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(petabits)]
        pub unsafe fn petabits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(terabits)]
        pub unsafe fn terabits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(gigabits)]
        pub unsafe fn gigabits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(megabits)]
        pub unsafe fn megabits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(kilobits)]
        pub unsafe fn kilobits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(yobibytes)]
        pub unsafe fn yobibytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(zebibytes)]
        pub unsafe fn zebibytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(exbibytes)]
        pub unsafe fn exbibytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(pebibytes)]
        pub unsafe fn pebibytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(tebibytes)]
        pub unsafe fn tebibytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(gibibytes)]
        pub unsafe fn gibibytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(mebibytes)]
        pub unsafe fn mebibytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(kibibytes)]
        pub unsafe fn kibibytes() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(yobibits)]
        pub unsafe fn yobibits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(zebibits)]
        pub unsafe fn zebibits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(exbibits)]
        pub unsafe fn exbibits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(pebibits)]
        pub unsafe fn pebibits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(tebibits)]
        pub unsafe fn tebibits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(gibibits)]
        pub unsafe fn gibibits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(mebibits)]
        pub unsafe fn mebibits() -> Id<NSUnitInformationStorage, Shared>;
        #[method_id(kibibits)]
        pub unsafe fn kibibits() -> Id<NSUnitInformationStorage, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitLength;
    unsafe impl ClassType for NSUnitLength {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitLength {
        #[method_id(megameters)]
        pub unsafe fn megameters() -> Id<NSUnitLength, Shared>;
        #[method_id(kilometers)]
        pub unsafe fn kilometers() -> Id<NSUnitLength, Shared>;
        #[method_id(hectometers)]
        pub unsafe fn hectometers() -> Id<NSUnitLength, Shared>;
        #[method_id(decameters)]
        pub unsafe fn decameters() -> Id<NSUnitLength, Shared>;
        #[method_id(meters)]
        pub unsafe fn meters() -> Id<NSUnitLength, Shared>;
        #[method_id(decimeters)]
        pub unsafe fn decimeters() -> Id<NSUnitLength, Shared>;
        #[method_id(centimeters)]
        pub unsafe fn centimeters() -> Id<NSUnitLength, Shared>;
        #[method_id(millimeters)]
        pub unsafe fn millimeters() -> Id<NSUnitLength, Shared>;
        #[method_id(micrometers)]
        pub unsafe fn micrometers() -> Id<NSUnitLength, Shared>;
        #[method_id(nanometers)]
        pub unsafe fn nanometers() -> Id<NSUnitLength, Shared>;
        #[method_id(picometers)]
        pub unsafe fn picometers() -> Id<NSUnitLength, Shared>;
        #[method_id(inches)]
        pub unsafe fn inches() -> Id<NSUnitLength, Shared>;
        #[method_id(feet)]
        pub unsafe fn feet() -> Id<NSUnitLength, Shared>;
        #[method_id(yards)]
        pub unsafe fn yards() -> Id<NSUnitLength, Shared>;
        #[method_id(miles)]
        pub unsafe fn miles() -> Id<NSUnitLength, Shared>;
        #[method_id(scandinavianMiles)]
        pub unsafe fn scandinavianMiles() -> Id<NSUnitLength, Shared>;
        #[method_id(lightyears)]
        pub unsafe fn lightyears() -> Id<NSUnitLength, Shared>;
        #[method_id(nauticalMiles)]
        pub unsafe fn nauticalMiles() -> Id<NSUnitLength, Shared>;
        #[method_id(fathoms)]
        pub unsafe fn fathoms() -> Id<NSUnitLength, Shared>;
        #[method_id(furlongs)]
        pub unsafe fn furlongs() -> Id<NSUnitLength, Shared>;
        #[method_id(astronomicalUnits)]
        pub unsafe fn astronomicalUnits() -> Id<NSUnitLength, Shared>;
        #[method_id(parsecs)]
        pub unsafe fn parsecs() -> Id<NSUnitLength, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitIlluminance;
    unsafe impl ClassType for NSUnitIlluminance {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitIlluminance {
        #[method_id(lux)]
        pub unsafe fn lux() -> Id<NSUnitIlluminance, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitMass;
    unsafe impl ClassType for NSUnitMass {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitMass {
        #[method_id(kilograms)]
        pub unsafe fn kilograms() -> Id<NSUnitMass, Shared>;
        #[method_id(grams)]
        pub unsafe fn grams() -> Id<NSUnitMass, Shared>;
        #[method_id(decigrams)]
        pub unsafe fn decigrams() -> Id<NSUnitMass, Shared>;
        #[method_id(centigrams)]
        pub unsafe fn centigrams() -> Id<NSUnitMass, Shared>;
        #[method_id(milligrams)]
        pub unsafe fn milligrams() -> Id<NSUnitMass, Shared>;
        #[method_id(micrograms)]
        pub unsafe fn micrograms() -> Id<NSUnitMass, Shared>;
        #[method_id(nanograms)]
        pub unsafe fn nanograms() -> Id<NSUnitMass, Shared>;
        #[method_id(picograms)]
        pub unsafe fn picograms() -> Id<NSUnitMass, Shared>;
        #[method_id(ounces)]
        pub unsafe fn ounces() -> Id<NSUnitMass, Shared>;
        #[method_id(poundsMass)]
        pub unsafe fn poundsMass() -> Id<NSUnitMass, Shared>;
        #[method_id(stones)]
        pub unsafe fn stones() -> Id<NSUnitMass, Shared>;
        #[method_id(metricTons)]
        pub unsafe fn metricTons() -> Id<NSUnitMass, Shared>;
        #[method_id(shortTons)]
        pub unsafe fn shortTons() -> Id<NSUnitMass, Shared>;
        #[method_id(carats)]
        pub unsafe fn carats() -> Id<NSUnitMass, Shared>;
        #[method_id(ouncesTroy)]
        pub unsafe fn ouncesTroy() -> Id<NSUnitMass, Shared>;
        #[method_id(slugs)]
        pub unsafe fn slugs() -> Id<NSUnitMass, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitPower;
    unsafe impl ClassType for NSUnitPower {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitPower {
        #[method_id(terawatts)]
        pub unsafe fn terawatts() -> Id<NSUnitPower, Shared>;
        #[method_id(gigawatts)]
        pub unsafe fn gigawatts() -> Id<NSUnitPower, Shared>;
        #[method_id(megawatts)]
        pub unsafe fn megawatts() -> Id<NSUnitPower, Shared>;
        #[method_id(kilowatts)]
        pub unsafe fn kilowatts() -> Id<NSUnitPower, Shared>;
        #[method_id(watts)]
        pub unsafe fn watts() -> Id<NSUnitPower, Shared>;
        #[method_id(milliwatts)]
        pub unsafe fn milliwatts() -> Id<NSUnitPower, Shared>;
        #[method_id(microwatts)]
        pub unsafe fn microwatts() -> Id<NSUnitPower, Shared>;
        #[method_id(nanowatts)]
        pub unsafe fn nanowatts() -> Id<NSUnitPower, Shared>;
        #[method_id(picowatts)]
        pub unsafe fn picowatts() -> Id<NSUnitPower, Shared>;
        #[method_id(femtowatts)]
        pub unsafe fn femtowatts() -> Id<NSUnitPower, Shared>;
        #[method_id(horsepower)]
        pub unsafe fn horsepower() -> Id<NSUnitPower, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitPressure;
    unsafe impl ClassType for NSUnitPressure {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitPressure {
        #[method_id(newtonsPerMetersSquared)]
        pub unsafe fn newtonsPerMetersSquared() -> Id<NSUnitPressure, Shared>;
        #[method_id(gigapascals)]
        pub unsafe fn gigapascals() -> Id<NSUnitPressure, Shared>;
        #[method_id(megapascals)]
        pub unsafe fn megapascals() -> Id<NSUnitPressure, Shared>;
        #[method_id(kilopascals)]
        pub unsafe fn kilopascals() -> Id<NSUnitPressure, Shared>;
        #[method_id(hectopascals)]
        pub unsafe fn hectopascals() -> Id<NSUnitPressure, Shared>;
        #[method_id(inchesOfMercury)]
        pub unsafe fn inchesOfMercury() -> Id<NSUnitPressure, Shared>;
        #[method_id(bars)]
        pub unsafe fn bars() -> Id<NSUnitPressure, Shared>;
        #[method_id(millibars)]
        pub unsafe fn millibars() -> Id<NSUnitPressure, Shared>;
        #[method_id(millimetersOfMercury)]
        pub unsafe fn millimetersOfMercury() -> Id<NSUnitPressure, Shared>;
        #[method_id(poundsForcePerSquareInch)]
        pub unsafe fn poundsForcePerSquareInch() -> Id<NSUnitPressure, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitSpeed;
    unsafe impl ClassType for NSUnitSpeed {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitSpeed {
        #[method_id(metersPerSecond)]
        pub unsafe fn metersPerSecond() -> Id<NSUnitSpeed, Shared>;
        #[method_id(kilometersPerHour)]
        pub unsafe fn kilometersPerHour() -> Id<NSUnitSpeed, Shared>;
        #[method_id(milesPerHour)]
        pub unsafe fn milesPerHour() -> Id<NSUnitSpeed, Shared>;
        #[method_id(knots)]
        pub unsafe fn knots() -> Id<NSUnitSpeed, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitTemperature;
    unsafe impl ClassType for NSUnitTemperature {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitTemperature {
        #[method_id(kelvin)]
        pub unsafe fn kelvin() -> Id<NSUnitTemperature, Shared>;
        #[method_id(celsius)]
        pub unsafe fn celsius() -> Id<NSUnitTemperature, Shared>;
        #[method_id(fahrenheit)]
        pub unsafe fn fahrenheit() -> Id<NSUnitTemperature, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnitVolume;
    unsafe impl ClassType for NSUnitVolume {
        type Super = NSDimension;
    }
);
extern_methods!(
    unsafe impl NSUnitVolume {
        #[method_id(megaliters)]
        pub unsafe fn megaliters() -> Id<NSUnitVolume, Shared>;
        #[method_id(kiloliters)]
        pub unsafe fn kiloliters() -> Id<NSUnitVolume, Shared>;
        #[method_id(liters)]
        pub unsafe fn liters() -> Id<NSUnitVolume, Shared>;
        #[method_id(deciliters)]
        pub unsafe fn deciliters() -> Id<NSUnitVolume, Shared>;
        #[method_id(centiliters)]
        pub unsafe fn centiliters() -> Id<NSUnitVolume, Shared>;
        #[method_id(milliliters)]
        pub unsafe fn milliliters() -> Id<NSUnitVolume, Shared>;
        #[method_id(cubicKilometers)]
        pub unsafe fn cubicKilometers() -> Id<NSUnitVolume, Shared>;
        #[method_id(cubicMeters)]
        pub unsafe fn cubicMeters() -> Id<NSUnitVolume, Shared>;
        #[method_id(cubicDecimeters)]
        pub unsafe fn cubicDecimeters() -> Id<NSUnitVolume, Shared>;
        #[method_id(cubicCentimeters)]
        pub unsafe fn cubicCentimeters() -> Id<NSUnitVolume, Shared>;
        #[method_id(cubicMillimeters)]
        pub unsafe fn cubicMillimeters() -> Id<NSUnitVolume, Shared>;
        #[method_id(cubicInches)]
        pub unsafe fn cubicInches() -> Id<NSUnitVolume, Shared>;
        #[method_id(cubicFeet)]
        pub unsafe fn cubicFeet() -> Id<NSUnitVolume, Shared>;
        #[method_id(cubicYards)]
        pub unsafe fn cubicYards() -> Id<NSUnitVolume, Shared>;
        #[method_id(cubicMiles)]
        pub unsafe fn cubicMiles() -> Id<NSUnitVolume, Shared>;
        #[method_id(acreFeet)]
        pub unsafe fn acreFeet() -> Id<NSUnitVolume, Shared>;
        #[method_id(bushels)]
        pub unsafe fn bushels() -> Id<NSUnitVolume, Shared>;
        #[method_id(teaspoons)]
        pub unsafe fn teaspoons() -> Id<NSUnitVolume, Shared>;
        #[method_id(tablespoons)]
        pub unsafe fn tablespoons() -> Id<NSUnitVolume, Shared>;
        #[method_id(fluidOunces)]
        pub unsafe fn fluidOunces() -> Id<NSUnitVolume, Shared>;
        #[method_id(cups)]
        pub unsafe fn cups() -> Id<NSUnitVolume, Shared>;
        #[method_id(pints)]
        pub unsafe fn pints() -> Id<NSUnitVolume, Shared>;
        #[method_id(quarts)]
        pub unsafe fn quarts() -> Id<NSUnitVolume, Shared>;
        #[method_id(gallons)]
        pub unsafe fn gallons() -> Id<NSUnitVolume, Shared>;
        #[method_id(imperialTeaspoons)]
        pub unsafe fn imperialTeaspoons() -> Id<NSUnitVolume, Shared>;
        #[method_id(imperialTablespoons)]
        pub unsafe fn imperialTablespoons() -> Id<NSUnitVolume, Shared>;
        #[method_id(imperialFluidOunces)]
        pub unsafe fn imperialFluidOunces() -> Id<NSUnitVolume, Shared>;
        #[method_id(imperialPints)]
        pub unsafe fn imperialPints() -> Id<NSUnitVolume, Shared>;
        #[method_id(imperialQuarts)]
        pub unsafe fn imperialQuarts() -> Id<NSUnitVolume, Shared>;
        #[method_id(imperialGallons)]
        pub unsafe fn imperialGallons() -> Id<NSUnitVolume, Shared>;
        #[method_id(metricCups)]
        pub unsafe fn metricCups() -> Id<NSUnitVolume, Shared>;
    }
);
