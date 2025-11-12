use std::collections::HashMap;
use std::fs;
use regex::Regex;
use once_cell::sync::Lazy;

static PARTS: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(turn off|turn on|toggle)\s(\d+),(\d+)\sthrough\s(\d+),(\d+)$").unwrap());

fn main() {
    let list = fs::read_to_string("instructions.txt")
        .expect("Should have been able to read list.txt");

    let part_1_actions: HashMap<&str, Box<dyn Fn(&mut u32)>> = HashMap::from([
        ("turn on", Box::new(|v: &mut u32| *v = 1) as Box<dyn Fn(&mut u32)>),
        ("turn off", Box::new(|v: &mut u32| *v = 0) as Box<dyn Fn(&mut u32)>),
        ("toggle", Box::new(|v: &mut u32| *v = (*v + 1) % 2) as Box<dyn Fn(&mut u32)>),
    ]);

    let part_2_actions: HashMap<&str, Box<dyn Fn(&mut u32)>> = HashMap::from([
        ("turn on", Box::new(|v: &mut u32| *v += 1) as Box<dyn Fn(&mut u32)>),
        ("turn off", Box::new(|v: &mut u32| *v = v.saturating_sub(1)) as Box<dyn Fn(&mut u32)>),
        ("toggle", Box::new(|v: &mut u32| *v += 2) as Box<dyn Fn(&mut u32)>),
    ]);

    println!("Part 1: {:?}", solve(&list, part_1_actions));

    println!("Part 2: {:?}", solve(&list, part_2_actions));
}

fn solve(list: &String, actions: HashMap<&str, Box<dyn Fn(&mut u32)>>) -> u32 {
    let mut grid: Vec<Vec<u32>> = vec![vec![0u32; 1000]; 1000];

    for line in list.lines() {
        if let Some(caps) = PARTS.captures(line) {
            let action = actions.get(caps.get(1).unwrap().as_str()).unwrap();

            let nums: Vec<u16> = (2..=5)
                .map(|i| caps.get(i).unwrap().as_str().parse::<u16>().unwrap())
                .collect();

            for i in nums[0]..=nums[2] {
                for j in nums[1]..=nums[3] {
                    action(&mut grid[i as usize][j as usize]);
                }
            }
        }
    }

    grid.iter().flatten().sum::<u32>()
}
