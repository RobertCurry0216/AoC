use std::collections::BinaryHeap;
use std::time::Instant;
use core::cmp::Ordering;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 15: Chiton ---");
    let start = Instant::now();
    println!("problem1: {}\nduration: {:?}", solve_problem1(INPUT), start.elapsed());
    let start = Instant::now();
    println!("problem2: {}\nduration: {:?}", solve_problem2(INPUT), start.elapsed());
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal { return Some(cost); }
        if cost > dist[position] { continue; }

        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

fn create_graph(map: &Vec<Vec<usize>>) -> Vec<Vec<Edge>> {
    let mut nodes = vec![];
    let w = map[0].len();
    let h = map.len();

    for y in 0..h {
        for x in 0..w {
            let mut node = vec![];
            for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                if (dx == -1 && x == 0) || (dx == 1 && x == w-1) { continue; }
                if (dy == -1 && y == 0) || (dy == 1 && y == h-1) { continue; }
                
                let i = (dx + x as isize) as usize;
                let j = (dy + y as isize) as usize;
                
                node.push(Edge{ node: i + j*w, cost: map[j][i] })
            }
            nodes.push(node);
        }
    }

    nodes
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let map = parse_input(input);
    let graph = create_graph(&map);
    let tx = map[0].len() - 1;
    let ty = map.len() - 1;
    shortest_path(&graph, 0, tx + ty*(tx+1)).unwrap()
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

    let graph = create_graph(&map);
    let tx = map[0].len() - 1;
    let ty = map.len() - 1;
    shortest_path(&graph, 0, tx + ty*(tx+1)).unwrap()
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