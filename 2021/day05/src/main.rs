use regex::Regex;
use std::cmp;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 5: Hydrothermal Venture ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[derive(Debug)]
struct Line(i32, i32, i32, i32);

fn parse_input(input: &str) -> Vec<Line> {
    let mut lines = vec![];
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    for m in re.captures_iter(input) {
        lines.push(
            Line(
                m[1].parse().unwrap(),
                m[2].parse().unwrap(),
                m[3].parse().unwrap(),
                m[4].parse().unwrap(),
            )
        );
    }
    lines
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    let lines = parse_input(input);
    let mut board = vec![0; 1000*1000];

    for Line(x1, y1, x2, y2) in lines {
        if y1 == y2 { // horizontal
            let start = cmp::min(x1, x2);
            let end = cmp::max(x1, x2);
            for x in start..=end {
                board[(x + y1*1000) as usize] += 1;
            }
        } else if x1 == x2 { // vertical
            let start = cmp::min(y1, y2);
            let end = cmp::max(y1, y2);
            for y in start..=end {
                board[(x1 + y*1000) as usize] += 1;
            }
        }
    }

    board.iter().fold(0, |acc, &cur| if cur > 1 {acc+1} else {acc})
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    let lines = parse_input(input);
    let mut board = vec![0; 1000*1000];

    for Line(x1, y1, x2, y2) in lines {
        let mut x = x1;
        let mut y = y1;
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        let l = if x1 == x2 {(y1 - y2).abs()} else {(x1 - x2).abs()};

        for _ in 0..=l {
            board[(x + y*1000) as usize] += 1;
            x += dx;
            y += dy;
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