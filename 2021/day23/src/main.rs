use core::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 23: Amphipod ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Amphipod{
    Amber,
    Bronze,
    Copper,
    Desert,
    Empty
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    hallway: Vec<Amphipod>,
    burrow_a: Vec<Amphipod>,
    burrow_b: Vec<Amphipod>,
    burrow_c: Vec<Amphipod>,
    burrow_d: Vec<Amphipod>,
}

impl State {
    fn new(
        a1: Amphipod, a2: Amphipod,
        b1: Amphipod, b2: Amphipod,
        c1: Amphipod, c2: Amphipod,
        d1: Amphipod, d2: Amphipod,
    ) -> State {
        State {
            cost: 0,
            hallway: vec![Amphipod::Empty; 11],
            burrow_a: vec![a1, a2],
            burrow_b: vec![b1, b2],
            burrow_c: vec![c1, c2],
            burrow_d: vec![d1, d2],
        }
    }

    
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    0
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "";

    #[test]
    fn problem1() {
        let expected = 0;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 0;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}