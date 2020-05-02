use range_pointer::BoundedValue;
use std::u16;

// #[test]
// pub fn test_new() {
//     assert_eq!(
//         BoundedValue{min: 1, max: 6, value: 1}, 
//         BoundedValue::new(1, 6, 1).unwrap()
//     );
// }

#[test]
pub fn test_increase() {
    let mut r = BoundedValue::new(1, 6, 1).unwrap();
    let a = r.increase().unwrap();
    assert_eq!(a, 2);
}

#[test]
pub fn test_increase_limit() {
    let mut r = BoundedValue::new(1, 6, 6).unwrap();
    let a = r.increase().unwrap();
    assert_eq!(a, 6);
}

#[test]
pub fn test_increase_overflow() {
    let mut r = BoundedValue::new(1, u16::MAX, u16::MAX).unwrap();
    let a = r.increase().unwrap();
    assert_eq!(a, u16::MAX);
}

#[test] 
pub fn test_increase_by() {
    let mut r = BoundedValue::new(1, 6, 1).unwrap();
    let a = r.increase_by(3).unwrap();
    assert_eq!(a, 4);
}

#[test] 
pub fn test_increase_by_limit() {
    let mut r = BoundedValue::new(1, 6, 1).unwrap();
    let a = r.increase_by(10).unwrap();
    assert_eq!(a, 6);
}

#[test] 
pub fn test_increase_by_overflow() {
    let mut r = BoundedValue::new(1, u16::MAX, u16::MAX).unwrap();
    let a = r.increase_by(10).unwrap();
    assert_eq!(a, u16::MAX);
}

#[test]
pub fn test_decrease() {
    let mut r = BoundedValue::new(1, 6, 3).unwrap();
    let a = r.decrease().unwrap();
    assert_eq!(a, 2);
}

pub fn test_decrease_limit() {
    let mut r = BoundedValue::new(1, 6, 1).unwrap();
    let a = r.decrease().unwrap();
    assert_eq!(a, 1);
}

pub fn test_decrease_overflow() {
    let mut r = BoundedValue::new(u16::MIN, 6, u16::MIN).unwrap();
    let a = r.decrease().unwrap();
    assert_eq!(a, u16::MIN);
}

#[test] 
pub fn test_decrease_by() {
    let mut r = BoundedValue::new(1, 6, 5).unwrap();
    let a = r.decrease_by(2).unwrap();
    assert_eq!(a, 3);
}

#[test] 
pub fn test_decrease_by_limit() {
    let mut r = BoundedValue::new(1, 6, 5).unwrap();
    let a = r.decrease_by(10).unwrap();
    assert_eq!(a, 1);
}

#[test] 
pub fn test_decrease_by_overflow() {
    let mut r = BoundedValue::new(u16::MIN, u16::MAX, u16::MIN).unwrap();
    let a = r.decrease_by(10).unwrap();
    assert_eq!(a, u16::MIN);
}

#[test] 
pub fn test_maximize() {
    let mut r = BoundedValue::new(1, 6, 1).unwrap();
    let a = r.maximize().unwrap();
    assert_eq!(a, 6);
}

#[test] 
pub fn test_minimize() {
    let mut r = BoundedValue::new(1, 6, 5).unwrap();
    let a = r.minimize().unwrap();
    assert_eq!(a, 1);
}