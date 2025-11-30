use std::fs;

fn decode(line: &str) -> String {
    let mut out = String::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') => out.push('\\'),
                Some('"')  => out.push('"'),
                Some('x')  => {
                    chars.next().unwrap();
                    chars.next().unwrap();
                    out.push('x'); // it's only 1 ascii char we can just put anything in
                }
                Some(other) => {
                    panic!("unexpected escape: \\{other}");
                }
                None => {}
            }
        } else {
            out.push(c);
        }
    }

    out
}

/**
 * Lazy counting of new encoding / characters for part 2
 */
fn count_encoded_line(line: &str) -> usize {
    let mut count = 6; // for wrapped "" we dropped off
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            count += 1;
        }
        if c == '"' {
            count += 1;
        }

        count += 1;
    }

    count
}

fn main() {
    let list = fs::read_to_string("list.txt")
        .expect("Should have been able to read list.txt");

    let mut string_lit = 0;
    let mut chars = 0;
    let mut encoded_count = 0;

    for line in list.lines() {
        chars += line.len();

        let _line = &line[1..line.len() - 1];
        let decoded = decode(_line);
        string_lit += decoded.len();

        let encoded_line_count = count_encoded_line(_line);
        encoded_count += encoded_line_count;
    }

    let part1 = chars - string_lit;
    let part2 = encoded_count - chars;

    println!("{}", part1);
    println!("{}", part2);
}
