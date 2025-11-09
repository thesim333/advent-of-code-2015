extern crate md5;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let duration = start.elapsed();

    println!("{:?}", mine_advent_coins("iwrupvqb", "00000"));
    println!("{:?}", mine_advent_coins("iwrupvqb", "000000"));
    println!("Duration: {:?}", duration);
}

fn mine_advent_coins(input: &str, start: &str) -> u32 {
    let mut buffer = String::with_capacity(input.len() + 10);
    let mut i = 0;

    loop {
        buffer.clear();
        buffer.push_str(input);
        buffer.push_str(&i.to_string());

        let digest = md5::compute(&buffer);

        if format!("{:x}", digest).starts_with(start) {
            break;
        }

        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::mine_advent_coins;

    #[test]
    fn test_mine_advent_coins_ex1() {
        assert_eq!(mine_advent_coins("abcdef", "00000"), 609043);
    }

    #[test]
    fn test_mine_advent_coins_ex2() {
        assert_eq!(mine_advent_coins("pqrstuv", "00000"), 1048970);
    }
}
