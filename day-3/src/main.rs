use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

fn visit_house(direction: char, pos: &mut Pos, grid: &mut HashMap<Pos, u16>) {
    match direction {
        '>' => pos.x += 1,
        '<' => pos.x -= 1,
        '^' => pos.y += 1,
        'v' => pos.y -= 1,
        _ => {}
    }

    *grid.entry(*pos).or_insert(0) += 1;
}

fn santa_night(directions: &str) -> usize {
    let mut grid: HashMap<Pos, u16> = HashMap::new();
    let mut pos = Pos { x: 0, y: 0 };
    grid.insert(pos, 0);

    for direction in directions.chars() {
        visit_house(direction, &mut pos, &mut grid);
    }

    grid.keys().count()
}

fn santa_and_robot_santa_night(directions: &str) -> usize {
    let mut grid: HashMap<Pos, u16> = HashMap::new();
    let mut pos_santa = Pos { x: 0, y: 0 };
    let mut pos_robot = Pos { x: 0, y: 0 };

    grid.insert(pos_santa, 2);
    let chars = directions.chars().collect::<Vec<char>>();

    for pairs in chars.chunks(2) {
        visit_house(pairs[0], &mut pos_santa, &mut grid);
        visit_house(pairs[1], &mut pos_robot, &mut grid);
    }

    grid.keys().count()
}

fn main() {
    let directions = fs::read_to_string("directions.txt")
        .expect("Failed to read directions.txt");

    let santa_alone = santa_night(&directions);
    let santa_with_robot_santa = santa_and_robot_santa_night(&directions);

    println!("Houses visited by Santa: {:?}", santa_alone);
    println!("Houses visited by Santa and Robot santa: {:?}", santa_with_robot_santa);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_houses_santa_only() {
        assert_eq!(santa_night("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_houses_visited_ex2() {
        assert_eq!(santa_and_robot_santa_night("^v^v^v^v^v"), 11);
    }
}
