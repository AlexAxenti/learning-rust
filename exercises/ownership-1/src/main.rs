fn main() {
    // Moving ownership
    let x = 5;
    let y = x;
    let z = x;

    println!("{y} and {z}");

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}");

    let s3 = String::from("hola");
    takes_ownership(s3);

    let num = 5;
    makes_copy(num);
    println!("{num}");

    // Borrowing
    let s4 = String::from("ahoy");

    let len = calculate_length(&s4);

    println!("Size of {s4} is {len}");

    // Mutatating references
    let mut s5 = String::from("first");
    change(&mut s5);
    println!("{s5}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" and second");
}