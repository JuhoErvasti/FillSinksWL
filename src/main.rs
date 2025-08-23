use std::path::Path;

use clap::{CommandFactory, Parser};
use fillsinkswl::fillsinkswl::fill_sinks_wl;

pub fn error_handler(class: gdal::errors::CplErrType, number: i32, message: &str) {
    let class_str = match class {
        gdal::errors::CplErrType::None => "[INFO]",
        gdal::errors::CplErrType::Debug => "[DEBUG]",
        gdal::errors::CplErrType::Warning => "[WARN]",
        gdal::errors::CplErrType::Failure => "[ERROR]",
        gdal::errors::CplErrType::Fatal => "[FATAL]",
    };

    let msg = format!("GDAL: {class_str}, [{number}] {message}");

    match class {
        gdal::errors::CplErrType::None => {
            println!("{msg}");
        },
        gdal::errors::CplErrType::Debug => {
            println!("{msg}");
        },
        gdal::errors::CplErrType::Warning => {
            println!("{msg}");
        },
        gdal::errors::CplErrType::Failure => {
            eprintln!("{msg}");
            panic!("Received a GDAL error, terminating program!");
        },
        gdal::errors::CplErrType::Fatal => {
            eprintln!("{msg}");
            panic!("Received a fatal GDAL error, terminating program!");
        },
    }
}

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(version, about, long_about = None)]
struct Cli {
    input: String,
    output: String,

    #[arg(long = "minimum-slope", value_name = "MINIMUM_SLOPE", help = "Minimum Slope in degrees")]
    minslope: Option<f64>,

    #[arg(long = "overwrite", value_name = "OVERWRITE", help = "Allow overwriting output file")]
    overwrite: bool,
}

fn show_short_usage() {
    match Cli::command().print_help() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("ERROR: Could not print usage: {err}");
        },
    }
}

fn main() {
    let cli = Cli::parse();
    let input = Path::new(cli.input.as_str());
    let output = Path::new(cli.output.as_str());
    let minslope = cli.minslope.unwrap_or(0.0);

    if minslope < 0. {
        eprintln!("ERROR: minimum slope has to be zero or above.");
        show_short_usage();
        return ();
    }

    if !cli.overwrite && output.exists() {
        eprintln!("ERROR: output already exists. You can choose to overwrite with the --overwrite option.\n");
        show_short_usage();
        return ();
    }

    if !input.exists() {
        eprintln!("ERROR: input file does not exist.\n");
        show_short_usage();
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
            eprintln!("ERROR: could not open input dataset: {err}");
            show_short_usage();
            return ();
        },
    };

    if ds.raster_count() != 1 {
        eprintln!("ERROR: input must have exactly 1 band.");
        show_short_usage();
        return ();
    }

    if let Some(out_driver) = gdal::DriverManager::get_output_driver_for_dataset_name(output, gdal::DriverType::Raster) {
        let in_band: gdal::raster::RasterBand = match ds.rasterband(1) {
            Ok(band) => band,
            Err(err) => {
                eprintln!("ERROR: Could not read rasterband: {err}");
                show_short_usage();
                return ();
            },
        };
        let in_buffer: gdal::raster::Buffer<f64> = match in_band.read_band_as() {
            Ok(buffer) => buffer,
            Err(err) => {
                eprintln!("ERROR: Could not read band into a buffer: {err}");
                show_short_usage();
                return ();
            },
        };

        let mut in_array: ndarray::Array2<f64> = match in_buffer.to_array() {
            Ok(array) => array,
            Err(err) => {
                eprintln!("ERROR: Could not buffer into an ndarray: {err}");
                show_short_usage();
                return ();
            },
        };

        let width = in_array.shape()[1];
        let height = in_array.shape()[0];

        let mut out_ds = match out_driver.create_with_band_type::<f64, _>(output, width, height, 1) {
            Ok(ds) => ds,
            Err(err) => {
                eprintln!("ERROR: Could not create output dataset: {err}!");
                show_short_usage();
                return ();
            },
        };

        match ds.spatial_ref() {
            Ok(crs) => {
                match out_ds.set_spatial_ref(&crs) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("WARNING: Could not set output CRS: {err}");
                    },
                }
            },
            Err(err) => {
                println!("WARNING: Could not read input CRS: {err}");
                println!("WARNING: Output CRS not set!");
            },
        }

        let nodata = match in_band.no_data_value() {
            Some(value) => value,
            None => {
                println!("WARNING: Could not read nodata value from input. Using -9999 instead.");
                -9999.
            },
        };

        let out_array = fill_sinks_wl(
            &mut in_array,
            minslope,
            nodata,
        );

        let mut out_band = match out_ds.rasterband(1) {
            Ok(band) => band,
            Err(err) => {
                eprintln!("ERROR: Could not get output rasterband: {err}!");
                show_short_usage();
                return ();
            },
        };

        let mut out_buffer = gdal::raster::Buffer::new(
            (width, height), out_array.into_raw_vec_and_offset().0
        );

        match out_band.write((0, 0), (width, height), &mut out_buffer) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("ERROR: Could not write output buffer to rasterband: {err}!");
                show_short_usage();
                return ();
            },
        }
    } else {
        eprintln!("ERROR: could not determine an appropriate raster driver from {}", output.to_str().unwrap_or("UNKNOWN OUTPUT"));
        show_short_usage();
        return ();
    }

}
