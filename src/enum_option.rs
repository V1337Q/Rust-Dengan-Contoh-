use core::option::Option;
pub fn double(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x * 2),
        None => None,
    }
}

pub fn test_double() {
    assert_eq!(double(Some(2)), Some(4));
    assert_eq!(double(None), None);
}
