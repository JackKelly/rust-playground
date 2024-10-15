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

enum Discipline {
    Meteorological(MeteorologicalProduct),
    Hydrological(HydrologicalProduct),

    // Local to NCEP:
    NcepFoo(NcepFooProduct),
    // etc.
}

impl Discipline {
    pub fn from_discipline_and_category_and_parameter_numbers(
        discipline_num: u8,
        category_num: u8,
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Discipline> {
        // This function just routes the query to the functions which handle Disciplines specified
        // in either local or master tables.
        match discipline_num {
            ..192 => Discipline::from_master_discipline_and_category_and_parameter_numbers(
                discipline_num,
                category_num,
                parameter_num,
                center_and_table_versions,
            ),

            // Reserved for local use:
            192..=254 => Discipline::from_local_discipline_and_category_and_parameter_numbers(
                discipline_num,
                category_num,
                parameter_num,
                center_and_table_versions,
            ),

            255 => None, // 255 means "missing"
        }
    }

    fn from_local_discipline_and_category_and_parameter_numbers(
        discipline_num: u8,
        category_num: u8,
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Discipline> {
        match center_and_table_versions.center {
            Center::NCEP => match discipline_num {
                192 => Some(Discipline::NcepFoo(
                    NcepFooProduct::from_category_and_parameter_numbers(
                        category_num,
                        parameter_num,
                        center_and_table_versions,
                    )?,
                )),
            },
        }
    }

    fn from_master_discipline_and_category_and_parameter_numbers(
        discipline_num: u8,
        category_num: u8,
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Self> {
        match discipline_num {
            0 => Some(Discipline::Meteorological(
                MeteorologicalProduct::from_category_and_parameter_numbers(
                    category_num,
                    parameter_num,
                    center_and_table_versions,
                )?,
            )),

            // Demo of how to handle a discipline number which changes meaning across different
            // master table versions. This discipline number is made up! Just for demo purposes!
            191 => match center_and_table_versions.master_table_version {
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
    fn from_category_and_parameter_numbers(
        category_num: u8,
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Self>
    where
        Self: Sized;
}

enum MeteorologicalProduct {
    Temperature(Temperature),
    Moisture(Moisture),
    // etc.
}

impl Product for MeteorologicalProduct {
    fn from_category_and_parameter_numbers(
        category_num: u8,
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        match category_num {
            0 => Some(MeteorologicalProduct::Temperature(
                Temperature::from_parameter_num(parameter_num, center_and_table_versions)?,
            )),
            1 => Some(MeteorologicalProduct::Moisture(
                Moisture::from_parameter_num(parameter_num, center_and_table_versions)?,
            )),
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

trait Parameter {
    fn from_parameter_number(
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Self>
    where
        Self: Sized;

    fn abbrev(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn unit(&self) -> &'static str;
}

#[derive(FromPrimitive)]
enum Temperature {
    Temperature = 0,
    VirtualTemperature,
    PotentialTemperature,
    PseudoAdiabaticPotentialTemperature,
    MaximumTemperature,
    MinimumTemperature,
    DewPointTemperature,
    DewPointDepression,
    LapseRate,
    // etc.

    // NCEP local:
    NcepSnowPhaseChangeHeatFlux,
    NcepTemperatureTendencyByAllRadiation,
    // etc.
}

#[derive(FromPrimitive)]
enum Moisture {}

impl Parameter for Temperature {
    fn from_parameter_number(
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if parameter_num < 192 {
            Temperature::from_u8(parameter_num)
        } else {
            // Parameter numbers >= 194 are reserved for local use:
            match center_and_table_versions.center {
                Center::NCEP => match parameter_num {
                    192 => Some(Temperature::NcepSnowPhaseChangeHeatFlux),
                    193 => Some(Temperature::NcepTemperatureTendencyByAllRadiation),
                    _ => todo!(),
                },
            }
        }
    }

    fn abbrev(&self) -> &'static str {
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

            // Local to NCEP:
            Temperature::NcepSnowPhaseChangeHeatFlux => "SNOHF",
            Temperature::NcepTemperatureTendencyByAllRadiation => "TTRAD",
        }
    }

    fn name(&self) -> &'static str {
        match *self {
            Temperature::Temperature => "Temperature",
            Temperature::VirtualTemperature => "Virtual temperature",
            _ => todo!(), // etc...
        }
    }

    fn unit(&self) -> &'static str {
        todo!();
    }
}

/// All the abbreviations which are common across all centers and all table versions.
///
/// `phf::Map` is compiled to a perfect hash table, which is O(1). In contrast,
/// matching strings compiles code which checks each string in turn, which is O(n).
static ABBREV_TO_PRODUCT_COMMON: phf::Map<&'static str, Discipline> = phf::phf_map! {
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

static ABBREV_TO_PRODUCT_NCEP: phf::Map<&'static str, Discipline> = phf::phf_map! {
    "SNOHF" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::NcepSnowPhaseChangeHeatFlux)),
    "TTRAD" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::NcepTemperatureTendencyByAllRadiation)),
};

// Contains only the diff between master table V32 and the common abbreviations.
static ABBREV_TO_PRODUCT_MASTER_TABLE_V32: phf::Map<&'static str, Discipline> = phf::phf_map! {
    "FOO" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::Foo)),
    "BAR" => Discipline::Meteorological(MeteorologicalProduct::Temperature(Temperature::Bar)),
};

pub fn abbrev_to_product(
    abbrev: &str,
    center_and_table_versions: &CenterAndTableVersions,
) -> Option<&'static Discipline> {
    // First, try to common abbreviations:
    if let Some(discipline) = ABBREV_TO_PRODUCT_COMMON.get(abbrev) {
        return Some(discipline);
    }

    // Next, try the abbreviations defined by the local Center:
    let local = match center_and_table_versions.center {
        Center::NCEP => ABBREV_TO_PRODUCT_NCEP.get(abbrev),
    };
    if let Some(discipline) = local {
        return Some(discipline);
    }

    // Finally, use the abbreviations defined in the specific version of the master table:
    match center_and_table_versions.master_table_version {
        MasterTableVersion::V32 => ABBREV_TO_PRODUCT_MASTER_TABLE_V32.get(abbrev),
        _ => todo!(),
    }
}

// TODO: Think about the API for decoding vertical levels and steps from IDX files and from GRIB.
// TODO: Split the code above into separate rust files.
// TODO: Update this main function.
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
