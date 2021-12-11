use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 11: Dumbo Octopus ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

struct Octopus {
    value: isize,
    pos: (isize, isize),
    neighbours: Vec<(isize, isize)>,
}

impl Octopus {
    fn new(x: isize, y: isize, v: isize) -> Octopus {
        let n = vec![(x-1, y-1), (x, y-1), (x+1, y-1),(x-1, y),(x+1, y),(x-1, y+1), (x, y+1), (x+1, y+1),]
            .iter()
            .filter(|(i, j)| *i >= 0 && *i < 10 && *j >= 0 && *j < 10)
            .map(|v| *v)
            .collect::<Vec<_>>();
        Octopus{
            value: v,
            pos: (x, y),
            neighbours: n
        }
    }
}

fn parse_input(input: &str) -> Vec<Octopus> {
    let mut octos = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            octos.push(
                Octopus::new(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    c.to_string().parse().unwrap()
                )
            )
        }
    }
    octos
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let mut octos = parse_input(input);
    let mut flashes = 0;
    let mut flashed: HashSet<(isize, isize)> = HashSet::new();
    let mut to_check = vec![];

    for _ in 0..100 {
        flashed.clear();
        to_check.clear();

        // increase all octopuses by 1
        octos.iter_mut().for_each(|o| {
            o.value += 1;
            if o.value > 9 { 
                to_check.push(o.pos);
            }
        });

        // cascade flashes
        while to_check.len() > 0 {
            let (x, y) = to_check.pop().unwrap();
            if flashed.contains(&(x, y)) {
                continue;
            }

            let o = &mut octos[(x + y*10) as usize];
            o.value += 1;

            if o.value > 9 {
                flashes += 1;
                o.value = 0;
                flashed.insert(o.pos);
                o.neighbours.iter().for_each(|n| to_check.push(*n))
            }
        }
    }
    
    flashes
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let mut octos = parse_input(input);
    let mut step = 0;
    let mut flashes = 0;
    let mut flashed: HashSet<(isize, isize)> = HashSet::new();
    let mut to_check = vec![];

    while flashes != 100 {
        flashes = 0;
        step += 1;
        flashed.clear();
        to_check.clear();    

        // increase all octopuses by 1
        octos.iter_mut().for_each(|o| {
            o.value += 1;
            if o.value > 9 { 
                to_check.push(o.pos);
            }
        });

        // cascade flashes
        while to_check.len() > 0 {
            let (x, y) = to_check.pop().unwrap();
            if flashed.contains(&(x, y)) {
                continue;
            }

            let o = &mut octos[(x + y*10) as usize];
            o.value += 1;

            if o.value > 9 {
                flashes += 1;
                o.value = 0;
                flashed.insert(o.pos);
                o.neighbours.iter().for_each(|n| to_check.push(*n))
            }
        }
    }
    
    step
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";

    #[test]
    fn problem1() {
        let expected = 1656;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 195;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}