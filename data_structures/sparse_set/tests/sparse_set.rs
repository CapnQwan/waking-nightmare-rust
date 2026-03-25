use std::collections::HashSet;

use sparse_set::sparse_set::SparseSet;

#[test]
fn get_on_empty_returns_none() {
  let set: SparseSet<i32> = SparseSet::new();
  assert_eq!(set.get(0), None);
  assert_eq!(set.get(123), None);
}

#[test]
fn insert_and_get_returns_value() {
  let mut set = SparseSet::new();

  set.insert(2, "b");
  set.insert(0, "a");

  assert_eq!(set.get(0), Some(&"a"));
  assert_eq!(set.get(2), Some(&"b"));
  assert_eq!(set.get(1), None);
}

#[test]
fn get_mut_allows_in_place_mutation() {
  let mut set = SparseSet::new();
  set.insert(7, 10);

  *set.get_mut(7).expect("value should exist") += 5;

  assert_eq!(set.get(7), Some(&15));
  assert_eq!(set.get_mut(999), None);
}

#[test]
fn extract_returns_value_and_removes_entry() {
  let mut set = SparseSet::new();
  set.insert(3, 123);

  assert_eq!(set.extract(3), Some(123));
  assert_eq!(set.get(3), None);

  // Removing again should be a no-op.
  assert_eq!(set.extract(3), None);
}

#[test]
fn extract_is_swap_remove_and_keeps_other_entries_accessible() {
  let mut set = SparseSet::new();

  // Use sparse indices that make it easy to verify mapping survives a swap.
  set.insert(10, "a");
  set.insert(20, "b");
  set.insert(30, "c");

  // Removing "b" may swap "c" into its dense slot internally.
  assert_eq!(set.extract(20), Some("b"));

  assert_eq!(set.get(20), None);
  assert_eq!(set.get(10), Some(&"a"));
  assert_eq!(set.get(30), Some(&"c"));

  // Removing the remaining values should still work.
  let mut removed = HashSet::new();
  removed.insert(set.extract(10));
  removed.insert(set.extract(30));

  assert!(removed.contains(&Some("a")));
  assert!(removed.contains(&Some("c")));
}

#[test]
fn iter_yields_all_values_currently_stored() {
  let mut set = SparseSet::new();
  set.insert(5, 1);
  set.insert(1, 2);
  set.insert(9, 3);

  let got: HashSet<i32> = set.iter().copied().collect();
  let expected: HashSet<i32> = [1, 2, 3].into_iter().collect();
  assert_eq!(got, expected);

  // After an extract, iteration should reflect removal.
  set.extract(1);
  let got: HashSet<i32> = set.iter().copied().collect();
  let expected: HashSet<i32> = [1, 3].into_iter().collect();
  assert_eq!(got, expected);
}

#[test]
fn into_iter_consumes_and_yields_values() {
  let mut set = SparseSet::new();
  set.insert(100, 10);
  set.insert(200, 20);

  let got: HashSet<i32> = set.into_iter().collect();
  let expected: HashSet<i32> = [10, 20].into_iter().collect();
  assert_eq!(got, expected);
}

#[test]
fn clear_removes_everything() {
  let mut set = SparseSet::new();
  set.insert(0, 1);
  set.insert(5, 2);

  set.clear();

  assert_eq!(set.get(0), None);
  assert_eq!(set.get(5), None);
  assert_eq!(set.extract(0), None);
  assert_eq!(set.iter().count(), 0);
}

#[test]
fn replace_on_existing_entry_returns_old_value_and_updates() {
  let mut set = SparseSet::new();
  set.insert(4, "old");

  let prev = set.replace(4, "new");
  assert_eq!(prev, Some("old"));
  assert_eq!(set.get(4), Some(&"new"));
}

#[test]
fn replace_out_of_bounds_inserts_and_returns_none() {
  let mut set = SparseSet::new();

  let prev = set.replace(42, "x");
  assert_eq!(prev, None);
  assert_eq!(set.get(42), Some(&"x"));
}

#[test]
fn replace_on_in_bounds_hole_returns_none_and_does_not_insert() {
  let mut set = SparseSet::new();

  // Create an in-bounds hole at index 1:
  set.insert(1, "temp");
  set.insert(10, "other");
  assert_eq!(set.extract(1), Some("temp"));
  assert_eq!(set.get(1), None);

  // Current behavior: replace() on an in-bounds None returns None and does not insert.
  let prev = set.replace(1, "new");
  assert_eq!(prev, None);
  assert_eq!(set.get(1), None);

  // But existing entries must remain intact.
  assert_eq!(set.get(10), Some(&"other"));
}