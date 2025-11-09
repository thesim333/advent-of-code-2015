use std::fs;

struct Answer {
    paper: u32,
    ribbon: u32,
}

fn main() {
    let packages = fs::read_to_string("packages.txt")
        .expect("Something went wrong reading the file");

    let mut total = Answer { paper: 0, ribbon: 0 };

    for package in packages.lines() {
        let result = calc_package(package);

        total.paper += result.paper;
        total.ribbon += result.ribbon;
    }

    println!("Total Paper: {}, Total Ribbon: {}", total.paper, total.ribbon);
}

fn calc_package(package: &str) -> Answer {
    let mut dims = package
        .split('x')
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    dims.sort_unstable();

    let [l, w, h]: [u32; 3] = dims.try_into().unwrap();
    let sides = [l * w, w * h, l * h];

    Answer {
        paper: sides.iter().map(|v| v * 2).sum::<u32>() + sides[0],
        ribbon: l + l + w + w + l * w * h,
    }
}

#[cfg(test)]
mod tests {
    use super::calc_package;

    #[test]
    fn test_calc_package_ex1() {
        let answer = calc_package("2x3x4");
        assert_eq!(answer.paper, 58);
        assert_eq!(answer.ribbon, 34);
    }

    #[test]
    fn test_calc_package_ex2() {
        let answer = calc_package("1x1x10");
        assert_eq!(answer.paper, 43);
        assert_eq!(answer.ribbon, 14);
    }
}
