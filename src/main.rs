/// Interpolation command-line utility
///
/// Given an input 2D matrix with missing values encoded as "nan",
/// output a 2D matrix with missing values interpolated from surrounding
/// non-empty values
// Use LinkedHashSet over HashSet to preserve insertion order,
// meaning we start interpolating from top-left-most nan of matrix
mod algorithm;
mod array;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let none_encoding = String::from("nan");
    let filename = &args[1];

    let output_filename = format!(
        "{}_interpolated.csv",
        filename.strip_suffix(".csv").unwrap_or(filename)
    );

    let source_array = array::parse_csv_to_array(filename, &none_encoding);
    let nones = array::find_nones_in_array(&source_array);
    let target_array = array::repair_array_inplace(nones, source_array);
    array::write_array_to_csv(&target_array, &output_filename)
}
