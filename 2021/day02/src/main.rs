fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l|{
            let split = l.split_whitespace().collect::<Vec<&str>>();
            match split[0] {
                "forward" => Instruction::Forward(split[1].trim().parse().unwrap()),
                "down" => Instruction::Down(split[1].trim().parse().unwrap()),
                "up" => Instruction::Up(split[1].trim().parse().unwrap()),
                _ => unreachable!(),
            }
        }).collect::<Vec<Instruction>>()
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let mut depth = 0;
    let mut horiz = 0;
    let instructions = parse_input(input);

    for instruction in instructions {
        match instruction {
            Instruction::Forward(value) => horiz += value,
            Instruction::Down(value) => depth += value,
            Instruction::Up(value) => depth -= value,
        }
    }

    depth * horiz
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;
    let instructions = parse_input(input);

    for instruction in instructions {
        match instruction {
            Instruction::Forward(value) => {
                horiz += value;
                depth += aim * value;
            },
            Instruction::Down(value) => aim += value,
            Instruction::Up(value) => aim -= value,
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