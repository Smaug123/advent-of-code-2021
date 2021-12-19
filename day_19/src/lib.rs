pub mod day_19 {

    use std::cmp::max;
    use std::collections::HashSet;
    use std::fmt::{Debug, Display};

    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct Coord {
        x: i32,
        y: i32,
        z: i32,
    }

    impl Coord {
        pub(crate) fn make(x: i32, y: i32, z: i32) -> Coord {
            Coord { x, y, z }
        }
    }

    impl Display for Coord {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {})", self.x, self.y, self.z)
        }
    }

    impl Debug for Coord {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {})", self.x, self.y, self.z)
        }
    }

    #[derive(Clone)]
    pub struct Scanner {
        pub(crate) coords: Vec<Coord>,
        pub(crate) index: u8,
    }

    fn parse_vector(s: &str) -> Option<Coord> {
        if s.len() == 0 {
            return None;
        }
        let mut coords = s.split(',').map(|s| str::parse::<i32>(s).unwrap());
        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();
        match coords.next() {
            None => {}
            Some(c) => {
                panic!("Expected last coord, got {:?}", c);
            }
        }

        Some(Coord { x, y, z })
    }

    pub(crate) fn parse(s: &str) -> Vec<Scanner> {
        let mut answer = Vec::new();
        // skip the first, empty, element
        for scanner in s.split("--- scanner ").skip(1) {
            let mut v = Vec::new();
            let mut lines = scanner.split('\n');
            let first_line = lines.next().unwrap();
            for line in lines {
                match parse_vector(line) {
                    None => {}
                    Some(vec) => {
                        v.push(vec);
                    }
                }
            }

            let index = str::parse::<u8>(first_line.split(' ').next().unwrap()).unwrap();
            answer.push(Scanner { coords: v, index });
        }
        answer
    }

    #[derive(PartialEq, Eq, Debug)]
    enum Axis {
        X,
        Y,
        Z,
    }

    pub(crate) struct ScannerOrientationIter<'a> {
        original: &'a Scanner,
        yield_next: Scanner,
        rotation_count: u8,
        up_axis: Axis,
        sense: bool,
        ended: bool,
    }

    impl<'a> std::fmt::Debug for ScannerOrientationIter<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Original {}, rotated {}, up is {:?}, sense {}",
                self.original.index, self.rotation_count, self.up_axis, self.sense
            )
        }
    }

    fn rotate(scanner: &mut Scanner, around: &Axis) {
        match around {
            Axis::X => {
                // X-axis stays the same; swap y/z and negate first
                for row in scanner.coords.iter_mut() {
                    std::mem::swap(&mut row.y, &mut row.z);
                    row.y *= -1;
                }
            }
            Axis::Y => {
                for row in scanner.coords.iter_mut() {
                    std::mem::swap(&mut row.z, &mut row.x);
                    row.z *= -1;
                }
            }
            Axis::Z => {
                for row in scanner.coords.iter_mut() {
                    std::mem::swap(&mut row.x, &mut row.y);
                    row.x *= -1;
                }
            }
        }
    }

    fn flip(scanner: &mut Scanner, up_direction: &Axis) {
        match up_direction {
            Axis::X => {
                for row in scanner.coords.iter_mut() {
                    row.x *= -1;
                    row.y *= -1;
                }
            }
            Axis::Y => {
                for row in scanner.coords.iter_mut() {
                    row.y *= -1;
                    row.z *= -1;
                }
            }
            Axis::Z => {
                for row in scanner.coords.iter_mut() {
                    row.z *= -1;
                    row.x *= -1;
                }
            }
        }
    }

    fn new_axis(axis: &Axis) -> Axis {
        match axis {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }

    fn move_to_axis(existing: &Scanner, output: &mut Scanner, axis: &Axis) {
        // (1,2,3) with X up, to Y up, makes (2, -1, 3)
        match axis {
            Axis::Y => {
                for (index, value) in existing.coords.iter().enumerate() {
                    output.coords[index] = Coord {
                        x: value.y,
                        y: value.z,
                        z: value.x,
                    };
                }
            }
            Axis::Z => {
                for (index, value) in existing.coords.iter().enumerate() {
                    output.coords[index] = Coord {
                        x: value.z,
                        y: value.x,
                        z: value.y,
                    };
                }
            }
            Axis::X => {
                for (index, value) in existing.coords.iter().enumerate() {
                    output.coords[index] = value.clone();
                }
            }
        }
    }

    impl<'a> Iterator for ScannerOrientationIter<'a> {
        type Item = Scanner;

        fn next(&mut self) -> Option<Self::Item> {
            if self.ended {
                return None;
            }
            let to_ret = self.yield_next.clone();
            if self.rotation_count < 3 {
                self.rotation_count += 1;
                rotate(&mut self.yield_next, &Axis::X);
                return Some(to_ret);
            }
            self.rotation_count = 0;
            rotate(&mut self.yield_next, &Axis::X);

            if !self.sense {
                self.sense = true;
                flip(&mut self.yield_next, &Axis::X);
                return Some(to_ret);
            }
            self.sense = false;

            self.up_axis = new_axis(&self.up_axis);
            if self.up_axis == Axis::X {
                self.ended = true;
            }
            move_to_axis(self.original, &mut self.yield_next, &self.up_axis);
            Some(to_ret)
        }
    }

    pub(crate) fn all_orientations<'a>(s: &'a Scanner) -> ScannerOrientationIter<'a> {
        ScannerOrientationIter {
            original: s,
            rotation_count: 0,
            up_axis: Axis::X,
            sense: false,
            yield_next: s.clone(),
            ended: false,
        }
    }

    pub fn input() -> Vec<Scanner> {
        parse(include_str!("../input.txt"))
    }

    const fn subtract(c1: &Coord, c2: &Coord) -> Coord {
        Coord {
            x: c1.x - c2.x,
            y: c1.y - c2.y,
            z: c1.z - c2.z,
        }
    }

    const fn add(c1: &Coord, c2: &Coord) -> Coord {
        Coord {
            x: c1.x + c2.x,
            y: c1.y + c2.y,
            z: c1.z + c2.z,
        }
    }

    /// Return what we need to add to each of scanner_2 to get us translated to scanner_1.
    pub(crate) fn align(scanner_1: &Scanner, scanner_2: &Scanner) -> Option<Coord> {
        for point in scanner_1.coords.iter() {
            for target_index in 0..scanner_2.coords.len() {
                // Translate target to equal point
                let translation = subtract(&point, &scanner_2.coords[target_index]);
                let alignments = scanner_2
                    .coords
                    .iter()
                    .map(|p| add(p, &translation))
                    .filter(|p| scanner_1.coords.contains(p))
                    .take(12)
                    .collect::<Vec<_>>();
                if alignments.len() == 12 {
                    return Some(translation);
                }
            }
        }
        None
    }

    pub(crate) fn stitch(scanners: &[Scanner]) -> Vec<(Coord, Scanner)> {
        let mut placed: Vec<Option<(Coord, Scanner)>> =
            std::iter::repeat(None).take(scanners.len()).collect();
        placed[0] = Some((Coord { x: 0, y: 0, z: 0 }, scanners[0].clone()));
        let mut placed_count = 1;

        while placed_count < placed.len() {
            for match_scanner_index in 0..scanners.len() {
                match placed[match_scanner_index].clone() {
                    None => {
                        // We can't denote positions relative to this one, so skip it
                    }
                    Some((base_offset, base_rotation)) => {
                        for (i, scanner) in scanners.iter().enumerate() {
                            if placed[i].is_none() && match_scanner_index != i {
                                for rotation in all_orientations(&scanner) {
                                    match align(&base_rotation, &rotation) {
                                        None => {}
                                        Some(alignment) => {
                                            placed[i] =
                                                Some((add(&alignment, &base_offset), rotation));
                                            placed_count += 1;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        placed
            .iter()
            .map(|i| i.as_ref().unwrap())
            .cloned()
            .collect()
    }

    pub fn part_1(data: &[Scanner]) -> usize {
        let placed = stitch(data);
        placed
            .iter()
            .flat_map(|(offset, rotated)| {
                rotated.coords.iter().map(move |probe| add(&offset, &probe))
            })
            .collect::<HashSet<_>>()
            .len()
    }

    const fn abs(x: i32) -> u32 {
        if x < 0 {
            (-x) as u32
        } else {
            x as u32
        }
    }

    const fn manhattan(c1: &Coord, c2: &Coord) -> u32 {
        let subtracted = subtract(c1, c2);
        abs(subtracted.x) + abs(subtracted.y) + abs(subtracted.z)
    }

    pub fn part_2(data: &[Scanner]) -> u32 {
        let placed: Vec<Coord> = stitch(data)
            .iter()
            .map(|(location, _)| location)
            .cloned()
            .collect();

        let mut m = 0;
        for (i, c1) in placed.iter().enumerate() {
            for j in i + 1..placed.len() {
                m = max(manhattan(c1, &placed[j]), m);
            }
        }

        m
    }
}

#[cfg(test)]
mod tests {
    use super::day_19::*;

    static TEST_INPUT: &str = "--- scanner 0 ---
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
    fn part1_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_1(&data), 79);
    }

    #[test]
    fn part2_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_2(&data), 3621);
    }

    #[test]
    fn test_day_19() {
        let input = input();
        assert_eq!(part_1(&input), 372);
        assert_eq!(part_2(&input), 12241);
    }
}
