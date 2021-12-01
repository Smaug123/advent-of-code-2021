pub mod day_1 {

    pub fn input() -> Vec<u32> {
        let input = include_str!("../input.txt");
        input
            .trim()
            .split('\n')
            .map(|l| str::parse::<u32>(l).unwrap())
            .collect::<Vec<u32>>()
    }

    pub fn part_1<T>(numbers: &mut T) -> u32
    where
        T: Iterator<Item = u32>,
    {
        let mut count = 0;
        let mut previous = 0;
        loop {
            if let Some(i) = numbers.next() {
                if previous < i {
                    count += 1;
                }
                previous = i;
            } else {
                return count - 1;
            }
        }
    }

    pub fn part_2_naive(numbers: &[u32]) -> u32 {
        part_1(&mut numbers.windows(3).map(|x| x[0] + x[1] + x[2]))
    }

    pub fn part_2(numbers: &[u32]) -> u32 {
        let mut count = 0;
        for i in 3..numbers.len() {
            if numbers[i - 3] < numbers[i] {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::day_1::*;

    #[test]
    fn part1_known() {
        assert_eq!(
            part_1(
                &mut [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
                    .iter()
                    .copied()
            ),
            7
        );
    }

    #[test]
    fn part2_known() {
        assert_eq!(
            part_2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }

    #[test]
    fn test_day_1() {
        let input = input();
        assert_eq!(part_1(&mut input.iter().copied()), 1766);
        assert_eq!(part_2(&input), 1797);
    }
}
