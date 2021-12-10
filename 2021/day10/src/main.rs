fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 10: Syntax Scoring ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn parse_line(l: &str) -> (isize, Vec<char>) {
    let mut stack = vec![];
    for c in l.chars() {
        match c {
            ')' => {
                match stack.pop() {
                    Some('(') => continue,
                    _ => return (3, vec![])
                }
            },
            ']' => {
                match stack.pop() {
                    Some('[') => continue,
                    _ => return (57, vec![])
                }
            }
            '}' => {
                match stack.pop() {
                    Some('{') => continue,
                    _ => return (1197, vec![])
                }
            },
            '>' => {
                match stack.pop() {
                    Some('<') => continue,
                    _ => return (25137, vec![])
                }
            },
            _ => stack.push(c),
        }
    }
    (0, stack.iter().rev().map(|c| *c).collect::<Vec<_>>())
}

#[allow(unused)]
fn solve_problem1(input: &str) -> isize {
    input.lines()
        .map(|s| s.trim())
        .map(|s| parse_line(s))
        .fold(0, |acc, (cur, _)| acc + cur)
}

fn complete_line(chars: Vec<char>) -> isize {
    chars.iter().fold(0, |acc, cur| (acc * 5) + (match cur {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!()
    }))
}

#[allow(unused)]
fn solve_problem2(input: &str) -> isize {
    let mut values = input.lines()
        .map(|s| s.trim())
        .map(|s| parse_line(s))
        .map(|(_, l)| complete_line(l))
        .filter(|&v| v != 0)
        .collect::<Vec<_>>();
    values.sort();
    values[values.len()/2]
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn problem1() {
        let expected = 26397;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 288957;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}