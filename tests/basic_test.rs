use minecraft_mth::{self as mth, wrap_degrees};

#[test]
fn positive_modulo_test() {
    assert_eq!(mth::positive_modulo(-5, 3), 1);
    assert_eq!(mth::positive_modulo(5, 3), 2);
    assert!(mth::equal(mth::positive_modulo(-5.5f64, 3.0), 0.5))
}

#[test]
fn wrap_degrees_test() {
    assert_eq!(mth::wrap_degrees(0), 0);
    assert_eq!(mth::wrap_degrees(180), -180);
    assert_eq!(mth::wrap_degrees(-180), -180);
    assert_eq!(mth::wrap_degrees(181), -179);

    assert_eq!(mth::wrap_degrees(720), 0);
    assert_eq!(mth::wrap_degrees(10_000i64), 10_000 % 360 - 360);

    assert!(mth::equal(wrap_degrees(180.5), -179.5));
    assert!(mth::equal(wrap_degrees(-180.5), 179.5));
}
