use regex::Regex;
use std::cmp;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[derive(Debug)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_vertical(&self) -> bool {
        self.x1 ==self.x2
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    let mut lines = vec![];
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    for m in re.captures_iter(input) {
        lines.push(
            Line{
                x1: m[1].parse::<u32>().unwrap(),
                y1: m[2].parse::<u32>().unwrap(),
                x2: m[3].parse::<u32>().unwrap(),
                y2: m[4].parse::<u32>().unwrap(),
            }
        );
    }
    lines
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u32 {
    let lines = parse_input(input);
    let mut board = vec![0; 1000*1000];

    for line in lines {
        if line.is_horizontal() {
            let start = cmp::min(line.x1, line.x2);
            let end = cmp::max(line.x1, line.x2);
            for x in start..=end {
                board[(x + line.y1*1000) as usize] += 1;
            }
        } else if line.is_vertical() {
            let start = cmp::min(line.y1, line.y2);
            let end = cmp::max(line.y1, line.y2);
            for y in start..=end {
                board[(line.x1 + y*1000) as usize] += 1;
            }
        }
    }

    board.iter().fold(0, |acc, &cur| if cur > 1 {acc+1} else {acc})
}

#[allow(unused)]
fn solve_problem2(input: &str) -> u32 {
    let lines = parse_input(input);
    let mut board = vec![0; 1000*1000];

    for line in lines {
        if line.is_horizontal() {
            let start = cmp::min(line.x1, line.x2);
            let end = cmp::max(line.x1, line.x2);
            for x in start..=end {
                board[(x + line.y1*1000) as usize] += 1;
            }
        } else if line.is_vertical() {
            let start = cmp::min(line.y1, line.y2);
            let end = cmp::max(line.y1, line.y2);
            for y in start..=end {
                board[(line.x1 + y*1000) as usize] += 1;
            }
        } else {
            let mut x: i32 = line.x1 as i32;
            let mut y: i32 = line.y1 as i32;
            let dx: i32 = if line.x1 < line.x2 {1} else {-1};
            let dy: i32 = if line.y1 < line.y2 {1} else {-1};
            let l = if line.x1 < line.x2 {line.x2 - line.x1} else {line.x1 - line.x2};

            for _ in 0..=l {
                board[(x+ y*1000) as usize] += 1;
                x += dx;
                y += dy;
            }
        }
    }

    board.iter().fold(0, |acc, &cur| if cur > 1 {acc+1} else {acc})
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    #[test]
    fn problem1() {
        let expected = 5;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 12;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}