fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    let mut prev = 0;
    let mut count = 0;
    for l in input.lines() {
        let v: i32 = l.trim().parse().expect("error parsing line"); 
        if v > prev {
            count += 1;
        }
        prev = v;
    }
    count - 1
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    let mut sum = 0;
    let mut prev_sum = 0;
    let mut count = 0;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for l in input.lines() {
        let v: i32 = l.trim().parse().expect("error parsing line"); 
        sum = sum + v - a;
        if sum > prev_sum {
            count += 1;
        }
        prev_sum = sum;
        a = b;
        b = c;
        c = v;
    }
    count - 3
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