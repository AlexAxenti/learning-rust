// Exercise 2
pub fn convert_to_pig_latin(s: &str) -> String {
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