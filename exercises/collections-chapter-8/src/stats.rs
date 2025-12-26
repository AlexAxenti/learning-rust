use std::collections::HashMap;

// Exercise 1
pub fn find_median(list: &[i32]) -> i32 {
    let length = list.len();

    let median_index = length / 2;

    println!("median index is {median_index}");

    list[median_index]
}

pub fn find_mode(list: &[i32]) -> i32 {
    let mut mode_map = HashMap::new();

    for n in list {
        let count = mode_map.entry(*n).or_insert(0);
        *count += 1;
    }

    println!("Mode Hash map: {mode_map:?}");

    let mut max_occurances: usize = 0;
    let mut max_key: i32 = 0;

    for (key, value) in &mode_map {
        if mode_map.get(key).copied().unwrap_or(0) > max_occurances { 
            max_occurances = *value;
            max_key = *key;
        }
    }

    max_key
}