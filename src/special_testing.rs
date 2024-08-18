use std::collections::{BTreeSet, HashSet};

use rand::Rng;

const STR_SIZE: usize = 8;
const ARRAY_SIZE: usize = 1_000_000;
const LOOP_COUNT: usize = 3;

pub(crate) fn prepare_str_array() -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut array: Vec<String> = Vec::with_capacity(ARRAY_SIZE);
    for _ in 0..ARRAY_SIZE {
        let mut s = String::with_capacity(STR_SIZE);
        for _ in 0..STR_SIZE {
            let c = rng.gen_range('a'..='z');
            s.push(c);
        }
        array.push(s);
    }
    array
}

pub(crate) fn prepare_str_array_with_duplication() -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut array: Vec<String> = Vec::with_capacity(ARRAY_SIZE);
    for _ in 0..ARRAY_SIZE {
        if array.len() == 0 || rng.gen_bool(0.5) {
            let mut s = String::with_capacity(STR_SIZE);
            for _ in 0..STR_SIZE {
                let c = rng.gen_range('a'..='z');
                s.push(c);
            }
            array.push(s);
        } else {
            let index = rng.gen_range(0..array.len());
            let s = array[index].clone();
            array.push(s);
        }
    }
    array
}

pub(crate) fn prepare_int_array() -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut array: Vec<usize> = Vec::with_capacity(ARRAY_SIZE);
    for _ in 0..ARRAY_SIZE {
        array.push(rng.gen());
    }
    array
}

fn test_strings(array: &[String]) {
    for _ in 0..LOOP_COUNT {
        // adjust the loop time manually, so that the total time is about 1 second
        let t = std::time::Instant::now();
        let hashset: HashSet<String> = array.iter().map(|s| s.to_string()).collect();
        // By checking result you make sure that compiler doesn't optimize it away,
        // and also ensure that your optimizations don't break semantics
        assert!(hashset.len() <= array.len());
        // Print time with two digit after ., to avoid excessive precision
        eprintln!("indirect {:0.2?}", t.elapsed());
    }
    for _ in 0..LOOP_COUNT {
        let t = std::time::Instant::now();
        let mut hashset: HashSet<String> = HashSet::new();
        for item in array {
            hashset.insert(item.to_string());
        }
        assert!(hashset.len() <= array.len());
        eprintln!("direct {:0.2?}", t.elapsed());
    }
    for _ in 0..LOOP_COUNT {
        let t = std::time::Instant::now();
        let mut array2: Vec<String> = vec![];
        for item in array {
            array2.push(item.to_string());
        }
        array2.pop();
        let hashset: HashSet<String> = array2.iter().map(|s| s.to_string()).collect();

        assert!(hashset.len() <= array.len());
        eprintln!("copy and indirect {:0.2?}", t.elapsed());
    }
    for _ in 0..LOOP_COUNT {
        let t = std::time::Instant::now();
        let mut hashset: HashSet<String> = HashSet::new();
        let mut array2: Vec<String> = vec![];
        for item in array {
            array2.push(item.to_string());
        }
        array2.pop();
        for item in &array2 {
            hashset.insert(item.to_string());
        }
        assert!(hashset.len() <= array2.len());
        eprintln!("copy and direct {:0.2?}", t.elapsed());
    }
    for _ in 0..LOOP_COUNT {
        let t = std::time::Instant::now();
        let mut array2: BTreeSet<String> = array.iter().map(|s| s.to_string()).collect();
        array2.pop_last();
        let hashset: HashSet<String> = array2.iter().map(|s| s.to_string()).collect();
        assert!(hashset.len() <= array2.len());
        eprintln!("copy btree and indirect {:0.2?}", t.elapsed());
    }
}

pub fn urusura() {
    eprintln!("string");
    test_strings(&prepare_str_array());
    eprintln!("duplicated string");
    test_strings(&prepare_str_array_with_duplication());

    eprintln!("usize");
    let array = prepare_int_array();
    for _ in 0..LOOP_COUNT {
        // adjust the loop time manually, so that the total time is about 1 second
        let t = std::time::Instant::now();
        let hashset: HashSet<usize> = array.iter().map(|s| *s).collect();
        // By checking result you make sure that compiler doesn't optimize it away,
        // and also ensure that your optimizations don't break semantics
        assert!(hashset.len() <= array.len());
        // Print time with two digit after ., to avoid excessive precision
        eprintln!("indirect {:0.2?}", t.elapsed());
    }
    for _ in 0..LOOP_COUNT {
        let t = std::time::Instant::now();
        let mut hashset: HashSet<usize> = HashSet::new();
        for item in &array {
            hashset.insert(*item);
        }
        assert!(hashset.len() <= array.len());
        eprintln!("direct {:0.2?}", t.elapsed());
    }
    for _ in 0..LOOP_COUNT {
        let t = std::time::Instant::now();

        let mut array2: Vec<usize> = vec![];
        for item in &array {
            array2.push(*item);
        }
        array2.pop();
        let hashset: HashSet<usize> = array2.iter().map(|a| *a).collect();
        assert!(hashset.len() <= array2.len());
        eprintln!("copy and indirect {:0.2?}", t.elapsed());
    }

    for _ in 0..LOOP_COUNT {
        let t = std::time::Instant::now();

        let btree_set: BTreeSet<usize> = array.iter().map(|i| *i).collect();
        let hashset: HashSet<usize> = btree_set.iter().map(|a| *a).collect();
        assert!(hashset.len() == btree_set.len());
        eprintln!("copy btree and indirect {:0.2?}", t.elapsed());
    }
}

pub fn urusura2() {}
