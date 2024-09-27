use crossbeam::epoch;
use flurry::HashMap;

#[test]
fn new() {
    let _map = HashMap::<usize, usize>::new();
}

#[test]
fn insert() {
    let mut map = HashMap::new();
    let old = map.insert(42, 0);
    assert_eq!(old, None);
}