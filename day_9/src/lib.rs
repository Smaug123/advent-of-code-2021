pub mod day_9 {

    use ::array::array::*;

    pub(crate) fn parse(s: &str) -> Array<u8> {
        Array::from_rows(
            s.split('\n')
                .map(|line| line.chars().map(|c| char::to_digit(c, 10).unwrap() as u8)),
        )
    }

    pub fn input() -> Array<u8> {
        parse(include_str!("../input.txt"))
    }

    pub fn part_1(data: &Array<u8>) -> u32 {
        let mut answer = 0;
        let row_len = data.row_len();
        let col_len = data.col_len();
        for col in 0..row_len {
            for row in 0..col_len {
                if (row == 0 || data.get(row - 1, col).unwrap() > data.get(row, col).unwrap())
                    && (row == col_len - 1
                        || data.get(row, col).unwrap() < data.get(row + 1, col).unwrap())
                    && (col == 0 || data.get(row, col - 1).unwrap() > data.get(row, col).unwrap())
                    && (col == row_len - 1
                        || data.get(row, col).unwrap() < data.get(row, col + 1).unwrap())
                {
                    answer += *data.get(row, col).unwrap() as u32 + 1;
                }
            }
        }

        answer
    }

    /// Flood-fill, returning the size of the given basin and setting the basin to 0.
    fn flood_fill(data: &mut Array<u8>, row: usize, col: usize) -> usize {
        let new_val = *data.get(row, col).unwrap();
        if new_val >= 9 {
            return 0;
        }
        data.set(row, col, 10);
        let mut ans = 1;
        if row < data.col_len() - 1 {
            ans += flood_fill(data, row + 1, col);
        }
        if row > 0 {
            ans += flood_fill(data, row - 1, col);
        }
        if col < data.row_len() - 1 {
            ans += flood_fill(data, row, col + 1);
        }
        if col > 0 {
            ans += flood_fill(data, row, col - 1);
        }

        ans
    }

    pub fn part_2(data: &Array<u8>) -> u32 {
        let mut data = data.clone();
        let mut answers: Vec<u32> = Vec::new();
        for col in 0..data.row_len() {
            for row in 0..data.col_len() {
                let got = flood_fill(&mut data, row, col) as u32;
                if got > 0 {
                    answers.push(got);
                }
            }
        }
        answers.sort_unstable();
        answers[answers.len() - 1] * answers[answers.len() - 2] * answers[answers.len() - 3]
    }
}

#[cfg(test)]
mod tests {
    use super::day_9::*;

    static TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_1(&data), 15);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_2(&data), 1134);
    }

    #[test]
    fn test_day_9() {
        let input = input();
        assert_eq!(part_1(&input), 539);
        assert_eq!(part_2(&input), 736920);
    }
}
