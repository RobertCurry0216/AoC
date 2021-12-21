use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //println!("input: {:?}", INPUT);
    println!("--- Day 19: Beacon Scanner ---");
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Vec<(isize, isize, isize)>>
}

impl Scanner {
    fn new(input: &str) -> Scanner {
        let re = Regex::new(r"([-\d]+),([-\d]+),([-\d]+)").unwrap();
        let mut beacons = vec![vec![]; 24];

        for m in re.captures_iter(input) {
            let x = m[1].parse::<isize>().unwrap();
            let y = m[2].parse::<isize>().unwrap();
            let z = m[3].parse::<isize>().unwrap();

            beacons[0].push((x,y,z));
            beacons[1].push((y,z,x));
            beacons[2].push((z,x,y));

            beacons[3].push((-x,-y,z));
            beacons[4].push((-y,z,-x));
            beacons[5].push((z,-x,-y));

            beacons[6].push((-x,-y,-z));
            beacons[7].push((-y,-z,-x));
            beacons[8].push((-z,-x,-y));

            beacons[9].push((x,y,-z));
            beacons[10].push((y,-z,x));
            beacons[11].push((-z,x,y));

            beacons[12].push((-y,x,z));
            beacons[13].push((x,z,-y));
            beacons[14].push((z,-y,x));

            beacons[15].push((y,-x,z));
            beacons[16].push((-x,z,y));
            beacons[17].push((z,y,-x));

            beacons[18].push((y,-x,-z));
            beacons[19].push((-x,-z,y));
            beacons[20].push((-z,y,-x));

            beacons[21].push((-y,x,-z));
            beacons[22].push((x,-z,-y));
            beacons[23].push((-z,-y,x));
        }

        Scanner{beacons}
    }

    fn count_overlap(&self, dir: usize, other: &HashSet<(isize, isize, isize)>) -> ((isize, isize, isize), usize) {
        let mut counts = HashMap::new();
        for (x, y, z) in self.beacons[dir].iter() {
            for (a,b,c) in other.iter() {
                *counts.entry((a-x, b-y, c-z)).or_insert(0) += 1;
            }
        }
        
        let mut max = 0;
        let mut off = (0, 0, 0);
        for (&k, &v) in counts.iter() {
            if v > max {
                max = v;
                off = k;
            }
        }

        (off, max)
    }
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input.split("scanner").skip(1).map(Scanner::new).collect()
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let mut scanners = parse_input(input);
    let mut beacons = HashSet::new();

    let scanner_0 = scanners.remove(0);
    let scanner_1 = scanners.remove(0);
    'setup: for (i, b) in scanner_0.beacons.iter().enumerate() {
        beacons.clear();

        for &v in b.iter() {
            beacons.insert(v);
        }

        for j in 0..24 {
            let ((dx, dy, dz), c) = scanner_1.count_overlap(j, &beacons);
            if c >= 12 {
                for &(x, y, z) in &scanner_1.beacons[j] {
                    beacons.insert((x-dx, y-dy, z-dz));
                }
                println!("found the start set");
                break 'setup;
            }
        }
    }
    
    beacons.len()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};
    const TEST_SMALL: &str = "--- scanner 0 ---
    0,2,0
    4,1,0
    3,3,0
    
    --- scanner 1 ---
    -1,-1,0
    -5,0,0
    -2,1,0";


    const TEST_INPUT: &str = "--- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401

    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390

    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562

    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596

    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14";

    #[test]
    fn problem1() {
        let expected = 79;
        let actual = solve_problem1(TEST_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 0;
        let actual = solve_problem2(TEST_INPUT);
        assert_eq!(expected, actual);
    }
}