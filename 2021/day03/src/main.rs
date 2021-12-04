fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT, 12));
    println!("problem2: {}", solve_problem2(INPUT, 12));
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Bit {
    One,
    Zero
}

fn parse_input(input: &str) -> Vec<Vec<Bit>> {
    input.lines()
        .map(|l| 
            l
            .trim()
            .chars()
            .map(|c| {
                match c {
                    '1' => Bit::One,
                    '0' => Bit::Zero,
                    _ => unreachable!()
                }
            })
            .collect::<Vec<Bit>>())
        .collect::<Vec<Vec<Bit>>>()
}

fn most_common_bits(bytes: &Vec<Vec<Bit>>) -> Vec<Bit> {
    let mut sums: Vec<i32> = vec![0; bytes[0].len()];
    for byte in bytes {
        for (i, bit) in byte.iter().enumerate() {
            match bit {
                Bit::One => sums[i] += 1,
                Bit::Zero => sums[i] -= 1,
            }
        }
    }

    // map values to Bit enum
    sums
        .iter()
        .map(|v| if v >= &0 {Bit::One} else {Bit::Zero})
        .collect()
}

#[allow(unused)]
fn solve_problem1(input: &str, size: usize) -> u32 {
    let input = parse_input(input);
    let sums = most_common_bits(&input);
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for i in 0..size {
        gamma <<= 1;
        epsilon <<= 1;
        match sums[i] {
            Bit::One => gamma |= 1,
            Bit::Zero => epsilon |= 1,
        }
    }
    gamma * epsilon
}

fn oxygen_generator_rating(input: &Vec<Vec<Bit>>, size: usize) -> u32 {
    let mut bytes = input.clone();
    for i in 0..size {
        let common_bits = most_common_bits(&bytes);
        bytes.retain(|b| b[i] == common_bits[i]);
        if bytes.len() == 1 {
            break;
        }
    }

    let mut value: u32 = 0;
    for i in 0..size {
        value <<= 1;
        match bytes[0][i] {
            Bit::One => value |= 1,
            _ => ()
        }
    }

    value
}

fn co2_scrubber_rating(input: &Vec<Vec<Bit>>, size: usize) -> u32 {
    let mut bytes = input.clone();
    for i in 0..size {
        let common_bits = most_common_bits(&bytes);
        bytes.retain(|b| b[i] != common_bits[i]);
        if bytes.len() == 1 {
            break;
        }
    }

    let mut value: u32 = 0;
    for i in 0..size {
        value <<= 1;
        match bytes[0][i] {
            Bit::One => value |= 1,
            _ => ()
        }
    }

    value
}

#[allow(unused)]
fn solve_problem2(input: &str, size: usize) -> u32 {
    let bytes = parse_input(input);
    let oxygen = oxygen_generator_rating(&bytes, size);
    let co2 = co2_scrubber_rating(&bytes, size);
    oxygen * co2
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    #[test]
    fn problem1() {
        let expected = 198;
        let actual = solve_problem1(TEST_INPUT, 5);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 230;
        let actual = solve_problem2(TEST_INPUT, 5);
        assert_eq!(expected, actual);
    }
}