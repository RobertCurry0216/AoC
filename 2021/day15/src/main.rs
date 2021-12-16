use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 15: Chiton ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|l| 
            l.trim()
            .chars()
            .map(|c| 
                c.to_string()
                .parse::<usize>()
                .unwrap()
            )
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
}

#[derive(Debug, Copy, Clone)]
struct Node {
    g_cost: usize,
    h_cost: usize,
    f_cost: usize,
    cost: usize,
    closed: bool
}

fn find_path_cost(start: (usize, usize), target: (usize, usize), map: &Vec<Vec<usize>>) -> usize {
    let mut a_star = vec![];
    let (tx, ty) = target;

    // create node map
    for (y, row) in map.iter().enumerate() {
        let mut nodes = vec![];
        for (x, &n) in row.iter().enumerate() {
            let g_cost = 0;
            let h_cost = tx - x + ty - y;
            let f_cost = g_cost + h_cost;
            let cost = n;
            nodes.push(Node{ g_cost, h_cost, f_cost, cost, closed: false })
        }
        a_star.push(nodes);
    }

    // set up
    let mut open_set = HashSet::new();
    open_set.insert(start);

    // pathfinding
    while open_set.len() > 0 {
        // find next node
        let (mut x, mut y) = open_set.iter().next().unwrap();
        for &(i, j) in open_set.iter() {
            let c = &a_star[y][x];
            let n = &a_star[j][i];
            if n.f_cost < c.f_cost {
                x = i;
                y = j;
            }
        }

        // close node
        a_star[y][x].closed = true;
        open_set.remove(&(x,y));

        // check if target
        if (x, y) == target {
            return a_star[y][x].g_cost;
        }

        // update and add neighbors to open set
        let w = a_star[0].len() -1;
        let h = a_star.len() -1;
        let c = a_star[y][x].clone();
        for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            if (dx == -1 && x == 0) || (dx == 1 && x == w) { continue; }
            if (dy == -1 && y == 0) || (dy == 1 && y == h) { continue; }

            let i = (dx + x as isize) as usize;
            let j = (dy + y as isize) as usize;

            let n = &mut a_star[j][i];
            if n.closed { continue; }

            if open_set.contains(&(i, j)){
                let g_cost = c.g_cost + n.cost;
                if g_cost < n.g_cost {
                    n.g_cost = g_cost;
                }
            } else {
                open_set.insert((i,j));
                n.g_cost = c.g_cost + n.cost;
            }
            n.f_cost = n.g_cost + n.h_cost;
        }
    }
    unreachable!()
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let map = parse_input(input);
    find_path_cost((0, 0), (map[0].len()-1, map.len()-1), &map)
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let mut map = parse_input(input);
    let h = map.len();
    let w = map[0].len();

    // extend rows
    for i in 0..map.len() {
        let mut values = vec![];
        for n in 1..5 {
            let a = map[i].iter().map(|&v| if v+n > 9 {v+n - 9} else {v+n}).collect::<Vec<_>>();
            values.extend(a);
        }
        map[i].extend(values);
    }

    // extend columns
    let mut new_rows = vec![];
    for n in 1..5 {
        let mut rows = vec![];
        for r in map.iter() {
            let a = r.iter().map(|&v| if v+n > 9 {v+n - 9} else {v+n}).collect::<Vec<_>>();
            rows.push(a);
        }
        new_rows.extend(rows);
    }
    map.extend(new_rows);

    find_path_cost((0, 0), (map[0].len()-1, map.len()-1), &map)
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";

    #[test]
    fn problem1() {
        let expected = 40;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 315;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}