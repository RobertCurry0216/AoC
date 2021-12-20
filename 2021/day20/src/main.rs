use std::collections::HashSet;

type Pixels = HashSet<(isize, isize)>;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 20: Trench Map ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn parse_input(input: &str) -> (Vec<bool>, Pixels) {
    let mut lines = input.lines();
    let key = lines.next().unwrap().trim().chars().map(|c| c == '#').collect();
    let _ = lines.next();
    
    let mut pixels = HashSet::new();
    for (y, l) in lines.enumerate() {
        for (x, c) in l.trim().chars().enumerate() {
            if c == '#' { pixels.insert((x as isize, y as isize)); }
        }
    }

    (key, pixels)
}

fn apply_enhancement(pixels: &Pixels, key: &Vec<bool>) -> Pixels {
    let mut new_pixels = HashSet::new();
    let dirs = vec![
        (-1,-1), (0,-1), (1,-1),
        (-1, 0), (0, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1)
        ];
    
    let min = -60isize;
    let max = 160isize;

    for y in min..=max {
        for x in min..=max {
            let on_border = (x <= min || x >= max || y <= min || y >= max) & key[0];
            if on_border {
                if !pixels.contains(&(x, y)) { new_pixels.insert((x, y)); }
            } else {
                let mut v = 0;
                for (dx, dy) in dirs.iter() {
                    v <<= 1;
                    if pixels.contains(&(x+dx, y+dy)) { v |= 1; }
                }
                if key[v] { new_pixels.insert((x, y)); }
            }
        }
    }

    new_pixels
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let (key, mut pixels) = parse_input(input);
    pixels = apply_enhancement(&pixels, &key);
    pixels = apply_enhancement(&pixels, &key);
    pixels.len()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let (key, mut pixels) = parse_input(input);
    for _ in 0..50 {
        pixels = apply_enhancement(&pixels, &key);
    }
    pixels.len()
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
    #..#.
    #....
    ##..#
    ..#..
    ..###";

    #[test]
    fn problem1() {
        let expected = 35;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 3351;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}