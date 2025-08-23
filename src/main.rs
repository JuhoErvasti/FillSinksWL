use std::path::Path;

use clap::{CommandFactory, Parser};

// TODO: set GDAL error handler

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(version, about, long_about = None)]
struct Cli {
    input: String,
    output: String,

    // TODO: should be > 0
    #[arg(long = "minimum-slope", value_name = "MINIMUM_SLOPE", help = "Minimum Slope in degrees", long_help = "FIXME:")]
    minslope: Option<f64>,

    #[arg(long = "overwrite", value_name = "OVERWRITE", help = "Allow overwriting output file", long_help = "FIXME:")]
    overwrite: bool,
}

fn main() {
    let cli = Cli::parse();
    let input = Path::new(cli.input.as_str());
    let output = Path::new(cli.output.as_str());
    let minslope = cli.minslope.unwrap_or(0.1);

    if !cli.overwrite && output.exists() {
        println!("ERROR: output already exists. You can choose to overwrite with the --overwrite option.\n");
        Cli::command().print_help().unwrap();
        return ();
    }

    if !input.exists() {
        println!("ERROR: input file does not exist.\n");
        Cli::command().print_help().unwrap();
        return ();
    }

    let flags = gdal::GdalOpenFlags::GDAL_OF_RASTER | gdal::GdalOpenFlags::GDAL_OF_READONLY;
    let options = gdal::DatasetOptions {
        open_flags: flags,
        allowed_drivers: None,
        open_options: None,
        sibling_files: None,
    };


    let ds = match gdal::Dataset::open_ex(input, options) {
        Ok(ds) => ds,
        Err(err) => {
            panic!("ERROR: {}", err.to_string());
        },
    };

    if ds.raster_count() != 1 {
        println!("ERROR: input must have exactly 1 band.");
    }

    if let Some(out_driver) = gdal::DriverManager::get_output_driver_for_dataset_name(output, gdal::DriverType::Raster) {
        let in_band: gdal::raster::RasterBand = ds.rasterband(1).unwrap();
        let in_buffer: gdal::raster::Buffer<f64> = in_band.read_band_as().unwrap();
        let in_crs = ds.spatial_ref().unwrap();
        let in_array: ndarray::Array2<f64> = in_buffer.to_array().unwrap();

        let width = in_array.shape()[1];
        let height = in_array.shape()[0];

        let mut out_ds = out_driver.create_with_band_type::<f64, _>(
            output,
            width,
            height,
            1,
        ).unwrap();

        out_ds.set_spatial_ref(&in_crs).unwrap();
        let mut out_band = out_ds.rasterband(1).unwrap();
        let mut out_buffer = gdal::raster::Buffer::new(
            (width, height), in_array.into_raw_vec_and_offset().0
        );

        out_band.write(
            (0, 0),
            (width, height),
            &mut out_buffer,
        ).unwrap();
    } else {
        println!("ERROR: could not determine an appropriate raster driver from {}", output.to_str().unwrap());
        Cli::command().print_help().unwrap();
        return ();
    }

}
