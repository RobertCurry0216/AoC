use itermore::IterMore;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<usize>>()
        .iter()
        .windows()
        .filter(|[a, b]| a < b)
        .count()

}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<usize>>()
        .iter()
        .windows()
        .filter(|[a, _, _, b]| a < b)
        .count()
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    #[test]
    fn problem1() {
        let expected = 7;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 5;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}