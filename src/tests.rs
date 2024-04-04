use generator::LocalGenerator;
use rand::Rng;
use std::ops::Range;

use super::*;

const VALUE_RANGE: Range<Value> = 0..100;
const LENGTH_RANGE: Range<usize> = 0..100;
const REPEAT_EACH: usize = 10;

fn random_data(rng: &mut impl Rng, length: usize) -> Vec<Value> {
    let mut list = Vec::with_capacity(length);
    for _ in 0..length {
        list.push(rng.gen_range(VALUE_RANGE));
    }
    list
}

fn test_algorithm<F>(algorithm: F)
where
    F: Fn(ListRef) -> LocalGenerator<'static, (), Compare>,
{
    let mut rng = rand::thread_rng();

    for length in LENGTH_RANGE {
        for round in 0..REPEAT_EACH {
            let mut list = random_data(&mut rng, length);

            let ptr = ListRef::from(&mut list);
            let iter = algorithm(ptr);

            // Consumes iterator
            let yield_count = iter.count();

            assert!(
                is_sorted(&list),
                "--- Not sorted! ---\n  * {}\n  * LENGTH = {}\n  * ROUND = {}\n  * YIELD_COUNT = {}",
                std::any::type_name_of_val(&algorithm),
                length,
                round,
                yield_count,
            );
        }
    }
}

#[test]
fn bubble() {
    test_algorithm(sorts::bubble);
}
#[test]
fn insertion() {
    test_algorithm(sorts::insertion);
}
#[test]
fn selection() {
    test_algorithm(sorts::selection);
}
#[test]
fn quick() {
    test_algorithm(sorts::quick);
}
