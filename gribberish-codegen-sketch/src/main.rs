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

enum Product {
    Meteorological(MeteorologicalCategory),
    Hydrological(HydrologicalCategory),

    // Local to NCEP:
    NcepFoo(NcepFooProduct),
    // etc.
}

impl Product {
    pub fn from_discipline_and_category_and_parameter_numbers(
        discipline_num: u8,
        category_num: u8,
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Product> {
        // This function just routes the query to the functions which handle Disciplines specified
        // in either local or master tables.
        match discipline_num {
            ..192 => Product::from_master_discipline_and_category_and_parameter_numbers(
                discipline_num,
                category_num,
                parameter_num,
                center_and_table_versions,
            ),

            // Reserved for local use:
            192..=254 => Product::from_local_discipline_and_category_and_parameter_numbers(
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
    ) -> Option<Product> {
        match center_and_table_versions.center {
            Center::NCEP => match discipline_num {
                192 => Some(Product::NcepFoo(
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
            0 => Some(Product::Meteorological(
                MeteorologicalCategory::from_category_and_parameter_numbers(
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

trait Category {
    fn from_category_and_parameter_numbers(
        category_num: u8,
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Self>
    where
        Self: Sized;
}

enum MeteorologicalCategory {
    Temperature(TemperatureParameter),
    Moisture(MoistureParameter),
    // etc.
}

impl Category for MeteorologicalCategory {
    fn from_category_and_parameter_numbers(
        category_num: u8,
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        match category_num {
            0 => Some(MeteorologicalCategory::Temperature(
                TemperatureParameter::from_parameter_num(parameter_num, center_and_table_versions)?,
            )),
            1 => Some(MeteorologicalCategory::Moisture(
                MoistureParameter::from_parameter_num(parameter_num, center_and_table_versions)?,
            )),
            _ => None,
        }
    }
}

enum HydrologicalCategory {
    HydrologyBasicProduct, // TODO: Add embedded enum
}

impl Category for HydrologicalCategory {
    fn from_category_and_parameter_numbers(
        category_num: u8,
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        todo!();
    }
}

trait Parameter {
    fn from_parameter_num(
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
enum TemperatureParameter {
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
enum MoistureParameter {}

impl Parameter for TemperatureParameter {
    fn from_parameter_num(
        parameter_num: u8,
        center_and_table_versions: CenterAndTableVersions,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if parameter_num < 192 {
            TemperatureParameter::from_u8(parameter_num)
        } else {
            // Parameter numbers >= 194 are reserved for local use:
            match center_and_table_versions.center {
                Center::NCEP => match parameter_num {
                    192 => Some(TemperatureParameter::NcepSnowPhaseChangeHeatFlux),
                    193 => Some(TemperatureParameter::NcepTemperatureTendencyByAllRadiation),
                    _ => todo!(),
                },
            }
        }
    }

    fn abbrev(&self) -> &'static str {
        // This gets compiled to a jump table, which is O(1). See:
        // https://www.reddit.com/r/rust/comments/31kras/are_match_statements_constanttime_operations/
        match *self {
            TemperatureParameter::Temperature => "TMP",
            TemperatureParameter::VirtualTemperature => "VTMP",
            TemperatureParameter::PotentialTemperature => "POT",
            TemperatureParameter::PseudoAdiabaticPotentialTemperature => "EPOT",
            TemperatureParameter::MaximumTemperature => "TMAX",
            TemperatureParameter::MinimumTemperature => "TMIN",
            TemperatureParameter::DewPointTemperature => "DPT",
            TemperatureParameter::DewPointDepression => "DEPR",
            TemperatureParameter::LapseRate => "LAPR",
            // etc.

            // Local to NCEP:
            TemperatureParameter::NcepSnowPhaseChangeHeatFlux => "SNOHF",
            TemperatureParameter::NcepTemperatureTendencyByAllRadiation => "TTRAD",
        }
    }

    fn name(&self) -> &'static str {
        match *self {
            TemperatureParameter::Temperature => "Temperature",
            TemperatureParameter::VirtualTemperature => "Virtual temperature",
            _ => todo!(), // etc...
        }
    }

    fn unit(&self) -> &'static str {
        todo!();
    }
}

/// All the abbreviations which are common across all centers and all table versions.
///
/// To decode .idx files, we need a single hashmap which holds every abbreviation string.
/// So the values of the hashmap have to all be the same type.
///
/// `phf::Map` is compiled to a perfect hash table, which is O(1). In contrast,
/// matching strings compiles code which checks each string in turn, which is O(n).
static ABBREV_TO_PRODUCT_COMMON: phf::Map<&'static str, Product> = phf::phf_map! {
    "TMP" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::Temperature)),
    "VTMP" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::VirtualTemperature)),
    "POT" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::PotentialTemperature)),
    "EPOT" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::PseudoAdiabaticPotentialTemperature)),
    "TMAX" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::MaximumTemperature)),
    "TMIN" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::MinimumTemperature)),
    "DPT" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::DewPointTemperature)),
    "DEPR" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::DewPointDepression)),
    "LAPR" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::LapseRate)),
};

static ABBREV_TO_PRODUCT_NCEP: phf::Map<&'static str, Product> = phf::phf_map! {
    "SNOHF" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::NcepSnowPhaseChangeHeatFlux)),
    "TTRAD" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::NcepTemperatureTendencyByAllRadiation)),
};

// Contains only the diff between master table V32 and the common abbreviations.
static ABBREV_TO_PRODUCT_MASTER_TABLE_V32: phf::Map<&'static str, Product> = phf::phf_map! {
    "FOO" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::Foo)),
    "BAR" => Product::Meteorological(MeteorologicalCategory::Temperature(TemperatureParameter::Bar)),
};

pub fn abbrev_to_product(
    abbrev: &str,
    center_and_table_versions: &CenterAndTableVersions,
) -> Option<&'static Product> {
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
//       Probably using GRIB Templates, e.g. https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-0.shtml
// TODO: Move all this code into hypergrib/crates/grib_tables
// TODO: Split the code above into separate rust files.
// TODO: Update this main function.
// TODO: Flesh the tables out (manually) with enough data to get started
// TODO: Link to this code from https://github.com/mpiannucci/gribberish/issues/63
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
        .discipline(Product::from_u8(0)) // or Discipline::from_u8(0)
        .category(MeteorologicalCategory::from_u8(1)) //
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
