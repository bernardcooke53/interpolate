use interpolation;

#[test]
fn test_average() {
    let v = vec![];
    assert_eq!(interpolation::average(v), 0.);

    let v2 = vec![1., 2., 3., 4.];
    assert_eq!(interpolation::average(v2), 2.5);
}
