fn is_adult(age: i32) -> bool {
    age >= 18
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let age: i32 = 25;
    let height: f64 = 1.75;
    let is_student: bool = true;
    let initial: char = 'A';

    println!("Age: {}", age);
    println!("Height: {}", height);
    println!("Is student: {}", is_student);
    println!("Initial: {}", initial);
    println!("Is adult: {}", is_adult(age));
    println!();

    let mut score: i32 = 10;
    println!("Initial score: {}", score);

    score += 5;
    println!("Updated score: {}", score);

    let result = add(10, 20);
    println!("10 + 20 = {}", result);

    // if assignemnt to values
    let number = 7;
    let kind = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };

    println!("Number {} is {}", number, kind);
}
