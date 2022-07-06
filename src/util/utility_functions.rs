use std::collections::HashSet;
use std::hash::Hash;

#[allow(dead_code)]
pub fn has_unique_elements<T>(iter: T) -> bool
where
  T: IntoIterator,
  T::Item: Eq + Hash
{
  let mut unique = HashSet::new();
  iter.into_iter().all(move |x| unique.insert(x))
}

pub fn parse_str_to_int(value: &str) -> u32
{
  return value.to_owned().parse::<u32>().unwrap();
}

#[cfg(test)]
mod utility_function_tests
{
  use crate::util::utility_functions::{has_unique_elements, parse_str_to_int};

  #[test]
  fn has_unique_elements_test_1()
  {
    let a = [1, 2, 3];
    assert!(has_unique_elements(a));

    let b = [1, 1, 1];
    assert!(!has_unique_elements(b));

    let c = ["a", "b", "c"];
    assert!(has_unique_elements(c));

    let d = [1, 2, 1];
    assert!(!has_unique_elements(d));
  }

  #[test]
  fn parse_str_to_int_test_1()
  {
    let a = parse_str_to_int("2000");
    let a_expected: u32 = 2000;
    assert_eq!(a, a_expected);

    let b = parse_str_to_int("1");
    let b_expected: u32 = 1;
    assert_eq!(b, b_expected);
  }
}
