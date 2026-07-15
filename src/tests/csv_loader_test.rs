use ds_algorithms_rs::load_csv;

/// Tests loading numeric CSV data.
#[test]
fn loads_csv() {
    let array = load_csv("tests/data/numbers.csv").unwrap();

    assert_eq!(array.shape(), &[2, 3]);
    assert_eq!(
        array.data(),
        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
    );
}