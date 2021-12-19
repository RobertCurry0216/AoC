fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 18: Snailfish ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[derive(Debug, Clone)]
struct SnailFish {
    values: Vec<usize>,
    depths: Vec<usize>
}

impl SnailFish {
    fn explode(&mut self) -> bool {
        for i in 0..self.values.len() {
            if self.depths[i] > 4 {
                if i > 0 {
                    self.values[i-1] += self.values[i];
                }
                if i < self.values.len() - 2 {
                    self.values[i+2] += self.values[i+1];
                }
                self.values.remove(i);
                self.values[i] = 0;
                self.depths.remove(i);
                self.depths[i] -= 1;
                return true
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.values.len() {
            if self.values[i] > 9 {
                let l = self.values[i] / 2;
                let r = l + self.values[i] % 2;
                self.values[i] = r;
                self.values.insert(i, l);

                let d = self.depths[i] + 1;
                self.depths[i] = d;
                self.depths.insert(i, d);
                return true
            }
        }

        false
    }

    fn add(&mut self, other: SnailFish) {
        self.values.extend(other.values);
        self.depths.extend(other.depths);
        self.depths.iter_mut().for_each(|d| *d += 1);
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() { continue }
            if self.split() { continue }
            break;
        }
    }

    fn magnitude(&self) -> usize {
        let mut v = self.values.clone();
        let mut d = self.depths.clone();

        'outer: while v.len() > 1 {
            for i in 0..v.len() -1 {
                if d[i] == d[i+1] {
                    d.remove(i);
                    d[i] -= 1;

                    v[i] = v[i]*3 + v[i+1]*2;
                    v.remove(i+1);

                    continue 'outer;
                }
            }
        }

        v[0]
    }
}

fn parse_snailfish_number(input: &str) -> SnailFish {
    let mut values = vec![];
    let mut depths = vec![];
    let mut d = 0;

    for c in input.trim().chars() {
        match c {
            '[' => d += 1,
            ']' => d -= 1,
            ',' => (),
            _ => {
                values.push(c.to_digit(10).unwrap() as usize);
                depths.push(d);
            }
        }
    }

    SnailFish{values, depths}
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut n = parse_snailfish_number(lines.next().unwrap());
    for l in lines {
        let m = parse_snailfish_number(l);
        n.add(m);
        n.reduce();
    }
    n.magnitude()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let snailfish = input.lines().map(parse_snailfish_number).collect::<Vec<_>>();
    let mut max = 0;

    for i in 0..snailfish.len() -1 {
        for j in i+1..snailfish.len() {
            let mut a = snailfish[i].clone();
            let b = snailfish[j].clone();
            a.add(b);
            a.reduce();
            max = std::cmp::max(max, a.magnitude());

            let mut a = snailfish[j].clone();
            let b = snailfish[i].clone();
            a.add(b);
            a.reduce();
            max = std::cmp::max(max, a.magnitude());
        }
    }

    max
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn problem1() {
        let expected = 4140;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 3993;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}