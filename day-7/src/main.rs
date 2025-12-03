use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Op {
    Not(String, String),
    And(String, String, String),
    Or(String, String, String),
    LShift(String, u16, String),
    RShift(String, u16, String),
    Assign(String, String),
}

fn main() {
    let list = fs::read_to_string("instructions.txt")
        .expect("Should have been able to read instructions.txt");

    println!("wire a = {:#?}", solve_for("a", &list));
}

fn parse_instruction(line: &str) -> Op {
    let parts: Vec<&str> = line.split_whitespace().collect();

    match parts.as_slice() {
        ["NOT", a, "->", out] => Op::Not((*a).into(), (*out).into()),
        [a, "OR", b, "->", out] => Op::Or((*a).into(), (*b).into(), (*out).into()),
        [a, "AND", b, "->", out] => Op::And((*a).into(), (*b).into(), (*out).into()),
        [a, "LSHIFT", n, "->", out] => Op::LShift((*a).into(), n.parse().unwrap(), (*out).into()),
        [a, "RSHIFT", n, "->", out] => Op::RShift((*a).into(), n.parse().unwrap(), (*out).into()),
        [a, "->", out] => Op::Assign((*a).into(), (*out).into()),
        _ => panic!("Unrecognized instruction: {:?}", parts),
    }
}

fn get_wire_name(op: &Op) -> &str {
    match op {
        Op::Not(_, out)
        | Op::Assign(_, out)
        | Op::And(_, _, out)
        | Op::Or(_, _, out)
        | Op::LShift(_, _, out)
        | Op::RShift(_, _, out) => out,
    }
}

fn get_value(arg: &str, processes: &HashMap<String, Op>, wires: &mut HashMap<String, u16>) -> u16 {
    if let Ok(n) = arg.parse::<u16>() {
        n
    } else if wires.contains_key(arg) {
        wires[arg]
    } else {
        evaluate(arg, processes, wires)
    }
}

fn assign_wire(wire: &str, value: u16, wires: &mut HashMap<String, u16>) -> u16 {
        wires.insert(wire.to_string(), value);
        value
}

fn evaluate(wire: &str, processes: &HashMap<String, Op>, wires: &mut HashMap<String, u16>) -> u16 {
    if let Some(&v) = wires.get(wire) {
        return v;
    }

    let op = processes.get(wire).expect("No op found for wire");

    match op {
        Op::Assign(a, _) => assign_wire(wire, get_value(a, processes, wires), wires),
        Op::Not(a, _) => assign_wire(wire, !get_value(a, processes, wires), wires),
        Op::And(a, b, _) => assign_wire(wire, get_value(a, processes, wires) & get_value(b, processes, wires), wires),
        Op::Or(a, b, _) => assign_wire(wire, get_value(a, processes, wires) | get_value(b, processes, wires), wires),
        Op::LShift(a, n, _) => assign_wire(wire, get_value(a, processes, wires) << n, wires),
        Op::RShift(a, n, _) => assign_wire(wire, get_value(a, processes, wires) >> n, wires),
    }
}

fn solve_for(wire: &str, instructions: &String) -> u16 {
    let mut processes: HashMap<String, Op> = HashMap::new();
    let mut wires: HashMap<String, u16> = HashMap::new();

    for line in instructions.lines() {
        let op = parse_instruction(line);
        let _wire = get_wire_name(&op).to_string();
        processes.insert(_wire, op);
    }

    // part 1
    let answer = evaluate(wire, &processes, &mut wires);

    // part 2
    wires.clear();
    wires.insert("b".to_string(), answer);
    evaluate(wire, &processes, &mut wires)
}

#[cfg(test)]
mod tests {
    use crate::solve_for;

    #[test]
    fn test_wires() {
        let cases = [
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];

        let instructions: String = String::from("123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i");

        for (input, expected) in cases {
            let result = solve_for(input, &instructions);
            assert_eq!(
                result, expected,
                "Failed for input `{}`: expected {}, got {}",
                input, expected, result
            );
        }
    }
}
