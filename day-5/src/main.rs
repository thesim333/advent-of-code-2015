use regex::Regex;
use once_cell::sync::Lazy;
use std::time::Instant;
use std::fs;

static VOWELS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[aeiou]").unwrap());
static BAD_MATCHES: Lazy<Regex> = Lazy::new(|| Regex::new(r"ab|cd|pq|xy").unwrap());

fn main() {
    let start = Instant::now();
    let duration = start.elapsed();

    let list = fs::read_to_string("list.txt")
        .expect("Should have been able to read list.txt");

    let mut count = 0;

    for string in list.lines() {
        if is_nice(string) {
            count += 1;
        }
    }

    println!("{} nice strings were found", count);
    println!("Duration: {:?}", duration);
}

fn is_nice(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let doubles = chars.windows(2).any(|w| w[0] == w[1]);
    let triple_vowels = VOWELS.find_iter(input).count() >= 3;

    doubles && triple_vowels && !BAD_MATCHES.is_match(input)
}

#[cfg(test)]
mod tests {
    use super::is_nice;

    #[test]
    fn test_is_nice_ex1() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
    }

    #[test]
    fn test_is_nice_ex2() {
        assert_eq!(is_nice("aaa"), true);
    }

    #[test]
    fn test_is_nice_ex3() {
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
    }

    #[test]
    fn test_is_nice_ex4() {
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
    }

    #[test]
    fn test_is_nice_ex5() {
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }
}