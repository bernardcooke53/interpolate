use linked_hash_set::LinkedHashSet;

/// Given a vec of numbers, Calculate the average of the numbers in the vec
pub fn average(v: Vec<&f64>) -> f64 {
    if v.len() == 0 {
        0.0_f64
    } else {
        v.clone().into_iter().sum::<f64>() / (v.len() as f64)
    }
}

pub enum VectorDirection {
    Left,
    Down,
    Up,
    Right,
}

pub fn walk<'a>(
    vec: Vec<&'a Option<f64>>,
    vec_direc: VectorDirection,
    start: (usize, usize),
    nones: &LinkedHashSet<(usize, usize)>,
) -> Option<&'a f64> {
    let (start_row, start_col) = start;
    let make_nan_search_index = |k| match vec_direc {
        VectorDirection::Left => dbg!((start_row, start_col - k)),
        VectorDirection::Down => dbg!((start_row + k, start_col)),
        VectorDirection::Up => dbg!((start_row - k, start_col)),
        VectorDirection::Right => dbg!((start_row, start_col + k)),
    };

    let break_condition = |&(index, val): &(usize, &Option<f64>)| {
        let search_idx = make_nan_search_index(index);
        dbg!(&val);
        dbg!(&search_idx);

        if !dbg!(nones.contains(&search_idx)) {
            match val {
                Some(_) => return true,
                None => return false,
            }
        }
        false
    };

    let res = vec.into_iter().enumerate().find(break_condition);

    match res {
        Some((_, v)) => {
            // If the predicate was met then we know it's
            // safe to unwrap as v definitely has Some(val)
            Some(v.as_ref().unwrap())
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_average_empty_vec() {
        let v = vec![];
        assert_eq!(average(v), 0.);
    }

    #[test]
    fn test_average() {
        let v = vec![&1., &2., &3., &4.];
        assert_eq!(average(v), 2.5);
    }

    #[test]
    fn test_walk_skips_none() {
        // Test a mock slice from a 3x3 array
        let nones: LinkedHashSet<(usize, usize)> = [(2, 1), (1, 0), (0, 2)].into_iter().collect();
        // start in the upper right hand corner
        let start = (0, 2);
        // Take the slice [(0, 2), (1, 2), (2, 2)]
        let slice = vec![&None, &Some(2.0_f64), &None];
        let direction = VectorDirection::Down;
        let next_val_down = walk(slice, direction, start, &nones);
        assert_eq!(next_val_down, Some(&2.0_f64));
    }

    #[test]
    fn test_walk_none_if_all_none() {
        // Test a mock slice from a 3x3 array
        let nones: LinkedHashSet<(usize, usize)> = [(2, 1), (1, 0), (0, 2)].into_iter().collect();
        // start in the upper right hand corner
        let start = (0, 2);
        // Take the slice [(0, 2), (1, 2), (2, 2)]
        let slice = vec![&None, &None, &None];
        let direction = VectorDirection::Down;
        let next_val_down = walk(slice, direction, start, &nones);
        assert_eq!(next_val_down, None);
    }

    #[test]
    fn test_walk_takes_first() {
        // Test a mock slice from a 3x3 array
        let nones: LinkedHashSet<(usize, usize)> = [(2, 1), (1, 0), (0, 2)].into_iter().collect();
        // start in the lower left hand corner
        let start = (2, 0);
        // Take the slice [(2, 2), (1, 0), (0, 0)]
        let slice = vec![&Some(1.0_f64), &Some(3.0_f64), &Some(2.0_f64)];
        let direction = VectorDirection::Up;
        let next_val_down = walk(slice, direction, start, &nones);
        assert_eq!(next_val_down, Some(&1.0_f64));
    }
}
