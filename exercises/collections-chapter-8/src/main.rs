use std::{collections::HashMap, io};

fn main() {
    let mut numbers = vec![5, 2, -4, 8, 1, 5, 2, 2];

    numbers.sort();

    println!("Sorted vector (asecnding): {:?}", numbers);

    println!("median: {}", find_median(&numbers));

    println!("mode: {}", find_mode(&numbers));

    let english = "Hey how are you doing";

    println!("{english} converted to pig latin: {}", convert_to_pig_latin(english));

    manage_company();
}

// Exercise 3
fn manage_company() {
    let mut company_directory: HashMap<String, Vec<String>>  = HashMap::new();

    println!("Welcome to managing your company!");
    loop {
        let mut input_text = String::new();
        display_options();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");

        match input_text.trim() {
            "1" => {
                print_existing_companies(&company_directory);
            }
            "2" => {
                println!("{company_directory:#?}");
            }
            "3" => {
                add_new_department(&mut company_directory);
            }
            "4" => {
                add_new_employee(&mut company_directory);
            }
            "5" => {
                break;
            }
            _ => {
                println!("Invalid input");
                continue;
            }
        }
    }
}

fn display_options() {
    println!("Select a control:");
    println!("(1): View departments");
    println!("(2): View company");
    println!("(3): Create department");
    println!("(4): Add employee");
    println!("(5): Exit");
}

fn print_existing_companies(company_directory: &HashMap<String, Vec<String>>) {
    println!("Existing departments:");
    for (key, _) in company_directory {
        println!("{key}");
    }
}

fn add_new_department(company_directory: &mut HashMap<String, Vec<String>>) {
    let mut new_department = String::new();
    println!("Input department name:");
    io::stdin()
        .read_line(&mut new_department)
        .expect("Failed to read line");

    let new_department = new_department.trim().to_string();
    company_directory.entry(new_department).or_insert(vec![]);
}

fn add_new_employee(company_directory: &mut HashMap<String, Vec<String>>) {
    let mut department = String::new();
    let mut new_employee = String::new();

    println!("Enter new employee's name:");

    io::stdin()
        .read_line(&mut new_employee)
        .expect("Failed to read line");

    let new_employee = new_employee.trim().to_string();


    println!("Enter new or existing department:");

    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");

    let department = department.trim().to_string();
    let found_directory = company_directory.entry(department).or_insert(vec![]);

    found_directory.push(new_employee);
}

// Exercise 1
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

// Exercise 2
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