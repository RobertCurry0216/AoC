use std::collections::HashMap;
use std::time::Instant;
use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 14: Extended Polymerization ---");
    let start = Instant::now();
    println!("problem1: {}\nduration: {:?} \n", solve_problem1(INPUT), start.elapsed());
    let start = Instant::now();
    println!("problem2: {}\nduration: {:?}", solve_problem2(INPUT), start.elapsed());
}

fn parse_input(input: &str) -> (
    HashMap<(char, char), usize>,
    HashMap<char, usize>,
    HashMap<(char, char), char>
) {
    let input = input.split("\n\n").collect::<Vec<_>>();
    let seq_chars = input[0].trim().chars().collect::<Vec<_>>();
    
    let mut seq = HashMap::new();
    for i in 1..seq_chars.len() {
        *seq.entry((seq_chars[i-1], seq_chars[i])).or_insert(0) += 1;
    }

    let mut counts = HashMap::new();
    for &c in seq_chars.iter() {
        *counts.entry(c).or_insert(0) += 1;
    }
    
    let mut rules = HashMap::new();
    let re = Regex::new(r"(\w)(\w) -> (\w)").unwrap();

    for rule in re.captures_iter(input[1]) {
        let key = (rule[1].chars().next().unwrap(), rule[2].chars().next().unwrap());
        let value = rule[3].chars().next().unwrap();
        rules.insert(key, value);
    }

    (seq, counts, rules)
}

fn apply_rules(
    counts: &mut HashMap<char, usize>,
    pairs: HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), char>
) -> HashMap<(char, char), usize> {
    let mut pairs_new = HashMap::new();
    
    for (&(a, b), &c) in rules.iter() {
        if let Some(&val) = pairs.get(&(a, b)) {
            *pairs_new.entry((a, c)).or_insert(0) += val;
            *pairs_new.entry((c, b)).or_insert(0) += val;
            *counts.entry(c).or_insert(0) += val;
        }
    }
    pairs_new
}


#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let (mut seq, mut counts, rules) = parse_input(input);
    for _ in 0..10 {
        seq = apply_rules(&mut counts, seq, &rules);
    }

    let values = counts.into_values().collect::<Vec<_>>();
    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();

    max - min
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let (mut seq, mut counts, rules) = parse_input(input);
    for _ in 0..40 {
        seq = apply_rules(&mut counts, seq, &rules);
    }

    let values = counts.into_values().collect::<Vec<_>>();
    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();

    max - min
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    #[test]
    fn problem1() {
        let expected = 1588;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 2188189693529;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}