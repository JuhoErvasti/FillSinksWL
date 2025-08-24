# Fill Sinks algorithm

A commandline tool to run an algorithm proposed by [Wang &
Liu](https://www.researchgate.net/publication/220649976_An_efficient_method_for_identifying_and_filling_surface_depressions_in_digital_elevation_models_for_hydrologic_analysis_and_modelling),
which processes digital elevation models (DEMs) and fills surface depressions
in them.

In addition this tool supports setting a minimum slope argument which is found
in other open-source implementations (QGIS, SAGA GIS).

References: Wang, L. & H. Liu (2006): An efficient method for identifying and
filling surface depressions in digital elevation models for hydrologic analysis
and modelling. International Journal of Geographical Information Science, Vol.
20, No. 2: 193-213.

## Usage

```
Usage: fillsinkswl [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>
  <OUTPUT>

Options:
      --minimum-slope <MINIMUM_SLOPE>  Minimum Slope in degrees. Default is 0.0
      --overwrite                      Allow overwriting output file
  -h, --help                           Print help
  -V, --version                        Print version
```

### Examples:

```shell
fillsinkswl input.tif output.tif
```

```shell
fillsinkswl input.tif output.tif --minimum-slope=0.1
```

## Installation

### Dependencies

[GDAL >= 3.4](https://gdal.org/en/stable/download.html)

### With Cargo from GitHub

```shell
cargo install --git https://github.com/JuhoErvasti/FillSinksWL
```
