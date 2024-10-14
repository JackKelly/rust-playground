/// Identification of originating/generating center.
enum Center {}

#[derive(num_derive::FromPrimitive)]
enum Discipline {
    Meteorological = 0,
    Hydrological = 1,
    // etc.
}

trait ProductMarker {}

impl Discipline {
    fn category(&self, category_number: u8) -> Box<dyn ProductMarker> {
        match self {
            Discipline::Meteorological => {
                Box::new(MeteorologicalProduct::from_u8(category_number).unwrap())
            }
            Discipline::Hydrological => {
                Box::new(HydrologicalProduct::from_u8(category_number).unwrap())
            }
        }
    }
}

#[derive(num_derive::FromPrimitive)]
enum MeteorologicalProduct {
    Temperature = 0,
    Moisture = 1,
    // etc.
}

#[derive(num_derive::FromPrimitive)]
enum HydrologicalProduct {
    HydrologyBasicProduct = 0,
}

impl ProductMarker for MeteorologicalProduct {}
impl ProductMarker for HydrologicalProduct {}

#[derive(num_derive::FromPrimitive)]
enum Temperature {
    Temperature,
    VirtualTemperature,
    PotentialTemperature,
    PseudoAdiabaticPotentialTemperature,
    MaximumTemperature,
    MinimumTemperature,
    DewPointTemperature,
    DewPointDepression,
    LapseRate,
    // etc.
}

trait Parameter {}

impl Parameter for Temperature;

impl Temperature {
    #[no_mangle] // Required when viewing this in godbolt.org
    pub fn abbrev(&self) -> &'static str {
        // This gets compiled to a jump table, which is O(1). See:
        // https://www.reddit.com/r/rust/comments/31kras/are_match_statements_constanttime_operations/
        match *self {
            Temperature::Temperature => "TMP",
            Temperature::VirtualTemperature => "VTMP",
            Temperature::PotentialTemperature => "POT",
            Temperature::PseudoAdiabaticPotentialTemperature => "EPOT",
            Temperature::MaximumTemperature => "TMAX",
            Temperature::MinimumTemperature => "TMIN",
            Temperature::DewPointTemperature => "DPT",
            Temperature::DewPointDepression => "DEPR",
            Temperature::LapseRate => "LAPR",
            // etc.
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Temperature::Temperature => "Temperature",
            Temperature::VirtualTemperature => "Virtual temperature",
            _ => todo!(), // etc...
        }
    }

    pub fn unit(&self) -> &'static str {
        todo!();
    }
}

// `phf::Map` is compiled to a perfect hash table, which is O(1). In contrast,
// matching strings compiles code which checks each string in turn, which is O(n).
// TODO: Maybe we do need a super-struct of all Parameters?
static ABBREV_TO_PRODUCT_VARIANT: phf::Map<&'static str, Box<dyn Parameter>> = phf::phf_map! {
    "TMP" => Box::new(Temperature::Temperature),
    "VTMP" => Box::new(Temperature::VirtualTemperature),
    "POT" => Box::new(Temperature::PotentialTemperature),
    "EPOT" => Box::new(Temperature::PseudoAdiabaticPotentialTemperature),
    "TMAX" => Box::new(Temperature::MaximumTemperature),
    "TMIN" => Box::new(Temperature::MinimumTemperature),
    "DPT" => Box::new(Temperature::DewPointTemperature),
    "DEPR" => Box::new(Temperature::DewPointDepression),
    "LAPR" => Box::new(Temperature::LapseRate),
};

fn main() {
    // First, build a ParamDecoder:
    let param_decoder: ParamDecoder = ParamDecoderBuilder::new()
        .master_tables_version_number(24) // Optional. Defaults to latest table version.
        .with_local_tables_version_number(4) // Optional. Defaults to ignoring local tables.
        .center(Center::NCEP) // Required iff local tables are used.
        .build();

    // Now, there are two ways to "decode" a parameter:
    // 1) Decode from the binary data in the GRIB sections:
    let param = param_decoder
        .from_grib() // -> ParamDecoderFromGrib
        .discipline(Discipline::from_u8(0)) // or Discipline::from_u8(0)
        .category(MeteorologicalProduct::from_u8(1)) //
        .parameter_number(0)
        .build();

    // 2) Or decode from the abbreviation string from an .idx file:
    let param = param_decoder
        .from_idx() // -> ParamDecoderFromIdx
        .abbrev("TMP")
        .build();

    // Now print information:
    println!("{}, {}, {}", param.name(), param.unit(), param.abbrev());

    // The above code is probably sufficient for the MVP. Later, we could add things like:
    let param = param_decoder
        .from_idx() // -> ParamDecoderFromIdx
        .abbrev("TMP")
        .vertical_level("5000-2000 m above ground")
        .step("775-780 min ave fcst")
        .build();
}
