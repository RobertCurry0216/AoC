fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
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