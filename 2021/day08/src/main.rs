fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 8: Seven Segment Search ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn parse_input(input: &str) -> Vec<(&str, &str)>{
    input.lines()
        .map(|l| {
            let s = l.split("|")
                .map(|s| s.trim())
                .collect::<Vec<_>>();
            (s[0], s[1])
        })
        .collect::<Vec<_>>()
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let mut count = 0;

    for (_, output) in parse_input(input) {
        count += output
            .split_whitespace()
            .map(|s| s.trim().len())
            .filter(|&v| v == 2 || v == 4 || v == 3 || v == 7 )
            .collect::<Vec<_>>()
            .len();
    }

    count
}

struct Decoder {
    one: Vec<char>,
    four: Vec<char>
}

impl Decoder {
    fn new(mixed_wires: &str) -> Decoder {
        let segments = mixed_wires.split_whitespace().map(|s| s.trim()).collect::<Vec<_>>();
        let one = **segments.iter().filter(|s| s.len() == 2).collect::<Vec<_>>().first().unwrap();
        let four = **segments.iter().filter(|s| s.len() == 4).collect::<Vec<_>>().first().unwrap();

        Decoder {
            one: one.chars().collect(),
            four: four.chars().collect()
        }
    }

    fn common_with_one(&self, s: &str) -> usize {
        self.one.iter().fold(0, |acc, &c| if s.contains(c) {acc + 1} else {acc})
    }

    fn common_with_four(&self, s: &str) -> usize {
        self.four.iter().fold(0, |acc, &c| if s.contains(c) {acc + 1} else {acc})
    }

    fn decode_segment(&self, s: &str) -> usize {
        match s.len() {
            2 => 1,
            4 => 4,
            3 => 7,
            7 => 8,
            5 => {
                if self.common_with_one(s) == 2 {
                    3
                } else if self.common_with_four(s) == 3 {
                    5
                } else {
                    2
                }
            },
            6 => {
                if self.common_with_one(s) == 1 {
                    6
                } else if self.common_with_four(s) == 4 {
                    9
                } else {
                    0
                }
            }
            _ => panic!()
        }
    }

    fn decode(&self, s: &str) -> usize {
        let digits = s.trim().split_whitespace().map(|v| v.trim()).collect::<Vec<_>>();
        let thousands = self.decode_segment(digits[0]) * 1000;
        let hundreds = self.decode_segment(digits[1]) * 100;
        let tens = self.decode_segment(digits[2]) * 10;
        let ones = self.decode_segment(digits[3]);

        thousands + hundreds + tens + ones
    }
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let mut sum = 0;
    for (segs, value) in  parse_input(input) {
        sum += Decoder::new(segs).decode(value);
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2, Decoder};
    const TEST_INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn problem1() {
        let expected = 26;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 61229;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn decoder_test() {
        let d = Decoder::new("cf bcdf");

        assert_eq!(0, d.decode_segment("abcefg"));
        assert_eq!(1, d.decode_segment("cf"));
        assert_eq!(2, d.decode_segment("acdeg"));
        assert_eq!(3, d.decode_segment("acdfg"));
        assert_eq!(4, d.decode_segment("bcdf"));
        assert_eq!(5, d.decode_segment("abdfg"));
        assert_eq!(6, d.decode_segment("abdfeg"));
        assert_eq!(7, d.decode_segment("acf"));
        assert_eq!(8, d.decode_segment("abcdefg"));
        assert_eq!(9, d.decode_segment("abcdfg"));
    }
}