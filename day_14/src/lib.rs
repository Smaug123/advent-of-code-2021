pub mod day_14 {

    pub(crate) fn parse(s: &str) -> Vec<u8> {
        panic!("TODO")
    }

    pub fn input() -> Vec<u8> {
        parse(include_str!("../input.txt"))
    }

    pub fn part_1(data: &[u8]) -> u32 {
        panic!("TODO");
    }

    pub fn part_2(_data: &[u8]) -> u32 {
        panic!("TODO");
    }
}

#[cfg(test)]
mod tests {
    use super::day_14::*;

    static TEST_INPUT: &str = "";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);
        assert_eq!(part_1(&data), 226);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);
        assert_eq!(part_2(&data), 36);
    }

    #[test]
    fn test_day_14() {
        let input = input();
        assert_eq!(part_1(&input), 5958);
        assert_eq!(part_2(&input), 150426);
    }
}
