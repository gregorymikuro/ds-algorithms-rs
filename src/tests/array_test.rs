use ds_algorithms_rs::Array;

/// Tests vector creation.
#[test]
fn creates_vector() {
    let vector = Array::vector(vec![1.0, 2.0, 3.0]);

    assert_eq!(vector.shape(), &[3]);
    assert_eq!(vector.data(), &[1.0, 2.0, 3.0]);
}

/// Tests matrix creation.
#[test]
fn creates_matrix() {
    let matrix = Array::matrix(
        vec![1.0, 2.0, 3.0, 4.0],
        2,
        2,
    );

    assert_eq!(matrix.shape(), &[2, 2]);
}

/// Tests tensor creation.
#[test]
fn creates_tensor() {
    let tensor = Array::tensor(
        vec![1.0, 2.0, 3.0, 4.0],
        vec![2, 2],
    );

    assert_eq!(tensor.shape(), &[2, 2]);
}

/// Tests that an invalid shape is rejected.
#[test]
#[should_panic]
fn rejects_invalid_shape() {
    Array::matrix(vec![1.0, 2.0, 3.0], 2, 2);
}

/// Tests the array length.
#[test]
fn returns_array_length() {
    let vector = Array::vector(vec![1, 2, 3]);

    assert_eq!(vector.len(), 3);
}

/// Tests whether an array is empty.
#[test]
fn identifies_empty_array() {
    let vector: Array<i32> = Array::vector(vec![]);

    assert!(vector.is_empty());
}