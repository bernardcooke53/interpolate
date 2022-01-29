// Interpolation command-line utility
//
// Given an input 2D matrix with missing values encoded as "nan",
// output a 2D matrix with missing values interpolated from surrounding
// non-empty values

// Use LinkedHashSet over HashSet to preserve insertion order,
// meaning we start interpolating from top-left-most nan of matrix
use csv;
use linked_hash_set::LinkedHashSet;
use ndarray::prelude::*;
use ndarray_csv::{Array2Reader, Array2Writer};
use std::env;
// use std::num::ParseFloatError;

fn interpolate(v: Vec<&f64>) -> f64 {
    if v.len() == 0 {
        0.0
    } else {
        v.clone().into_iter().sum::<f64>() / (v.len() as f64)
    }
}

fn parse_csv(filename: &String) -> Array2<Option<f64>> {
    if let Ok(mut reader) = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)
    {
        let source_array: Array2<String> = reader
            .deserialize_array2_dynamic()
            .expect("Inconsistent array dimensions");
        source_array.mapv_into_any::<Option<f64>, _>(|elem| {
            if elem == "nan" {
                return None;
            } else {
                Some(
                    elem.parse::<f64>()
                        .expect(&format!("Invalid float: {}", elem)),
                )
            }
        })
    } else {
        panic!("Couldn't read that file!");
    }
}

fn find_nones(arr: &Array2<Option<f64>>) -> LinkedHashSet<(usize, usize)> {
    let mut nones: LinkedHashSet<(usize, usize)> = LinkedHashSet::new();
    'a: for ((i, j), val) in arr.indexed_iter() {
        match val {
            Some(_) => {
                continue 'a;
            }
            None => {
                nones.insert((i, j));
            }
        }
    }
    nones
}

enum VectorDirection {
    Left,
    Down,
    Up,
    Right,
}

fn walk<'a>(
    vec: Vec<&'a Option<f64>>,
    vec_direc: VectorDirection,
    start: (usize, usize),
    nans: &LinkedHashSet<(usize, usize)>,
) -> Option<&'a f64> {
    let (start_row, start_col) = start;
    let make_nan_search_index = |k| match vec_direc {
        VectorDirection::Left => dbg!((start_row, start_col - k)),
        VectorDirection::Down => dbg!((start_row + k, start_col)),
        VectorDirection::Up => dbg!((start_row - k, start_col)),
        VectorDirection::Right => dbg!((start_row, start_col + k)),
    };

    let break_condition = |item: &(usize, &Option<f64>)| {
        let index = dbg!(item.0);
        let val = item.1;
        let search_idx = make_nan_search_index(index);

        dbg!(&search_idx);
        if dbg!(!nans.contains(&search_idx)) {
            match val {
                Some(_) => return true,
                None => return false,
            }
        }
        false
    };

    let mut res: Option<&Option<f64>> = None;
    // let res = vec.into_iter().enumerate().find(break_condition);
    for (index, val) in vec.into_iter().enumerate() {
        if dbg!(break_condition(&(index, val))) {
            res = Some(val);
            break;
        }
    }

    match res {
        Some(v) => {
            // If the predicate was met then we know it's
            // safe to unwrap as v.1 definitely has Some(val)
            return Some(v.as_ref().unwrap());
        }
        None => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let output_filename = format!(
        "{}_interpolated.csv",
        filename.strip_suffix(".csv").unwrap_or(filename)
    );

    //   let (array_dimensions, index_to_val, mut nans) = parse_csv(filename);
    let mut array = parse_csv(filename);
    let shape = array.shape();
    let (length, width) = (shape[0], shape[1]);
    let mut nans = find_nones(&array);
    println!("{:?}", array);
    println!("\n");
    println!("{:?}", nans);
    while nans.len() > 0 {
        // for (rownum, colnum) in nans.iter().cloned().to_owned() {
        let (rownum, colnum) = dbg!(nans.iter().next().cloned().unwrap());
        // Remove this from nans so we know what's already been calculated
        nans.remove(&(rownum, colnum));
        dbg!(&nans);

        // Note if we're walking left or up we want to start closest to our current value -
        // so we need to reverse the vectors.
        // ArrayBase<ViewRepr<_>>.indexed_iter doesn't have the required trait implementation
        // for reversing so we use vectors instead

        let left_sl = dbg!(array
            .slice(s![rownum, 0..colnum;-1])
            .into_iter()
            .collect::<Vec<_>>());

        let right_sl = dbg!(array
            .slice(s![rownum, (colnum + 1)..width])
            .into_iter()
            .collect::<Vec<_>>());

        let up_sl = dbg!(array
            .slice(s![0..rownum;-1, colnum..(colnum + 1)])
            .into_iter()
            .collect::<Vec<_>>());

        let down_sl = dbg!(array
            .slice(s![(rownum + 1)..length, colnum..(colnum + 1)])
            .into_iter()
            .collect::<Vec<_>>());

        let left = walk(left_sl, VectorDirection::Left, (rownum, colnum), &nans);
        let down = walk(down_sl, VectorDirection::Down, (rownum, colnum), &nans);
        let up = walk(up_sl, VectorDirection::Up, (rownum, colnum), &nans);
        let right = walk(right_sl, VectorDirection::Right, (rownum, colnum), &nans);
        let present: Vec<&f64> = dbg!(vec![up, down, left, right].into_iter().flatten().collect());

        let new_value = dbg!(interpolate(present));
        println!(
            "Point ({}, {}) --- Up: {} Down: {}, Left: {}, Right: {}, New Value: {}",
            rownum.to_string(),
            colnum.to_string(),
            up.map(|c| c.to_string())
                .unwrap_or(String::from("<Out of bounds>")),
            down.map(|c| c.to_string())
                .unwrap_or(String::from("<Out of bounds>")),
            left.map(|c| c.to_string())
                .unwrap_or(String::from("<Out of bounds>")),
            right
                .map(|c| c.to_string())
                .unwrap_or(String::from("<Out of bounds>")),
            new_value.to_string(),
        );

        //
        // Value found, time to write it in
        //

        array[[rownum, colnum]] = Some(new_value);
    }

    println!("Repaired array:");
    println!("{:?}", array);
}
