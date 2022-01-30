use crate::algorithm::{average as vec_average, walk, VectorDirection};
use csv;
use linked_hash_set::LinkedHashSet;
use ndarray::prelude::*;
use ndarray_csv::{Array2Reader, Array2Writer};
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::str::FromStr;

pub fn parse_csv_to_array<T: 'static>(
    filename: &String,
    none_encoding: &String,
) -> Array2<Option<T>>
where
    T: FromStr + PartialEq + Clone,
    <T as FromStr>::Err: Debug,
{
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)
        .expect(&format!("Couldn't read file: {}", filename));

    let source_array: Array2<String> = reader
        .deserialize_array2_dynamic()
        .expect("Inconsistent array dimensions");

    source_array.mapv_into_any::<Option<T>, _>(|elem| {
        parse_none_encoding::<T>(&elem, &none_encoding).expect(&format!("Couldn't parse: {}", elem))
    })
}

pub fn write_array_to_csv(
    array: &Array2<Option<f64>>,
    output_filename: &String,
) -> Result<(), Box<dyn Error>> {
    let file = File::create(output_filename)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    Ok(writer.serialize_array2(array)?)
}

pub fn find_nones(arr: &Array2<Option<f64>>) -> LinkedHashSet<(usize, usize)> {
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

fn parse_none_encoding<T>(
    from: &String,
    none_encoding: &String,
) -> Result<Option<T>, <T as FromStr>::Err>
where
    T: FromStr + PartialEq,
    <T as FromStr>::Err: Debug,
{
    if from == none_encoding {
        Ok(None)
    } else {
        match from.parse::<T>() {
            Ok(v) => Ok(Some(v)),
            Err(e) => Err(e),
        }
    }
}

pub fn repair_array_inplace(
    mut nones: LinkedHashSet<(usize, usize)>,
    mut array: Array2<Option<f64>>,
) -> Array2<Option<f64>> {
    let shape = array.shape();
    let (length, width) = (shape[0], shape[1]);

    while nones.len() > 0 {
        // Use this construct instead if we don't want to interpolate recursively
        // based on our already interpolated values
        // for (rownum, colnum) in nones.iter().cloned().to_owned() {

        let (rownum, colnum) = nones.iter().next().cloned().unwrap();
        dbg!(&(rownum, colnum));
        // Remove this from nones so we know what's already been calculated
        nones.remove(&(rownum, colnum));
        // Note if we're walking left or up we want to start closest to our current value -
        // so we need to reverse the vectors.
        // ArrayBase<ViewRepr<_>>.indexed_iter doesn't have the required trait implementation
        // for reversing so we use vectors instead

        let left_sl = array
            .slice(s![rownum, 0..colnum;-1])
            .into_iter()
            .collect::<Vec<_>>();

        let right_sl = array
            .slice(s![rownum, colnum..width])
            .into_iter()
            .collect::<Vec<_>>();

        let up_sl = array
            .slice(s![0..rownum;-1, colnum..(colnum + 1)])
            .into_iter()
            .collect::<Vec<_>>();

        let down_sl = array
            .slice(s![rownum..length, colnum..(colnum + 1)])
            .into_iter()
            .collect::<Vec<_>>();

        let left = dbg!(walk(
            left_sl,
            VectorDirection::Left,
            (rownum, colnum),
            &nones
        ));
        let down = dbg!(walk(
            down_sl,
            VectorDirection::Down,
            (rownum, colnum),
            &nones
        ));
        let up = dbg!(walk(up_sl, VectorDirection::Up, (rownum, colnum), &nones));
        let right = dbg!(walk(
            right_sl,
            VectorDirection::Right,
            (rownum, colnum),
            &nones
        ));
        let present: Vec<&f64> = dbg!(vec![up, down, left, right].into_iter().flatten().collect());

        let new_value = dbg!(vec_average(present));
        let rounded = ((new_value * 1_000_000_f64).round() / 1_000_000_f64) as f64;
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

        // Value found, time to write it in
        array[[rownum, colnum]] = Some(rounded);
    }

    println!("Repaired array:");
    println!("{:?}", array);
    array
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn helper_read_csv(filename: &String) -> Array2<String> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(filename)
            .expect("Couldn't read that file!");

        reader
            .deserialize_array2_dynamic()
            .expect("Inconsistent array dimensions")
    }

    fn helper_test1_array() -> (String, Array2<Option<f64>>) {
        let test1_array = arr2(&[
            [
                Some(1_f64),
                None,
                Some(2_f64),
                Some(7_f64),
                Some(9_f64),
                Some(12_f64),
                None,
            ],
            [
                None,
                None,
                Some(4_f64),
                None,
                Some(6_f64),
                Some(9_f64),
                Some(0_f64),
            ],
            [
                Some(7_f64),
                Some(4_f64),
                Some(9_f64),
                Some(1_f64),
                Some(5_f64),
                Some(0_f64),
                Some(6_f64),
            ],
            [
                Some(10_f64),
                Some(1_f64),
                None,
                Some(7_f64),
                Some(4_f64),
                Some(3_f64),
                Some(8_f64),
            ],
            [
                None,
                Some(9_f64),
                Some(4_f64),
                Some(2_f64),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some(6_f64),
                Some(4_f64),
                None,
                Some(2_f64),
                Some(1_f64),
            ],
            [
                None,
                Some(2_f64),
                Some(2_f64),
                Some(3_f64),
                Some(4_f64),
                Some(5_f64),
                Some(6_f64),
            ],
        ]);

        let test1_array_filename = String::from("tests/test_data/test1.csv");
        (test1_array_filename, test1_array)
    }

    fn helper_test1_nones() -> LinkedHashSet<(usize, usize)> {
        LinkedHashSet::from_iter(
            vec![
                (0, 1),
                (0, 6),
                (1, 0),
                (1, 1),
                (1, 3),
                (3, 2),
                (4, 0),
                (4, 4),
                (4, 5),
                (4, 6),
                (5, 0),
                (5, 1),
                (5, 4),
                (6, 0),
            ]
            .into_iter(),
        )
    }
    mod test_none_encoding {
        use super::*;
        #[test]
        fn test_parse_none_encoding() -> Result<(), Box<dyn Error>> {
            let none_encoding = String::from("foo");
            let mut from = String::from("foo");
            self::assert_eq!(parse_none_encoding::<f64>(&from, &none_encoding)?, None);

            from = String::from("0.1");
            self::assert_eq!(
                parse_none_encoding::<f64>(&from, &none_encoding)?,
                Some(0.1_f64)
            );

            // Test we can parse other types with the trait bounds
            // satisfied
            from = String::from("2");
            self::assert_eq!(
                parse_none_encoding::<usize>(&from, &none_encoding)?,
                Some(2)
            );

            Ok(())
        }

        #[test]
        #[should_panic]
        fn test_parse_none_encoding_panics() {
            let none_encoding = String::from("foo");
            let from = String::from("something else");
            parse_none_encoding::<f64>(&from, &none_encoding).unwrap();
        }
    }

    mod test_csv {
        use super::*;

        #[test]
        fn test_parse_csv_to_array() {
            let (test_array_filename, expected_array) = helper_test1_array();
            let none_encoding = String::from("nan");
            let parsed = parse_csv_to_array::<f64>(&test_array_filename, &none_encoding);

            self::assert_eq!(expected_array, parsed);
        }

        #[test]
        #[should_panic(expected = "Couldn't read file: tests/test_data/nonexistent.csv")]
        fn test_missing_file() {
            let test_array_filename = String::from("tests/test_data/nonexistent.csv");
            let none_encoding = String::from("nan");
            parse_csv_to_array::<f64>(&test_array_filename, &none_encoding);
        }

        #[test]
        #[should_panic(expected = "Inconsistent array dimensions")]
        fn test_inconsistent_array_dimensions() {
            let none_encoding = String::from("nan");
            let test_array_filename =
                String::from("tests/test_data/test_inconsistent_dimensions.csv");
            parse_csv_to_array::<f64>(&test_array_filename, &none_encoding);
        }

        #[test]
        #[should_panic(expected = "Couldn't parse: BAD")]
        fn test_invalid_input() {
            let none_encoding = String::from("nan");
            let test_array_filename = String::from("tests/test_data/test_invalid_input.csv");
            parse_csv_to_array::<f64>(&test_array_filename, &none_encoding);
        }
    }

    mod test_array_repair {
        use super::*;
        use pretty_assertions::assert_eq;
        use std::fs;

        fn helper_test1_expected_repaired_array() -> Array2<Option<f64>> {
            arr2(&[
                [
                    Some(1_f64),
                    Some(2.333333_f64),
                    Some(2_f64),
                    Some(7_f64),
                    Some(9_f64),
                    Some(12_f64),
                    Some(6_f64),
                ],
                [
                    Some(4_f64),
                    Some(3.583333_f64),
                    Some(4_f64),
                    Some(4.5_f64),
                    Some(6_f64),
                    Some(9_f64),
                    Some(0_f64),
                ],
                [
                    Some(7_f64),
                    Some(4_f64),
                    Some(9_f64),
                    Some(1_f64),
                    Some(5_f64),
                    Some(0_f64),
                    Some(6_f64),
                ],
                [
                    Some(10_f64),
                    Some(1_f64),
                    Some(5.25_f64),
                    Some(7_f64),
                    Some(4_f64),
                    Some(3_f64),
                    Some(8_f64),
                ],
                [
                    Some(9.5_f64),
                    Some(9_f64),
                    Some(4_f64),
                    Some(2_f64),
                    Some(3.333333_f64),
                    Some(2.777778_f64),
                    Some(3.925926_f64),
                ],
                [
                    Some(7.75_f64),
                    Some(6.1875_f64),
                    Some(6_f64),
                    Some(4_f64),
                    Some(3.333333_f64),
                    Some(2_f64),
                    Some(1_f64),
                ],
                [
                    Some(4.875_f64),
                    Some(2_f64),
                    Some(2_f64),
                    Some(3_f64),
                    Some(4_f64),
                    Some(5_f64),
                    Some(6_f64),
                ],
            ])
        }

        #[test]
        fn test_find_nones() {
            let expected_nones = helper_test1_nones();
            let (_, input_array) = helper_test1_array();
            let nones = find_nones(&input_array);
            self::assert_eq!(expected_nones, nones);
        }

        #[test]
        fn test_no_nones_found() {
            let expected_nones: LinkedHashSet<(usize, usize)> = LinkedHashSet::new();
            let input_array = arr2(&[[Some(1_f64)], [Some(2_f64)], [Some(3_f64)]]);
            let nones = find_nones(&input_array);

            self::assert_eq!(expected_nones, nones);
        }

        #[test]
        fn test_repair_array_inplace() {
            let (_, test_array) = helper_test1_array();
            let nones = helper_test1_nones();
            let repaired = repair_array_inplace(nones, test_array);

            self::assert_eq!(repaired, helper_test1_expected_repaired_array());
        }

        #[test]
        fn test_array_write() {
            let test_array = arr2(&[[Some(1_f64), Some(2_f64)], [Some(3_f64), Some(4_f64)]]);
            let expected_array = arr2(&[
                [String::from("1.0"), String::from("2.0")],
                [String::from("3.0"), String::from("4.0")],
            ]);
            let test_filename = String::from("tests/test_array_write.csv");

            match write_array_to_csv(&test_array, &test_filename) {
                Ok(_) => {
                    let actual_array = helper_read_csv(&test_filename);

                    self::assert_eq!(actual_array, expected_array);
                    if let Ok(_) = fs::remove_file(&test_filename) {
                        ()
                    } else {
                        panic!("Couldn't remove file {}", test_filename);
                    }
                }
                Err(_) => {
                    panic!("Couldn't write to file {}", test_filename);
                }
            };
        }
    }
}
