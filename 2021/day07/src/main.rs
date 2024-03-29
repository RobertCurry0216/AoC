fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 7: The Treachery of Whales ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    let mut crabs = parse_input(input);
    crabs.sort();
    let target: i32 = crabs[crabs.len() / 2];
    
    crabs.iter().map(|v| (v - target).abs()).sum()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    let mut crabs = parse_input(input);
    let mut heat_map = vec![0i32; 2000];

    crabs.iter().for_each(|&c| {
        for i in 0..2000i32 {
            let n = (c-i).abs();
            heat_map[i as usize] += n * (n+1) / 2
        }
    });

    *heat_map.iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn problem1() {
        let expected = 37;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 168;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}