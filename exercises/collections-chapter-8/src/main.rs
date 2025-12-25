use std::collections::HashMap;

fn main() {
    let mut numbers = vec![5, 2, -4, 8, 1, 5, 2, 2];

    numbers.sort();

    println!("Sorted vector (asecnding): {:?}", numbers);

    println!("median: {}", find_median(&numbers));

    println!("mode: {}", find_mode(&numbers));

    let english = "Hey how are you doing";

    println!("{english} converted to pig latin: {}", convert_to_pig_latin(english));
}


fn find_median(list: &[i32]) -> i32 {
    let length = list.len();

    let median_index = length / 2;

    println!("median index is {median_index}");

    list[median_index]
}

fn find_mode(list: &[i32]) -> i32 {
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

fn convert_to_pig_latin(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut pig_latin_s = String::new();

    for word in s.split_whitespace() {
        let mut chars = word.chars();

        let first_char = match chars.next() {
            Some(c) => c,
            None => continue,
        };

        let rest = chars.collect::<String>();
        if vowels.contains(&first_char) {
        pig_latin_s = pig_latin_s + word + "-hay ";
        } else {
            pig_latin_s = pig_latin_s + &rest + "-" + &first_char.to_string() + "ay "
        }
    }

    pig_latin_s
}