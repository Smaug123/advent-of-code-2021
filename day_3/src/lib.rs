pub mod day_3 {

    use std::cmp::Ordering;

    pub(crate) fn parse<const N: usize>(s: &str) -> [bool; N] {
        let mut answer = [false; N];
        for (i, c) in s.chars().enumerate() {
            answer[i] = c != '0';
        }
        answer
    }

    pub fn input<const N: usize>() -> Vec<[bool; N]> {
        let input = include_str!("../input.txt");
        input
            .trim()
            .split('\n')
            .map(parse)
            .collect::<Vec<[bool; N]>>()
    }

    // lol interview prep
    fn boyer_moore_majority_element<I, U>(input: I) -> U
    where
        U: Eq,
        U: Copy,
        I: Iterator<Item = U>,
    {
        let mut counter = 0;
        let mut bag = None;
        for elt in input {
            if counter == 0 {
                bag = Some(elt);
                counter = 1;
            } else if elt == bag.unwrap() {
                counter += 1;
            } else {
                counter -= 1;
            }
        }
        bag.unwrap()
    }

    fn most_common<const N: usize>(bits: &[[bool; N]], pos: usize) -> bool {
        boyer_moore_majority_element(bits.iter().map(|arr| arr[pos]))
    }

    pub fn part_1<const N: usize>(bits: &[[bool; N]]) -> u32 {
        let mut gamma = 0;
        let mut eps = 0;
        for i in 0..N {
            let most = most_common(bits, i);
            gamma = gamma * 2 + if most { 1 } else { 0 };
            eps = eps * 2 + if most { 0 } else { 1 };
        }
        gamma * eps
    }

    fn majority_element_weak<I>(input: I, default: bool) -> Option<bool>
    where
        I: Iterator<Item = bool>,
    {
        let mut falses = 0;
        let mut trues = 0;
        for elt in input {
            if elt {
                trues += 1;
            } else {
                falses += 1;
            }
        }
        if trues + falses == 1 {
            return None;
        }

        match trues.cmp(&falses) {
            Ordering::Greater => Some(true),
            Ordering::Less => Some(false),
            Ordering::Equal => Some(default),
        }
    }

    fn most_common_skipping<F, const N: usize>(
        bits: &[[bool; N]],
        pos: usize,
        default: bool,
        should_ignore: F,
    ) -> Result<&[bool; N], bool>
    where
        F: Fn(&[bool; N]) -> bool,
    {
        let mut elt = None;

        let majority = majority_element_weak(
            bits.iter().filter(|&a| !should_ignore(a)).map(|arr| {
                elt = Some(arr);
                arr[pos]
            }),
            default,
        );
        match majority {
            None => Ok(elt.unwrap()),
            Some(b) => Err(b),
        }
    }

    fn agree<const N: usize>(b1: &[bool; N], b2: &[bool; N], count: usize) -> bool {
        for i in 0..count {
            if b1[i] != b2[i] {
                return false;
            }
        }
        true
    }

    fn to_u16<const N: usize>(bits: [bool; N]) -> u16 {
        let mut ans = 0;
        for b in bits.iter() {
            ans = ans * 2 + if *b { 1 } else { 0 };
        }

        ans
    }

    pub fn oxygen<const N: usize>(bits: &[[bool; N]]) -> u16 {
        let mut most_common_arr = [false; N];
        for i in 0..N {
            let most = most_common_skipping(bits, i, true, |arr| !agree(&most_common_arr, arr, i));
            match most {
                Ok(answer) => {
                    return to_u16(*answer);
                }
                Err(b) => {
                    most_common_arr[i] = b;
                }
            }
        }
        to_u16(most_common_arr)
    }

    pub fn co2<const N: usize>(bits: &[[bool; N]]) -> u16 {
        let mut least_common_arr = [false; N];
        for i in 0..N {
            let most = most_common_skipping(bits, i, true, |arr| !agree(&least_common_arr, arr, i));
            match most {
                Ok(answer) => {
                    return to_u16(*answer);
                }
                Err(b) => {
                    least_common_arr[i as usize] = !b;
                }
            }
        }
        to_u16(least_common_arr)
    }

    pub fn part_2<const N: usize>(bits: &[[bool; N]]) -> u32 {
        let oxygen = oxygen(bits);
        let co = co2(bits);

        (oxygen as u32) * (co as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::day_3::*;

    #[test]
    fn part1_known() {
        let input = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .map(parse::<5>);

        assert_eq!(part_1(&input), 198);
    }

    #[test]
    fn part2_known() {
        let input = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .map(parse::<5>);
        assert_eq!(part_2(&input), 230);
    }

    #[test]
    fn test_day_2() {
        let input = input::<12>();
        assert_eq!(part_1(&input), 3374136);
        assert_eq!(part_2(&input), 1620141160);
    }
}
