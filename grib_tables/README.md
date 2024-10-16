An deprecated design for hypergrib/crates/grib_tables.

This design sketch tries to faithfully capture the GRIB tables hierarchy (Discipline > Category > Parameter).
But this feels overkill. It might be _slightly_ faster because we can create a perfect hash at compile time.
But then I realised that we only need to look things up in the GRIB table whilst indexing a new GRIB dataset.
When "normal users" use the dataset, they can just read the metadata that we create for the dataset.


## Old TODO:
- [ ] Think about the API for decoding vertical levels and steps from IDX files and from GRIB.
      Probably using GRIB Templates, e.g. https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-0.shtml
- [ ] Update the unit test until it compiles.
- [ ] Write a few paragraphs in this README about the structure of the code, and the three main use-cases (converting from 
      numbers to Products, converting from abbreviation strings to Products, and getting information about each Product).
- [ ] Flesh the tables out (manually) with enough data to get started
- [ ] Link to this code from https://github.com/mpiannucci/gribberish/issues/63
- [ ] Also open issue on https://github.com/noritada/grib-rs to ask if it makes sense to have a common repo for the GRIB code tables
- [ ] Think about code-gen for some parts (e.g. the parameter tables).
