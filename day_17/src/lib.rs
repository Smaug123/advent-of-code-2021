pub mod day_17 {

    pub(crate) fn parse(s: &str) -> Vec<u8> {
        let s = s.trim_end();
        let mut answer = Vec::with_capacity(s.len() * 4);

        for c in s.chars() {
            let number = if ('0'..='9').contains(&c) {
                c as u8 - b'0'
            } else {
                (c as u8 - b'A') + 10
            };
            answer.push(number / 8);
            answer.push((number / 4) % 2);
            answer.push((number / 2) % 2);
            answer.push(number % 2);
        }

        answer
    }

    pub fn input() -> Vec<u8> {
        parse(include_str!("../input.txt"))
    }

    pub fn part_1(data: &[u8]) -> u32 {
        panic!("TODO");
    }

    pub fn part_2(data: &[u8]) -> u64 {
        panic!("TODO");
    }
}

#[cfg(test)]
mod tests {
    use super::day_17::*;

    static TEST_INPUT: &str = "";

    #[test]
    fn part1_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_1(&data), 16);
    }

    #[test]
    fn part2_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_2(&data), 2021);
    }

    #[test]
    fn test_day_17() {
        let input = input();
        assert_eq!(part_1(&input), 923);
        assert_eq!(part_2(&input), 258888628940);
    }
}
