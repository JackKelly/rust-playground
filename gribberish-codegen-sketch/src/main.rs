use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Identification of originating/generating center.
enum Center {
    NCEP,
}

enum MasterTableVersion {
    V32,
    V33,
}

struct CenterAndTableVersions {
    center: Center,
    local_table_version: u8,
    master_table_version: MasterTableVersion,
}

impl CenterAndTableVersions {
    pub fn from_discipline_and_category_and_parameter_numbers(
        &self,
        discipline_num: u8,
        category_num: u8,
        parameter_num: u8,
    ) -> Option<Discipline> {
        match discipline_num {
            ..192 => Discipline::from_master_discipline_and_category_and_parameter_numbers(
                discipline_num,
                category_num,
                parameter_num,
                &self.master_table_version,
            ),

            // Reserved for local use:
            192..=254 => self.from_local_discipline_and_category_and_parameter_numbers(
                discipline_num,
                category_num,
                parameter_num,
            ),

            255 => None, // 255 means "missing"
        }
    }

    fn from_local_discipline_and_category_and_parameter_numbers(
        &self,
        discipline_num: u8,
        category_num: u8,
        parameter_num: u8,
    ) -> Option<Discipline> {
        match self.center {
            Center::NCEP => match discipline_num {
                192 => Some(Discipline::NcepFoo(
                    NcepFooProduct::from_category_and_parameter_numbers(
                        category_num,
                        parameter_num,
                    )?,
                )),
            },
        }
    }
}

enum Discipline {
    Meteorological(MeteorologicalProduct),
    Hydrological(HydrologicalProduct),

    // Local to NCEP:
    NcepFoo(NcepFooProduct),
    // etc.
}

impl Discipline {
    fn from_master_discipline_and_category_and_parameter_numbers(
        discipline_num: u8,
        category_num: u8,
        parameter_num: u8,
        master_table_version: &MasterTableVersion,
    ) -> Option<Self> {
        match discipline_num {
            0 => Some(Discipline::Meteorological(
                MeteorologicalProduct::from_category_and_parameter_numbers(
                    category_num,
                    parameter_num,
                )?,
            )),

            // Demo of how to handle a discipline number which changes meaning across different
            // master table versions. This discipline number is made up! Just for demo purposes!
            191 => match *master_table_version {
                MasterTableVersion::V32 => todo!(),
                MasterTableVersion::V33 => todo!(),
            },

            // Reserved for local use:
            192..=254 => panic!("Local disciplines should never be passed to this function!"),
            _ => None,
        }
    }
}

trait Product {
    fn from_category_and_parameter_numbers(category_num: u8, parameter_num: u8) -> Option<Self>
    where
        Self: Sized;
}

enum MeteorologicalProduct {
    Temperature(Temperature),
    Moisture(Moisture),
    // etc.
}

impl Product for MeteorologicalProduct {
    fn from_category_and_parameter_numbers(category_num: u8, parameter_num: u8) -> Option<Self>
    where
        Self: Sized,
    {
        match category_num {
            0 => Some(MeteorologicalProduct::Temperature(Temperature::from_u8(
                parameter_num,
            )?)),
            1 => Some(MeteorologicalProduct::Moisture(Moisture::from_u8(
                parameter_num,
            )?)),
            _ => None,
        }
    }
}

enum HydrologicalProduct {
    HydrologyBasicProduct, // TODO: Add embedded enum
}

impl Product for HydrologicalProduct {
    fn from_category_and_parameter_numbers(category_num: u8, parameter_num: u8) -> Option<Self>
    where
        Self: Sized,
    {
        todo!();
    }
}

#[derive(FromPrimitive)]
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

#[derive(FromPrimitive)]
enum Moisture {}

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

/// All the abbreviations which are common across all centers and all table versions.
///
/// `phf::Map` is compiled to a perfect hash table, which is O(1). In contrast,
/// matching strings compiles code which checks each string in turn, which is O(n).
static COMMON_ABBREV_TO_PRODUCT: phf::Map<&'static str, Discipline> = phf::phf_map! {
    "TMP" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::Temperature)),
    "VTMP" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::VirtualTemperature)),
    "POT" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::PotentialTemperature)),
    "EPOT" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::PseudoAdiabaticPotentialTemperature)),
    "TMAX" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::MaximumTemperature)),
    "TMIN" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::MinimumTemperature)),
    "DPT" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::DewPointTemperature)),
    "DEPR" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::DewPointDepression)),
    "LAPR" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::LapseRate)),
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
