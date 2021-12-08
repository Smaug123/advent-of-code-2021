pub mod day_8 {

    #[derive(Copy, Clone, Debug)]
    pub struct Digit {
        pub(crate) segments: [bool; 7],
    }

    #[derive(Debug)]
    pub struct Data {
        seen: Vec<Digit>,
        current: [Digit; 4],
    }

    fn parse_digit(s: &str) -> Digit {
        let mut answer = [false; 7];
        for c in s.chars() {
            if ('a'..='g').contains(&c) {
                answer[c as usize - 97] = true;
            } else {
                panic!("Unexpected char: {} in {}", c, s);
            }
        }
        Digit { segments: answer }
    }

    fn parse_row(s: &str) -> Data {
        let mut iter = s.split('|');
        let left = iter.next().unwrap().trim_end();
        let seen = left.split(' ').map(parse_digit).collect::<Vec<Digit>>();
        let right = iter.next().unwrap().trim_start();
        match iter.next() {
            None => {}
            Some(next) => {
                panic!("Expected two components, got {}", next);
            }
        }

        let current = right
            .split(' ')
            .map(parse_digit)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Data { seen, current }
    }

    pub(crate) fn parse(s: &str) -> Vec<Data> {
        s.trim().split('\n').map(parse_row).collect()
    }

    pub fn input() -> Vec<Data> {
        parse(include_str!("../input.txt"))
    }

    fn count_on(d: &Digit) -> u32 {
        let answer = d.segments.iter().filter(|&i| *i).count() as u32;
        answer
    }

    fn is_easy_count(i: u32) -> bool {
        i == 2 || i == 3 || i == 4 || i == 7
    }

    pub fn part_1(data: &[Data]) -> u32 {
        data.iter()
            .map::<u32, _>(|line| {
                line.current
                    .iter()
                    .filter(|i| is_easy_count(count_on(i)))
                    .count() as u32
            })
            .sum()
    }

    pub(crate) fn equal<T>(a1: &[T], a2: &[T]) -> bool
    where
        T: Eq,
    {
        for (i, v1) in a1.iter().enumerate() {
            if *v1 != a2[i] {
                return false;
            }
        }
        true
    }

    pub(crate) fn translate(identification: &[u8; 7], d: &Digit) -> Option<u8> {
        let mut unshuffled = [false; 7];
        for (after_shuffle, before_shuffle) in identification.iter().enumerate() {
            // (0, 2); (1, 5); ...
            unshuffled[*before_shuffle as usize] = d.segments[after_shuffle as usize];
        }

        if !unshuffled[0] {
            // 1 or 4
            if !unshuffled[1] {
                if equal(&unshuffled[2..7], &[true, false, false, true, false]) {
                    return Some(1);
                }
                return None;
            }
            if equal(&unshuffled[2..7], &[true, true, false, true, false]) {
                return Some(4);
            }
            return None;
        }

        if !unshuffled[1] {
            // 2, 3, 7
            if !unshuffled[2] {
                return None;
            }
            if !unshuffled[3] {
                // 7 only
                if equal(&unshuffled[4..7], &[false, true, false]) {
                    return Some(7);
                }
                return None;
            }
            if unshuffled[4] {
                // 2
                if equal(&unshuffled[5..7], &[false, true]) {
                    return Some(2);
                }
                return None;
            }

            if equal(&unshuffled[5..7], &[true, true]) {
                return Some(3);
            }
            return None;
        }

        if !unshuffled[2] {
            // 5, 6
            if !unshuffled[3] {
                return None;
            }
            if unshuffled[4] {
                if equal(&unshuffled[5..7], &[true, true]) {
                    return Some(6);
                } else {
                    return None;
                }
            }
            if equal(&unshuffled[5..7], &[true, true]) {
                return Some(5);
            }
            return None;
        }

        if !unshuffled[3] {
            // 0 only
            if equal(&unshuffled[4..7], &[true, true, true]) {
                return Some(0);
            }
            return None;
        }

        if !unshuffled[4] {
            if equal(&unshuffled[5..7], &[true, true]) {
                return Some(9);
            }
            return None;
        }

        if equal(&unshuffled[5..7], &[true, true]) {
            return Some(8);
        }

        None
    }

    fn last_decreasing<const N: usize>(d: &[u8; N]) -> Option<usize> {
        for i in (0..N - 1).rev() {
            if d[i + 1] > d[i] {
                return Some(i);
            }
        }
        None
    }

    fn reverse<T, const N: usize>(d: &mut [T; N], i: usize)
    where
        T: Copy,
    {
        let mut x = i;
        let mut y = d.len() - 1;
        while x < y {
            d.swap(x, y);
            x += 1;
            y -= 1;
        }
    }

    pub(crate) fn next_perm<const N: usize>(d: &mut [u8; N]) -> Option<()> {
        // First decreasing element
        let last_decreasing = last_decreasing(d)?;
        let elt = d[last_decreasing];
        let incremented = match (last_decreasing + 1..d.len()).rev().find(|i| d[*i] > elt) {
            Some(incremented) => incremented,
            None => {
                panic!("Unexpected ");
            }
        };
        d.swap(last_decreasing, incremented);
        reverse(d, last_decreasing + 1);
        Some(())
    }

    /// Returns a permutation of 0..=6, so e.g. 2541063 means
    /// the input 'a' refers to c, 'b' refers to f, and so on.
    fn identify(d: &Data) -> [u8; 7] {
        let mut perm = [0u8; 7];
        for (i, slot) in perm.iter_mut().enumerate() {
            *slot = i as u8;
        }

        loop {
            match d
                .seen
                .iter()
                .map(|digit| translate(&perm, digit))
                .find(|translation| translation.is_none())
            {
                None => {
                    return perm;
                }
                Some(_) => {
                    // Some digit was untranslateable
                }
            }
            match next_perm(&mut perm) {
                None => {
                    panic!("Expected an answer!");
                }
                Some(()) => {
                    // loop!
                }
            }
        }
    }

    pub fn part_2(data: &[Data]) -> u32 {
        data.iter()
            .map(|line| {
                let identification = identify(line);
                line.current
                    .map(|i| match translate(&identification, &i) {
                        None => {
                            panic!(
                                "Couldn't translate {:?} using identification {:?}",
                                i, identification
                            );
                        }
                        Some(digit) => digit,
                    })
                    .iter()
                    .fold(0, |state, next| state * 10 + (*next as u32))
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::day_8::*;

    static TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_1(&data), 26);
    }

    #[test]
    fn test_next_perm_even() {
        let mut perm = [0u8; 8];
        for i in 0..perm.len() {
            perm[i] = i as u8;
        }

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[0, 1, 2, 3, 4, 5, 7, 6]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[0, 1, 2, 3, 4, 6, 5, 7]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[0, 1, 2, 3, 4, 6, 7, 5]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[0, 1, 2, 3, 4, 7, 5, 6]), true);

        for i in 0..perm.len() {
            perm[i] = (perm.len() - 1 - i) as u8;
        }
        assert_eq!(next_perm(&mut perm), None);
        assert_eq!(equal(&perm, &[7, 6, 5, 4, 3, 2, 1, 0]), true);
    }

    #[test]
    fn test_next_perm_odd() {
        let mut perm = [0u8; 7];
        for i in 0..perm.len() {
            perm[i] = i as u8 + 1;
        }

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[1, 2, 3, 4, 5, 7, 6]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[1, 2, 3, 4, 6, 5, 7]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[1, 2, 3, 4, 6, 7, 5]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[1, 2, 3, 4, 7, 5, 6]), true);

        for (i, &value) in [0, 1, 2, 5, 3, 4, 6].iter().enumerate() {
            perm[i] = value;
        }
        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[0, 1, 2, 5, 3, 6, 4]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[0, 1, 2, 5, 4, 3, 6]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[0, 1, 2, 5, 4, 6, 3]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[0, 1, 2, 5, 6, 3, 4]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[0, 1, 2, 5, 6, 4, 3]), true);

        next_perm(&mut perm);
        assert_eq!(equal(&perm, &[0, 1, 2, 6, 3, 4, 5]), true);

        for i in 0..perm.len() {
            perm[i] = (perm.len() - i) as u8;
        }
        assert_eq!(next_perm(&mut perm), None);
        assert_eq!(equal(&perm, &[7, 6, 5, 4, 3, 2, 1]), true);
    }

    #[test]
    fn part2_mini_known() {
        let data = parse(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );

        assert_eq!(part_2(&data), 5353);
    }

    #[test]
    fn translations() {
        let identification = [0, 1, 2, 3, 4, 5, 6];
        assert_eq!(
            translate(
                &identification,
                &Digit {
                    segments: [true, true, true, false, true, true, true]
                }
            ),
            Some(0)
        );
        assert_eq!(
            translate(
                &identification,
                &Digit {
                    segments: [false, false, true, false, false, true, false]
                }
            ),
            Some(1)
        );
        assert_eq!(
            translate(
                &identification,
                &Digit {
                    segments: [true, false, true, true, true, false, true]
                }
            ),
            Some(2)
        );
        assert_eq!(
            translate(
                &identification,
                &Digit {
                    segments: [true, false, true, true, false, true, true]
                }
            ),
            Some(3)
        );
        assert_eq!(
            translate(
                &identification,
                &Digit {
                    segments: [false, true, true, true, false, true, false]
                }
            ),
            Some(4)
        );
        assert_eq!(
            translate(
                &identification,
                &Digit {
                    segments: [true, true, false, true, false, true, true]
                }
            ),
            Some(5)
        );
        assert_eq!(
            translate(
                &identification,
                &Digit {
                    segments: [true, true, false, true, true, true, true]
                }
            ),
            Some(6)
        );
        assert_eq!(
            translate(
                &identification,
                &Digit {
                    segments: [true, false, true, false, false, true, false]
                }
            ),
            Some(7)
        );
        assert_eq!(
            translate(
                &identification,
                &Digit {
                    segments: [true, true, true, true, true, true, true]
                }
            ),
            Some(8)
        );
        assert_eq!(
            translate(
                &identification,
                &Digit {
                    segments: [true, true, true, true, false, true, true]
                }
            ),
            Some(9)
        );
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_2(&data), 61229);
    }

    #[test]
    fn test_day_8() {
        let input = input();
        assert_eq!(part_1(&input), 412);
        assert_eq!(part_2(&input), 978171);
    }
}
