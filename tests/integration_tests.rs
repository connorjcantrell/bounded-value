use range_pointer::Range;
use std::u16;

#[test]
pub fn test_new() {
    assert_eq!(
        Range{min: 1, max: 6, current: 1}, 
        Range::new(1, 6, 1)
    );
}

#[test]
pub fn test_increase() {
    let r = Range{min: 1, max: 6, current: 1};
    let a = r.increase().unwrap();
    assert_eq!(a, 2);
}

#[test]
pub fn test_increase_limit() {
    let r = Range{min: 1, max: 6, current: 6};
    let a = r.increase().unwrap();
    assert_eq!(a, 6);
}

#[test]
pub fn test_increase_overflow() {
    let r = Range{min: 1, max: u16::MAX, current: u16::MAX};
    let a = r.increase().unwrap();
    assert_eq!(a, u16::MAX);
}

#[test] 
pub fn test_increase_by() {
    let r = Range{min: 1, max: 6, current: 1};
    let a = r.increase_by(3).unwrap();
    assert_eq!(a, 4);
}

#[test] 
pub fn test_increase_by_limit() {
    let r = Range{min: 1, max: 6, current: 1};
    let a = r.increase_by(10).unwrap();
    assert_eq!(a, 6);
}

#[test] 
pub fn test_increase_by_overflow() {
    let r = Range{min: 1, max: u16::MAX, current: u16::MAX};
    let a = r.increase_by(10).unwrap();
    assert_eq!(a, u16::MAX);
}

#[test]
pub fn test_decrease() {
    let r = Range{min: 1, max: 6, current: 3};
    let a = r.decrease().unwrap();
    assert_eq!(a, 2);
}

pub fn test_decrease_limit() {
    let r = Range{min: 1, max: 6, current: 1};
    let a = r.decrease().unwrap();
    assert_eq!(a, 1);
}

pub fn test_decrease_overflow() {
    let r = Range{min: u16::MIN, max: 6, current: u16::MIN};
    let a = r.decrease().unwrap();
    assert_eq!(a, u16::MIN);
}

#[test] 
pub fn test_decrease_by() {
    let r = Range{min: 1, max: 6, current:5};
    let a = r.decrease_by(2).unwrap();
    assert_eq!(a, 3);
}

#[test] 
pub fn test_decrease_by_limit() {
    let r = Range{min: 1, max: 6, current: 5};
    let a = r.decrease_by(10).unwrap();
    assert_eq!(a, 1);
}

#[test] 
pub fn test_decrease_by_overflow() {
    let r = Range{min: u16::MIN, max: u16::MAX, current: u16::MIN};
    let a = r.decrease_by(10).unwrap();
    assert_eq!(a, u16::MIN);
}

#[test] 
pub fn test_maximize() {
    let r = Range{min: 1, max: 6, current: 1};
    let a = r.maximize().unwrap();
    assert_eq!(a, 6);
}

#[test] 
pub fn test_minimize() {
    let r = Range{min: 1, max: 6, current: 5};
    let a = r.minimize().unwrap();
    assert_eq!(a, 1);
}