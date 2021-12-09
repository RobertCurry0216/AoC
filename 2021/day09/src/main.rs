use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("--- Day 9: Smoke Basin ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn parse_input(input: &str) -> (usize, usize, Vec<usize>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let height = lines.len();
    let width = lines[0].len();
    let mut values = vec![];

    for line in lines {
        let mut nums = line.trim().chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect();
        values.append(&mut nums)
    }

    (width, height, values)
}

fn find_low_points(w: usize, h: usize, values: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut points = vec![];
    for y in 0..h {
        for x in 0..w {
            let v = values[x + y*w];
            let a1 = if x > 0   {values[(x-1) + y*w]} else {v+1};
            let a2 = if x < w-1 {values[(x+1) + y*w]} else {v+1};
            let a3 = if y > 0   {values[x + (y-1)*w]} else {v+1};
            let a4 = if y < h-1 {values[x + (y+1)*w]} else {v+1};

            if a1 > v && a2 > v && a3 > v && a4 > v {
                points.push((x,y))
            }
        }
    }
    points
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let (w, h, values) = parse_input(input);
    let points = find_low_points(w, h, &values);
    
    points.iter().map(|(x,y)| values[x + y*w] + 1).sum()
}

fn get_neighbours((x, y): (usize, usize), w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut n = vec![];
    if x > 0   {n.push((x-1, y))};
    if x < w-1 {n.push((x+1, y))};
    if y > 0   {n.push((x, y-1))};
    if y < h-1 {n.push((x, y+1))};
    n
}

fn flood_fill(x: usize, y: usize, w: usize, h: usize, values: &Vec<usize>) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit = vec![(x,y)];

    while to_visit.len() > 0 {
        let l = to_visit.pop().unwrap();
        if visited.contains(&l) {
            continue;
        }

        let (x, y) = l;
        if values[x + y*w] < 9 {
            visited.insert(l);
            to_visit.append(&mut get_neighbours(l, w, h))
        }
    }

    visited.len()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let (w, h, values) = parse_input(input);
    let points = find_low_points(w, h, &values);

    let mut fills = points.iter().map(|&(x, y)| flood_fill(x, y, w, h, &values)).collect::<Vec<_>>();
    fills.sort();
    fills.iter().rev().take(3).fold(1, |acc, cur| acc * cur)
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";

    #[test]
    fn problem1() {
        let expected = 15;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 1134;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}