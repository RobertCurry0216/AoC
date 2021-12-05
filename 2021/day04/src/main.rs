fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[derive(Debug)]
struct BingoCard {
    values: Vec<u32>,
    cols: [u32; 5],
    rows: [u32; 5],
    has_won: bool,
    checked_sum: u32
}

impl BingoCard {
    fn new(values: Vec<u32>) -> BingoCard {
        BingoCard{
            values: values,
            cols: [0; 5],
            rows: [0; 5],
            has_won: false,
            checked_sum: 0,
        }
    }

    // ticks off the number if it's on the board
    // and returns true if the board has won
    fn check_number(&mut self, num: u32) -> bool {
        if self.has_won {
            // exit early if it's already won
            return self.has_won
        }

        if let Some(idx) = self.values.iter().position(|&v| v == num) {
            // add to checked value
            self.checked_sum += self.values[idx];

            // check off row / col
            let x = idx % 5;
            let y = idx / 5;
            self.rows[y] += 1;
            self.cols[x] += 1;
            
            if self.rows[y] == 5 || self.cols[x] == 5 {
                self.has_won = true;
            }
        };
        self.has_won
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<BingoCard>) {
    let mut lines = input.lines();

    // read numbers to be drawn
    let numbers = lines
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut cards: Vec<BingoCard> = Vec::new();

    loop {
        // discard the empty line between boards
        if lines.next() == None {
            break;
        }

        // read the numbers on the bingo card
        let mut values: Vec<u32> = Vec::new();
        for _ in 0..5 {
            lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .for_each(|v| values.push(v));
        }

        cards.push(BingoCard::new(values));
    }

    (numbers, cards)
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u32 {
    let (numbers, mut cards) = parse_input(input);

    for n in numbers {
        for card in cards.iter_mut() {
            if card.check_number(n) {
                let sum: u32 = card.values.iter().sum::<u32>() - card.checked_sum;

                return sum * n;
            }
        }
    }
    panic!()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> u32 {
    let (numbers, mut cards) = parse_input(input);

    for n in numbers {
        // check the number off the bingo cards
        for card in cards.iter_mut() {
            card.check_number(n);
        }

        // return when you're at the last card
        if cards.len() == 1 && cards[0].has_won {
            let sum: u32 = cards[0].values.iter().sum::<u32>() - cards[0].checked_sum;
            return sum * n;
        }

        // remove all the cards that have won
        cards.retain(|c| !c.has_won);

    }
    panic!()
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    #[test]
    fn problem1() {
        let expected = 4512;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 1924;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}