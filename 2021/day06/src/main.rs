fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 6: Lanternfish ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    // set up init buckets
    let mut buckets = vec![0; 9];
    parse_input(input)
        .iter()
        .for_each(|&v| buckets[v] += 1);

    // iter over days
    for _ in 0..80 {
        let fish = buckets[0];
        buckets.rotate_left(1);
        buckets[6] += fish;
    }

    buckets.iter().sum()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i64 {
    // set up init buckets
    let mut buckets = vec![0; 9];
    parse_input(input)
        .iter()
        .for_each(|&v| buckets[v] += 1);

    // iter over days
    for _ in 0..256 {
        let fish = buckets[0];
        buckets.rotate_left(1);
        buckets[6] += fish;
    }

    buckets.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn problem1() {
        let expected = 5934;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 26984457539;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}