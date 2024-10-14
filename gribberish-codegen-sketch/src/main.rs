#[derive(num_derive::FromPrimitive)]
enum Discipline {
    Meteorological = 0,
    // etc.
}

#[derive(num_derive::FromPrimitive)]
enum Category {
    Temperature = 0,
    Moisture = 1,
    // etc.
}

impl Category {
    fn discipline(&self) -> Discipline {
        match *self {
            Category::Temperature => Discipline::Meteorological,
            Category::Moisture => Discipline::Meteorological,
        }
    }
}

trait Parameter: Send + Sync {
    pub fn abbrev(&self) -> &'static str;
    pub fn name(&self) -> &'static str;
    pub fn unit(&self) -> &'static str;
}

// The `Parameter` enum variant names would be derived from the `name` field in the GDAL CSVs,
// with the strings turned into CammelCase, and the whitespace removed, and ignoring rows where
// the `name` contains "Reserved". Maybe this single enum would contain _all_ parameters
// (for all categories)?
#[derive(num_derive::FromPrimitive)]
enum TemperatureProduct {
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

impl Parameter for TemperatureProduct {
    fn abbrev(&self) -> &'static str {
        // This gets compiled to a jump table, which is O(1). See:
        // https://www.reddit.com/r/rust/comments/31kras/are_match_statements_constanttime_operations/
        match *self {
            Self::Temperature => "TMP",
            Self::VirtualTemperature => "VTMP",
            Self::PotentialTemperature => "POT",
            Self::PseudoAdiabaticPotentialTemperature => "EPOT",
            Self::MaximumTemperature => "TMAX",
            Self::MinimumTemperature => "TMIN",
            Self::DewPointTemperature => "DPT",
            Self::DewPointDepression => "DEPR",
            Self::LapseRate => "LAPR",
            // etc.
        }
    }

    fn name(&self) -> &'static str {
        match *self {
            Self::Temperature => "Temperature",
            Self::VirtualTemperature => "Virtual temperature",
            _ => todo!(), // etc...
        }
    }

    fn unit(&self) -> &'static str {
        match &self {
            Self::Temperature => "K",
            _ => todo!(),
        }
    }
}

// `phf::Map` is compiled to a perfect hash table, which is O(1). In contrast,
// matching strings compiles code which checks each string in turn, which is O(n).
static ABBREV_TO_PRODUCT_VARIANT: phf::Map<&'static str, Box<dyn Parameter>> = phf::phf_map! {
    "TMP" => Box::new(TemperatureProduct::Temperature),
    "VTMP" => Box::new(TemperatureProduct::VirtualTemperature),
    "POT" => Box::new(TemperatureProduct::PotentialTemperature),
    "EPOT" => Box::new(TemperatureProduct::PseudoAdiabaticPotentialTemperature),
    "TMAX" => Box::new(TemperatureProduct::MaximumTemperature),
    "TMIN" => Box::new(TemperatureProduct::MinimumTemperature),
    "DPT" => Box::new(TemperatureProduct::DewPointTemperature),
    "DEPR" => Box::new(TemperatureProduct::DewPointDepression),
    "LAPR" => Box::new(TemperatureProduct::LapseRate),
};

// struct Key(u32);
//
// impl Key {
//     const fn new(discipline: u8, category: u8, parameter: u8) -> Self {
//         let mut key: u32 = discipline as u32;
//         key |= (category as u32) << 8;
//         key |= (parameter as u32) << 16;
//         Self(key)
//     }
// }
//

struct Key {
    discipline: u8,
    category: u8,
    parameter: u8,
}

impl From<Key> for Parameter {
    fn from(value: Key) -> Self {
        match value {
            Key {
                discipline: 0,
                category: 0,
                parameter: 0,
            } => Temperature,
            _ => todo!(),
        }
    }
}

impl std::str::FromStr for &Parameter {
    type Err = String;

    #[no_mangle]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ABBREV_TO_PRODUCT_VARIANT.get(&s) {
            Some(prod) => Ok(prod),
            None => Err(format!("Unrecognised product abbreviation: {s}")),
        }
    }
}

fn main() {
    // Convert abbreviation string to Parameter:
    let abbrev = "TMP";
    let param: &Parameter = abbrev.parse().unwrap();

    // Get other information about the parameter:
    let name = param.name();
    let category = param.category();
    let discipline = category.discipline();
}
