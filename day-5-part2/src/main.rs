use std::collections::HashMap;
use std::fs;
use std::time::Instant;

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
    let bytes = input.as_bytes();
    let mut seen: HashMap<[u8; 2], usize> = HashMap::new();

    let mut contains_two_matches = false;
    let mut has_xyx_match = false;

    for i in 0..bytes.len().saturating_sub(1) {
        if !contains_two_matches {
            let pair = [bytes[i], bytes[i + 1]];

            if let Some(&prev_i) = seen.get(&pair) {
                if i >= prev_i + 2 {
                    contains_two_matches = true;
                }
            } else {
                seen.insert(pair, i);
            }
        }

        if !has_xyx_match && i < bytes.len() - 2 {
            if bytes[i] == bytes[i + 2] {
                has_xyx_match = true;
            }
        }

        if has_xyx_match && contains_two_matches {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::is_nice;

    #[test]
    fn test_is_nice_ex1() {
        assert_eq!(is_nice("qjhvhtzxzqqjkmpb"), true);
    }

    #[test]
    fn test_is_nice_ex2() {
        assert_eq!(is_nice("xxyxx"), true);
    }

    #[test]
    fn test_is_nice_ex3() {
        assert_eq!(is_nice("juurcxstgmygtbstg"), false);
    }

    #[test]
    fn test_is_nice_ex4() {
        assert_eq!(is_nice("ieodomkazucvgmuy"), false);
    }
}
