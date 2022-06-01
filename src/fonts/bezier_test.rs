use super::*;

#[test]
fn test_binomial_coeffs() {
    assert_eq!(binomial_coeffs(0), vec![1.0]);
    assert_eq!(binomial_coeffs(1), vec![1.0, 1.0]);
    assert_eq!(binomial_coeffs(2), vec![1.0, 2.0, 1.0]);
    assert_eq!(binomial_coeffs(3), vec![1.0, 3.0, 3.0, 1.0]);
    assert_eq!(binomial_coeffs(4), vec![1.0, 4.0, 6.0, 4.0, 1.0]);
    assert_eq!(binomial_coeffs(5), vec![1.0, 5.0, 10.0, 10.0, 5.0, 1.0]);
    assert_eq!(binomial_coeffs(6), vec![1.0, 6.0, 15.0, 20.0, 15.0, 6.0, 1.0]);
}

#[test]
fn test_matrix3() {
    use cgmath::Matrix;

    let m1: cgmath::Matrix3<f32> =
        [[1.0, 0.0, 0.0], [-2.0, 2.0, 0.0], [1.0, -2.0, 1.0]].into();
    let m2: cgmath::Matrix3<f32> = cgmath::Matrix3::from_cols(
        [1.0, 0.0, 0.0].into(),
        [-2.0, 2.0, 0.0].into(),
        [1.0, -2.0, 1.0].into(),
    );
    assert_eq!(m1, m2);
}
