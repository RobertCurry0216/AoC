use bit_vec::BitVec;

type Bits<'a> = bit_vec::Iter<'a>;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 16: Packet Decoder ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn parse_input(input: &str) -> BitVec {
    let h = hex::decode(input.trim()).unwrap();
    BitVec::from_bytes(&h)
}

fn pop_bits(bits: &mut Bits, n: usize) -> usize {
    let mut v = 0;
    for _ in 0..n {
        v <<= 1;
        if let Some(bit) = bits.next() {
            if bit { v |= 1; }
        }
    }
    v
}

trait Packet: std::fmt::Debug {
    fn version_sum(&self) -> usize;
    fn eval(&self) -> usize;
}

#[derive(Debug)]
struct PacketLiteral {
    version: usize,
    id: usize,
    value: usize
}

impl Packet for PacketLiteral {
    fn version_sum(&self) -> usize {
        self.version
    }

    fn eval(&self) -> usize {
        self.value
    }
}

#[derive(Debug)]
struct PacketOperator {
    version: usize,
    id: usize,
    sub_packets: Vec<Box<dyn Packet>>
}

impl Packet for PacketOperator {
    fn version_sum(&self) -> usize {
        self.version + self.sub_packets.iter().map(|p| p.version_sum()).sum::<usize>()
    }

    fn eval(&self) -> usize {
        match self.id {
            // sum
            0 => {
                self.sub_packets.iter().map(|p| p.eval()).sum()
            },
            // product
            1 => {
                self.sub_packets.iter().map(|p| p.eval()).fold(1, |a, c| a * c)
            },
            // minimum
            2 => {
                self.sub_packets.iter().map(|p| p.eval()).fold(usize::MAX, |a, c| std::cmp::min(a,c))
            },
            // maximum
            3 => {
                self.sub_packets.iter().map(|p| p.eval()).fold(0, |a, c| std::cmp::max(a,c))
            },
            // greater than
            5 => {
                if self.sub_packets[0].eval() > self.sub_packets[1].eval() {1} else {0}
            },
            // less than
            6 => {
                if self.sub_packets[0].eval() < self.sub_packets[1].eval() {1} else {0}
            },
            // equal
            7 => {
                if self.sub_packets[0].eval() == self.sub_packets[1].eval() {1} else {0}
            }
            _ => unreachable!()
        }
    }
}

fn parse_packet(bits: &mut Bits) -> Box<dyn Packet> {
    let version = pop_bits(bits, 3);
    let id = pop_bits(bits, 3);
    match id {
        4 => {
            let mut value = 0;
            let mut flag = 1;
            while flag == 1 {
                flag = pop_bits(bits, 1);
                value <<= 4;
                value |= pop_bits(bits, 4);
            }
            Box::new(
                PacketLiteral { version, id, value }
            )
        }
        _ => {
            let mut sub_packets = vec![];
            match pop_bits(bits, 1) {
                0 => {
                    let l = pop_bits(bits, 15);
                    let sub_bits = bits.take(l).collect::<bit_vec::BitVec>();
                    let mut sub_bits = sub_bits.iter();
                    while sub_bits.len() > 0 {
                        sub_packets.push(parse_packet(&mut sub_bits))
                    }
                },
                _ => {
                    let l = pop_bits(bits, 11);
                    for _ in 0..l {
                        sub_packets.push(parse_packet(bits));
                    }
                }
            }
            Box::new(
                PacketOperator{version, id, sub_packets}
            )
        }
    }
}


#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let bits = parse_input(input);
    let mut bits = bits.iter();
    let p = parse_packet(&mut bits);
    p.version_sum()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let bits = parse_input(input);
    let mut bits = bits.iter();
    let p = parse_packet(&mut bits);
    p.eval()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn problem1() {
        let expected = 31;
        let actual = solve_problem1("A0016C880162017C3686B18A3D4780");
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 1;
        let actual = solve_problem2("9C0141080250320F1802104A08");
        assert_eq!(expected, actual);
    }
}