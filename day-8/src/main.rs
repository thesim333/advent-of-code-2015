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

fn main() {
    let list = fs::read_to_string("list.txt")
        .expect("Should have been able to read list.txt");

    let mut string_lit = 0;
    let mut chars = 0;

    for line in list.lines() {
        chars += line.len();

        let _line = &line[1..line.len() - 1];
        let decoded = decode(_line);
        string_lit += decoded.len();
    }

    let answer = chars - string_lit;

    println!("{}", answer);
}
