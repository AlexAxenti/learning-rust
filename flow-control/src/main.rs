fn print_numbers_up_to(n: i32) {
    for i in 1..=n {
        println!("{}", i);
    }
}

fn print_chars(s: &str) {
    for c in s.chars() {
        println!("{}", c);
    }
}

fn count_chars(s: &str) -> usize {
    let mut count = 0;

    for _ in s.chars() {
        count += 1;
    }

    count
}

fn count_vowels(s: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;
    
    for c in s.chars() {
        if vowels.contains(&c) {
            count += 1;
        }
    }

    count
}


fn main() {
    let mut count = 3;

    while count > 0 {
        println!("Countdown: {}", count);
        count -= 1;
    }

    print_numbers_up_to(5);

    let text = "hello";

    print_chars(text);
    println!("{}", count_chars(text));

    println!("{}", count_vowels(text));
}
