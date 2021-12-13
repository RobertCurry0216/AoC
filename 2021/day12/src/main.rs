use std::time::Instant;
use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 12: Passage Pathing ---");
    let start = Instant::now();
    println!("problem1: {} duration: {:?}", solve_problem1(INPUT), start.elapsed());
    let start = Instant::now();
    println!("problem2: {} duration: {:?}", solve_problem2(INPUT), start.elapsed());
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let caves = line.trim().split('-').collect::<Vec<_>>();
        map.entry(caves[0]).or_insert(vec![]).push(caves[1]);
        map.entry(caves[1]).or_insert(vec![]).push(caves[0]);
    }

    map
}

fn find_paths<'a>(
    map: &HashMap<&'a str, Vec<&'a str>>,
    path: &mut Vec<&'a str>,
    curr: &'a str,
    double_visited: bool
) -> usize {
    if curr == "end" {
        return 1
    }

    map.get(curr).unwrap().iter().map(|&cave| {
        let mut double_visited = double_visited;
        if cave.chars().all(char::is_lowercase) && path.contains(&cave) {
            if cave == "start" || double_visited {
                return 0;
            } else {
                double_visited = true;
            }
        } 
        path.push(cave);
        let v = find_paths(map, path, cave, double_visited);
        let _ = path.pop();
        v
    })
    .sum()
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let map = parse_input(input);
    find_paths(&map, &mut vec!["start"], "start", true)
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let map = parse_input(input);
    find_paths(&map, &mut vec!["start"], "start", false)
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc";

    #[test]
    fn problem1() {
        let expected = 19;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 103;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}