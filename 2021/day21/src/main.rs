use cached::proc_macro::cached;
use std::time::Instant;

fn main() {
    const INPUT: (usize, usize) = (2, 5);
    //println!("input: {:?}", INPUT);
    println!("--- Day 21: Dirac Dice ---");
    let start = Instant::now();
    println!("problem1: {}\nduration: {:?}\n", solve_problem1(INPUT), start.elapsed());
    let start = Instant::now();
    println!("problem2: {}\nduration: {:?}", solve_problem2(INPUT), start.elapsed());
}

#[allow(unused)]
fn solve_problem1(input: (usize, usize)) -> usize {
    let mut positions = [input.0, input.1];
    let mut scores = [0,0];
    let mut rolls = 1;
    let mut p = 0;

    while scores[0] < 1000 && scores[1] < 1000 {
        positions[p] = ((positions[p] + (rolls *3) +2) %10) +1;
        rolls += 3;
        scores[p] += positions[p];
        p ^= 1;
    }
    (rolls-1) * (if scores[0] >= 1000 {scores[1]} else {scores[0]})
}

#[cached]
fn count_wins(p1_pos: usize, p1_score: usize, p2_pos: usize, p2_score: usize, die: usize, turn: bool) -> (usize, usize) {
    let mut p1_pos = p1_pos;
    let mut p2_pos = p2_pos;
    let mut p1_score = p1_score;
    let mut p2_score = p2_score;
    
    if turn {
        p1_pos = ((p1_pos + (die-1)) %10) +1;
        p1_score = p1_score + p1_pos;
    } else {
        p2_pos = ((p2_pos + (die-1)) %10) +1;
        p2_score = p2_score + p2_pos;
    }

    if p1_score >= 21 {return (1,0)}
    if p2_score >= 21 {return (0,1)}

    let mut s1 = 0;
    let mut s2 = 0;

    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                let (p1s, p2s) = count_wins(p1_pos, p1_score, p2_pos, p2_score, a+b+c, !turn);
                s1 += p1s;
                s2 += p2s;
            }
        }
    }
    (s1, s2)
}

#[allow(unused)]
fn solve_problem2(input: (usize, usize)) -> usize {
    let (p1, p2) = input;
    let mut s1 = 0;
    let mut s2 = 0;

    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                let (p1s, p2s) = count_wins(p1, 0, p2, 0, a+b+c, true);
                s1 += p1s;
                s2 += p2s;
            }
        }
    }
    std::cmp::max(s1, s2)
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: (usize, usize) = (4, 8);

    #[test]
    fn problem1() {
        let expected = 739785;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 444356092776315;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}