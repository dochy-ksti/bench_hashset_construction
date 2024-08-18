use std::collections::HashSet;

use rand::Rng;
use special_testing::{prepare_str_array, urusura, urusura2};

mod special_testing;
/// Basically, copy to vector and convert to hashmap/set.
/// Reallocation is quite ineffective for collections with hashes.
/// string
/// indirect 210.16ms
/// indirect 190.13ms
/// indirect 190.74ms
/// direct 423.37ms
/// direct 344.71ms
/// direct 341.09ms
/// copy and indirect 259.39ms
/// copy and indirect 263.87ms
/// copy and indirect 258.91ms
/// copy and direct 415.27ms
/// copy and direct 418.81ms
/// copy and direct 412.11ms
/// copy btree and indirect 831.95ms
/// copy btree and indirect 835.48ms
/// copy btree and indirect 825.23ms
/// duplicated string
/// indirect 246.51ms
/// indirect 241.37ms
/// indirect 244.50ms
/// direct 270.68ms
/// direct 267.45ms
/// direct 268.74ms
/// copy and indirect 316.07ms
/// copy and indirect 312.17ms
/// copy and indirect 313.08ms
/// copy and direct 338.04ms
/// copy and direct 344.18ms
/// copy and direct 337.20ms
/// copy btree and indirect 687.59ms
/// copy btree and indirect 711.10ms
/// copy btree and indirect 683.17ms
/// usize
/// indirect 69.10ms
/// indirect 72.78ms
/// indirect 72.18ms
/// direct 102.32ms
/// direct 99.43ms
/// direct 103.86ms
/// copy and indirect 85.73ms
/// copy and indirect 85.45ms
/// copy and indirect 84.63ms
/// copy btree and indirect 164.42ms
/// copy btree and indirect 212.72ms
/// copy btree and indirect 158.25ms
fn main() {
    doit();
}

fn doit() {
    urusura();
    urusura2();
}

#[test] // so that you can run this with `cargo test --release -- my_bench --nocapture`
fn my_bench() {
    doit();
}
