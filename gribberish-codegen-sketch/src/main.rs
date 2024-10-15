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
        .center(OriginatingCenter::NCEP) // Required iff local tables are used.
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
