fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let mut depth = 0;
    let mut horiz = 0;

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let dir = split[0];
        let dist: usize = split[1].trim().parse().expect("error parsing dist");

        match dir {
            "forward" => horiz += dist,
            "down" => depth += dist,
            "up" => depth -= dist,
            _ => panic!(),
        }
    }

    depth * horiz
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let dir = split[0];
        let dist: usize = split[1].trim().parse().expect("error parsing dist");

        match dir {
            "forward" => {
                horiz += dist;
                depth += aim * dist;
            },
            "down" => aim += dist,
            "up" => aim -= dist,
            _ => panic!(),
        }
    }

    depth * horiz
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    #[test]
    fn problem1() {
        let expected = 150;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 900;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}