use core::f32::consts::FRAC_1_SQRT_2;

use rask_engine::math::{Vec2, EPSILON};

#[test]
fn test_add_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let c = Vec2::new(-2.0, 10.0);

    assert_eq!(a + b, c);
}

#[test]
fn test_add_assign_vec2() {
    let mut a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let c = Vec2::new(-2.0, 10.0);
    a += b;

    assert_eq!(a, c);
}

#[test]
fn test_sub_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let c = Vec2::new(4.0, 5.0);

    assert_eq!(a - b, c);
}

#[test]
fn test_sub_assign_vec2() {
    let mut a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let c = Vec2::new(4.0, 5.0);
    a -= b;

    assert_eq!(a, c);
}

#[test]
fn test_neg_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-1.0, -7.5);

    assert_eq!(-a, b);
}

#[test]
fn test_mul_f32() {
    let a = Vec2::new(3.9, -4.2);

    assert_eq!(a * 2.0, 2.0 * a);
    assert_eq!(a * 2.0, Vec2::new(7.8, -8.4));
}

#[test]
fn test_mul_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-4.2, 2.0);

    assert_eq!(a * b, Vec2::new(-4.2, 15.0));
}

#[test]
fn test_div_f32() {
    let a = Vec2::new(3.0, -6.2);

    assert_eq!(a / 2.0, Vec2::new(1.5, -3.1));
}

#[test]
fn test_div_vec2() {
    let a = Vec2::new(-4.2, 7.5);
    let b = Vec2::new(1.0, 2.0);

    assert_eq!(a / b, Vec2::new(-4.2, 3.75));
}

#[test]
fn test_norm_vec2() {
    let a = Vec2::new(3.0, 4.0);

    assert!(f32::abs(a.norm() - 5.0) < EPSILON);
}

#[test]
fn test_norm2_vec2() {
    let a = Vec2::new(1.0, 2.0);

    assert!(f32::abs(a.norm2() - 5.0) < EPSILON);
}

#[test]
fn test_dot() {
    let a = Vec2::new(3.0, -6.0);
    let b = Vec2::new(1.0, 7.0);

    assert!(f32::abs(a.dot(b) - -39.0) < EPSILON);
}

#[test]
fn test_normalized_vec2() {
    let a = Vec2::new(2.0, -2.0);
    let b = Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2);

    assert_eq!(a.normalized(), b);
}
