mod stats;
mod pig_latin;
mod company;

use stats::{find_median, find_mode};
use pig_latin::convert_to_pig_latin;
use company::manage_company;

fn main() {
    let mut numbers = vec![5, 2, -4, 8, 1, 5, 2, 2];

    numbers.sort();

    println!("Sorted vector (ascending): {:?}", numbers);

    println!("median: {}", find_median(&numbers));

    println!("mode: {}", find_mode(&numbers));

    let english = "Hey how are you doing";

    println!("{english} converted to pig latin: {}", convert_to_pig_latin(english));

    manage_company();
}

