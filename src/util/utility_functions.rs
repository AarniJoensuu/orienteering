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

#[cfg(test)]
mod utility_function_tests
{
  use crate::util::utility_functions::{has_unique_elements};

  #[test]
  fn test_unique_elements()
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
}
