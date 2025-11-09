use std::fs;

struct Answers {
    floor: i32,
    first_basement_move: i32,
}

fn main() {
    let instructions = fs::read_to_string("instructions.txt")
        .expect("Should have been able to read instructions.txt");

    let answers = follow_instructions(&instructions);

    println!("Santa ends up on floor {}", answers.floor);
    println!("The first entry to the basement is {}", answers.first_basement_move);
}

fn follow_instructions(instructions: &str) -> Answers {
    let mut floor = 0;
    let mut first_basement_move:i32 = -1;

    for (i, c) in instructions.char_indices() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor < 0 && first_basement_move == -1 {
            first_basement_move = (i + 1) as i32;
        }
    }

    Answers { floor, first_basement_move }
}

#[cfg(test)]
mod tests {
    use super::{follow_instructions};

    #[test]
    fn test_follow_instructions_ex1() {
        let answers = follow_instructions("))(((((");
        assert_eq!(answers.floor, 3);
        assert_eq!(answers.first_basement_move, 1);
    }

    #[test]
    fn test_follow_instructions_ex2() {
        let answers = follow_instructions("()())");
        assert_eq!(answers.floor, -1);
        assert_eq!(answers.first_basement_move, 5);
    }
}