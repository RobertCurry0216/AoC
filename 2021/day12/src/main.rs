use std::collections::{HashMap, HashSet};

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 12: Passage Pathing ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
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

fn find_paths(map: &HashMap<&str, Vec<&str>>, path: HashSet<&str>, curr: &str) -> usize {
    if curr == "end" {
        return 1
    }
    
    map.get(curr).unwrap().iter().map(|&cave| {
        if cave.chars().all(char::is_lowercase) && path.contains(&cave) {
            0
        } else {
            let mut new_path = path.clone();
            new_path.insert(cave);
            find_paths(map, new_path, cave)
        }
    })
    .sum()
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let map = parse_input(input);
    find_paths(&map, HashSet::from(["start"]), "start")
}


fn find_paths_allow_small_cave_twice(map: &HashMap<&str, Vec<&str>>, path: HashSet<&str>, curr: &str, double_visited: bool) -> usize {
    if curr == "end" {
        return 1
    }
    
    map.get(curr).unwrap().iter().map(|&cave| {
        if cave.chars().all(char::is_lowercase) && path.contains(&cave) {
            if cave == "start" || double_visited {
                0
            } else {
                let mut new_path = path.clone();
                new_path.insert(cave);
                find_paths_allow_small_cave_twice(map, new_path, cave, true)
            }
        } else {
            let mut new_path = path.clone();
            new_path.insert(cave);
            find_paths_allow_small_cave_twice(map, new_path, cave, double_visited)
        }
    })
    .sum()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let map = parse_input(input);
    find_paths_allow_small_cave_twice(&map, HashSet::from(["start"]), "start", false)
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