use std::time::Instant;
use std::collections::HashMap;

const START: usize = 1000;
const END: usize = 1001;
const SMALL_CAVES: usize = 500;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 12: Passage Pathing ---");
    let start = Instant::now();
    println!("problem1: {} duration: {:?}", solve_problem1(INPUT), start.elapsed());
    let start = Instant::now();
    println!("problem2: {} duration: {:?}", solve_problem2(INPUT), start.elapsed());
}

fn parse_input(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut map = HashMap::new();
    let mut count = 0;
    let mut cave_index_map: HashMap<&str, usize> = HashMap::new();
    cave_index_map.insert("start", START);
    cave_index_map.insert("end", END);

    for line in input.lines() {
        let caves = line.trim().split('-').collect::<Vec<_>>();

        let cave_a = match cave_index_map.get(caves[0]) {
            Some(i) => *i,
            None => {
                count += 1;
                let i = if caves[0].chars().all(char::is_lowercase) {
                    count + SMALL_CAVES
                } else {
                    count
                };
                cave_index_map.insert(caves[0], i);
                i
            }
        };

        let cave_b = match cave_index_map.get(caves[1]) {
            Some(i) => *i,
            None => {
                count += 1;
                let i = if caves[1].chars().all(char::is_lowercase) {
                    count + SMALL_CAVES
                } else {
                    count
                };
                cave_index_map.insert(caves[1], i);
                i
            }
        };

        map.entry(cave_a).or_insert(vec![]).push(cave_b);
        map.entry(cave_b).or_insert(vec![]).push(cave_a);
    }
    map
}

fn find_paths(
    map: &HashMap<usize, Vec<usize>>,
    path: &mut Vec<usize>,
    curr: &usize,
    double_visited: bool
) -> usize {
    if *curr == END {
        return 1
    }

    map.get(curr).unwrap().iter().map(|&cave| {
        let mut double_visited = double_visited;
        if cave >= SMALL_CAVES && path.contains(&cave) {
            if cave == START || double_visited {
                return 0;
            } else {
                double_visited = true;
            }
        } 
        path.push(cave);
        let v = find_paths(map, path, &cave, double_visited);
        let _ = path.pop();
        v
    })
    .sum()
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let map = parse_input(input);
    find_paths(&map, &mut vec![START], &START, true)
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let map = parse_input(input);
    find_paths(&map, &mut vec![START], &START, false)
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