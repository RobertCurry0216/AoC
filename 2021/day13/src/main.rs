use std::cmp;
use regex::Regex;
use itertools::Itertools;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 13: Transparent Origami ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2:\n");
    solve_problem2(INPUT);
}

#[derive(Debug)]
enum Fold {
    X(isize),
    Y(isize)
}

fn parse_input(input: &str) -> (Vec<(isize, isize)>, Vec<Fold>) {
    let mut dots = vec![];
    let mut folds = vec![];
    let dot_re = Regex::new(r"(\d+),(\d+)").unwrap();
    let fold_re = Regex::new(r"fold along ([xy])=(\d+)").unwrap();

    for m in dot_re.captures_iter(input) {
        dots.push(
            (m[1].parse().unwrap(), m[2].parse().unwrap())
        )
    }

    for m in fold_re.captures_iter(input) {
        match &m[1] {
            "x" => folds.push(Fold::X(m[2].parse().unwrap())),
            "y" => folds.push(Fold::Y(m[2].parse().unwrap())),
            _ => unreachable!()
        }
    }

    (dots, folds)
}

fn fold_paper(paper: &mut Vec<(isize, isize)>, fold: &Fold) {
    for mut dot in paper {
        match fold {
            Fold::X(axis) => if &dot.0 > axis {dot.0 -= (dot.0 - axis).abs() * 2},
            Fold::Y(axis) => if &dot.1 > axis {dot.1 -= (dot.1 - axis).abs() * 2}
        }
    }
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let (mut dots, folds) = parse_input(input);
    fold_paper(&mut dots, &folds[0]);
    dots = dots.into_iter().unique().collect();
    dots.len()
}

#[allow(unused)]
fn solve_problem2(input: &str) {
    let (mut dots, folds) = parse_input(input);
    for fold in folds {
        fold_paper(&mut dots, &fold);
        dots = dots.into_iter().unique().collect();
    }

    let (x, y) = dots.iter().fold((0, 0), |a, c| (cmp::max(a.0, c.0 + 1), cmp::max(a.1, c.1 + 1)));
    let mut paper = vec![vec![" "; x as usize]; y as usize];
    dots.iter().for_each(|&(i, j)| paper[j as usize][i as usize] = "#");

    for j in 0..y {
        for i in 0..x {
            print!("{}", paper[j as usize][i as usize]);
        }
        print!("\n");
    }
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5";

    #[test]
    fn problem1() {
        let expected = 17;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        solve_problem2(TEST_INPUT);
    }
}