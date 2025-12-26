use std::{collections::HashMap, io};

// Exercise 3
pub fn manage_company() {
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