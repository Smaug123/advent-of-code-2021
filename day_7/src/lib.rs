pub mod day_7 {

    const fn difference(a: u16, b: u16) -> u16 {
        if a >= b {
            a - b
        } else {
            b - a
        }
    }

    pub(crate) fn parse(s: &str) -> Vec<u16> {
        s.trim()
            .split(',')
            .map(str::parse::<u16>)
            .map(|i| i.unwrap())
            .collect()
    }

    pub fn input() -> Vec<u16> {
        parse(include_str!("../input.txt"))
    }

    fn min_convex<I, T>(mut data: I) -> Option<T>
    where
        I: Iterator<Item = T>,
        T: Ord,
    {
        let mut fst = data.next()?;
        loop {
            let next = data.next();
            match next {
                None => {
                    return Some(fst);
                }
                Some(next) => {
                    if fst > next {
                        fst = next;
                    } else {
                        return Some(fst);
                    }
                }
            }
        }
    }

    pub fn part_1(data: &[u16]) -> u32 {
        let max = *data.iter().max().unwrap();
        min_convex((0..=max).map(|i| data.iter().map(|value| difference(i, *value) as u32).sum()))
            .unwrap()
    }

    const fn triangle(i: u32) -> u32 {
        i * (i + 1) / 2
    }

    pub fn part_2(data: &[u16]) -> u32 {
        let max = *data.iter().max().unwrap();
        min_convex((0..=max).map(|i| {
            data.iter()
                .map(|value| triangle(difference(i, *value) as u32))
                .sum()
        }))
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::day_7::*;

    static TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_1(&data), 37);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_2(&data), 168);
    }

    #[test]
    fn test_day_7() {
        let input = input();
        assert_eq!(part_1(&input), 356992);
        assert_eq!(part_2(&input), 101268110);
    }
}
