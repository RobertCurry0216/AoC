use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 17: Trick Shot ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

struct Target {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

fn parse_input(input: &str) -> Target {
    let re = Regex::new(r"target area: x=([\d-]+)..(\d+), y=([\d-]+)..([\d-]+)").unwrap();
    let m = &re.captures_iter(input).collect::<Vec<_>>()[0];
    Target{
        x1: m[1].parse().unwrap(),
        x2: m[2].parse().unwrap(),
        y1: m[3].parse().unwrap(),
        y2: m[4].parse().unwrap(),
    }
}

#[allow(unused)]
fn solve_problem1(input: &str) -> isize {
    let target = parse_input(input);
    let n = target.y1.abs();
    (n * (n-1)) / 2
}

fn check_velocity(t: &Target, vx: isize, vy: isize) -> bool {
    let mut x = 0;
    let mut dx = vx;
    let mut y = 0;
    let mut dy = vy;
    while y >= t.y1 && x <= t.x2 {
        if t.x1 <= x && t.y2 >= y {
            return true
        }
        x += dx;
        dx -= dx.signum();
        y += dy;
        dy -= 1;
    }
    false
}

#[allow(unused)]
fn solve_problem2(input: &str) -> isize {
    let target = parse_input(input);
    let min_x = 1;
    let max_x = target.x2;
    let min_y = target.y1;
    let max_y = target.y1.abs();
    let mut count = 0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if check_velocity(&target, x, y) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use crate::*;
    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn problem1() {
        let expected = 45;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn problem2() {
        let expected = 112;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}