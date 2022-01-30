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

    let break_condition = |item: &(usize, &Option<f64>)| {
        let index = dbg!(item.0);
        let val = item.1;
        let search_idx = make_nan_search_index(index);

        dbg!(&search_idx);
        if dbg!(!nones.contains(&search_idx)) {
            match val {
                Some(_) => return true,
                None => return false,
            }
        }
        false
    };

    let res = vec.into_iter().enumerate().find(break_condition);

    match res {
        Some(v) => {
            // If the predicate was met then we know it's
            // safe to unwrap as v.1 definitely has Some(val)
            return Some(v.1.as_ref().unwrap());
        }
        None => None,
    }
}
