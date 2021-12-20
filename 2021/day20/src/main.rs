use std::time::Instant;

type Pixels = Vec<Vec<bool>>;
const MAX_COORD: isize = 160;
const MIN_COORD: isize = -60;
const OFF: usize = MIN_COORD.abs() as usize;
const SIZE: usize = (MAX_COORD - MIN_COORD) as usize;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 20: Trench Map ---");
    let start = Instant::now();
    println!("problem1: {}\nduration: {:?}\n", solve_problem1(INPUT), start.elapsed());
    let start = Instant::now();
    println!("problem2: {}\nduration: {:?}\n", solve_problem2(INPUT), start.elapsed());
}

fn parse_input(input: &str) -> (Vec<bool>, Pixels) {
    let mut lines = input.lines();
    let key = lines.next().unwrap().trim().chars().map(|c| c == '#').collect();
    let _ = lines.next();
    
    let mut pixels = vec![vec![false; SIZE]; SIZE];
    for (y, l) in lines.enumerate() {
        for (x, c) in l.trim().chars().enumerate() {
            if c == '#' { 
                pixels[y+OFF][x+OFF] = true;
            }
        }
    }

    (key, pixels)
}

fn apply_enhancement(pixels: &Pixels, key: &Vec<bool>) -> Pixels {
    let mut new_pixels = vec![vec![false; SIZE]; SIZE];
    let dirs: Vec<(isize, isize)> = vec![
        (-1,-1), (0,-1), (1,-1),
        (-1, 0), (0, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1)
        ];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let on_border = x <= 0 || x >= SIZE-1 || y <= 0 || y >= SIZE-1;
            if on_border {
                if key[0] {new_pixels[y][x] = !pixels[y][x]};
            } else {
                let mut v = 0;
                for (dx, dy) in dirs.iter() {
                    v <<= 1;
                    if pixels[(y as isize +dy) as usize][(x as isize+dx) as usize] { v |= 1; }
                }
                new_pixels[y][x] = key[v];
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
    
    let mut count = 0;
    for row in pixels {
        for v in row {
            if v {count += 1}
        }
    }
    count
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let (key, mut pixels) = parse_input(input);
    for _ in 0..50 {
        pixels = apply_enhancement(&pixels, &key);
    }
    let mut count = 0;
    for row in pixels {
        for v in row {
            if v {count += 1}
        }
    }
    count
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