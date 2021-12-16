pub mod day_14 {

    use std::collections::HashMap;

    pub struct Data {
        start: char,
        end: char,
        rules: HashMap<(char, char), char>,
    }

    pub(crate) fn parse(s: &str) -> (Data, HashMap<(char, char), u64>) {
        let mut lines = s.split('\n');
        let start = lines.next().unwrap();
        match lines.next().unwrap() {
            "" => {}
            s => panic!("Expected empty line, got {}", s),
        }

        let mut map = HashMap::new();
        for line in lines {
            let mut iter = line.split(' ');
            let pair = iter.next().unwrap();
            match iter.next().unwrap() {
                "->" => {}
                s => panic!("Expected arrow, got {}", s),
            }
            let middle = iter.next().unwrap();
            match iter.next() {
                None => {}
                Some(next) => panic!("Expected end of line, got {}", next),
            }

            let mut chars = pair.chars();
            let p1 = chars.next().unwrap();
            let p2 = chars.next().unwrap();
            match chars.next() {
                None => {}
                Some(_) => panic!("Expected pair of chars, got {}", pair),
            }

            let mut chars = middle.chars();
            let insert = chars.next().unwrap();
            match chars.next() {
                None => {}
                Some(_) => panic!("Expected single char, got {}", middle),
            }
            map.insert((p1, p2), insert);
        }

        let mut pairs = HashMap::new();
        let mut iter = start.chars();
        let start = iter.next().unwrap();

        let mut prev = start;

        for next in iter {
            pairs
                .entry((prev, next))
                .and_modify(|x| *x += 1)
                .or_insert(1);
            prev = next;
        }

        (
            Data {
                start,
                end: prev,
                rules: map,
            },
            pairs,
        )
    }

    pub fn input() -> (Data, HashMap<(char, char), u64>) {
        parse(include_str!("../input.txt"))
    }

    fn run(
        data: &Data,
        pairs: &HashMap<(char, char), u64>,
        steps: u8,
    ) -> HashMap<(char, char), u64> {
        let mut pairs = pairs.clone();
        for _ in 0..steps {
            let mut after_step = HashMap::new();
            for (&(c1, c2), &count) in pairs.iter() {
                match data.rules.get(&(c1, c2)) {
                    None => {
                        after_step
                            .entry((c1, c2))
                            .and_modify(|x| *x += count)
                            .or_insert(count);
                    }
                    Some(&middle) => {
                        after_step
                            .entry((c1, middle))
                            .and_modify(|x| *x += count)
                            .or_insert(count);
                        after_step
                            .entry((middle, c2))
                            .and_modify(|x| *x += count)
                            .or_insert(count);
                    }
                };
            }
            pairs = after_step;
        }

        pairs
    }

    fn min_max<I, T, U>(mut iter: I, f: fn(T) -> U) -> Option<(T, T)>
    where
        I: Iterator<Item = T>,
        T: Copy,
        U: Ord + Copy,
    {
        let mut min_entry = iter.next()?;
        let mut max_entry = min_entry;
        let mut min = f(min_entry);
        let mut max = min;
        for entry in iter {
            let f_val = f(entry);
            if f_val < min {
                min = f_val;
                min_entry = entry;
            } else if f_val > max {
                max = f_val;
                max_entry = entry;
            }
        }
        Some((min_entry, max_entry))
    }

    fn count(map: &HashMap<(char, char), u64>, start: char, end: char) -> HashMap<char, u64> {
        let mut counts = HashMap::new();
        for (&(c1, c2), &count) in map.iter() {
            counts
                .entry(c1)
                .and_modify(|i| *i += count)
                .or_insert(count);
            counts
                .entry(c2)
                .and_modify(|i| *i += count)
                .or_insert(count);
        }
        *counts.get_mut(&start).unwrap() += 1;
        *counts.get_mut(&end).unwrap() += 1;

        counts
    }

    pub fn part_1(data: &(Data, HashMap<(char, char), u64>)) -> u64 {
        let map = run(&data.0, &data.1, 10);
        let counts = count(&map, data.0.start, data.0.end);
        match min_max(counts.iter(), |x| x.1) {
            Some((min, max)) => max.1 / 2 - min.1 / 2,
            None => panic!("Expected a nonempty map"),
        }
    }

    pub fn part_2(data: &(Data, HashMap<(char, char), u64>)) -> u64 {
        let map = run(&data.0, &data.1, 40);
        let counts = count(&map, data.0.start, data.0.end);
        match min_max(counts.iter(), |x| x.1) {
            Some((min, max)) => max.1 / 2 - min.1 / 2,
            None => panic!("Expected a nonempty map"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day_14::*;

    static TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);
        assert_eq!(part_1(&data), 1588);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);
        assert_eq!(part_2(&data), 2188189693529);
    }

    #[test]
    fn test_day_14() {
        let input = input();
        assert_eq!(part_1(&input), 2549);
        assert_eq!(part_2(&input), 2516901104210);
    }
}
