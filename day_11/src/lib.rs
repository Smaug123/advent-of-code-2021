pub mod day_11 {

    // TODO: commonise with day 9
    #[derive(Debug, Clone)]
    pub struct Array<T> {
        row_len: usize,
        elts: Vec<T>,
    }

    impl<T> Array<T> {
        fn col_len(&self) -> usize {
            self.elts.len() / self.row_len
        }
        fn get(&self, row: usize, col: usize) -> T
        where
            T: Copy,
        {
            self.elts[row * self.row_len + col]
        }
        fn set(&mut self, row: usize, col: usize, val: T)
        where
            T: Copy,
        {
            self.elts[row * self.row_len + col] = val;
        }
        fn apply<F>(&mut self, row: usize, col: usize, f: F)
        where
            F: Fn(T) -> T,
            T: Copy,
        {
            self.set(row, col, f(self.get(row, col)));
        }
    }

    pub(crate) fn parse(s: &str) -> Array<u8> {
        let mut answer = Array {
            row_len: 0,
            elts: Vec::new(),
        };
        for line in s.split('\n') {
            if answer.row_len == 0 {
                answer.row_len = line.len();
            }
            answer
                .elts
                .extend(line.chars().map(|c| char::to_digit(c, 10).unwrap() as u8));
        }
        answer
    }

    pub fn input() -> Array<u8> {
        parse(include_str!("../input.txt"))
    }

    fn flash_once(data: &mut Array<u8>, flashes: &mut Array<bool>) -> u32 {
        for col in 0..data.row_len {
            for row in 0..data.col_len() {
                data.apply(row, col, |i| i + 1);
            }
        }

        let mut still_flashing = true;
        while still_flashing {
            still_flashing = false;
            for col in 0..data.row_len {
                for row in 0..data.col_len() {
                    if !flashes.get(row, col) && data.get(row, col) > 9 {
                        still_flashing = true;
                        flashes.set(row, col, true);
                        if col > 0 {
                            if row > 0 {
                                data.apply(row - 1, col - 1, |i| i + 1);
                            }
                            if row < data.col_len() - 1 {
                                data.apply(row + 1, col - 1, |i| i + 1);
                            }
                            data.apply(row, col - 1, |i| i + 1);
                        }
                        if col < data.row_len - 1 {
                            if row > 0 {
                                data.apply(row - 1, col + 1, |i| i + 1);
                            }
                            if row < data.col_len() - 1 {
                                data.apply(row + 1, col + 1, |i| i + 1);
                            }
                            data.apply(row, col + 1, |i| i + 1);
                        }

                        if row > 0 {
                            data.apply(row - 1, col, |i| i + 1);
                        }
                        if row < data.col_len() - 1 {
                            data.apply(row + 1, col, |i| i + 1);
                        }
                    }
                }
            }
        }

        let mut total_flashes = 0;
        for col in 0..data.row_len {
            for row in 0..data.col_len() {
                if flashes.get(row, col) {
                    total_flashes += 1;
                    flashes.set(row, col, false);
                    data.set(row, col, 0);
                }
            }
        }
        total_flashes
    }

    pub fn part_1(data: &Array<u8>) -> u32 {
        let mut data = data.clone();
        let mut flashes = Array {
            row_len: data.row_len,
            elts: {
                let mut v = Vec::with_capacity(data.elts.len());
                v.resize(data.elts.len(), false);
                v
            },
        };

        let mut total_flashes = 0;

        for _ in 0..100 {
            total_flashes += flash_once(&mut data, &mut flashes);
        }

        total_flashes
    }

    pub fn part_2(data: &Array<u8>) -> u32 {
        let mut data = data.clone();
        let mut flashes = Array {
            row_len: data.row_len,
            elts: {
                let mut v = Vec::with_capacity(data.elts.len());
                v.resize(data.elts.len(), false);
                v
            },
        };

        let desired = (data.row_len * data.col_len()) as u32;
        for i in 0.. {
            if flash_once(&mut data, &mut flashes) == desired {
                return i + 1;
            }
        }
        panic!("Loop shouldn't have ended");
    }
}

#[cfg(test)]
mod tests {
    use super::day_11::*;

    static TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_1(&data), 1656);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_2(&data), 195);
    }

    #[test]
    fn test_day_10() {
        let input = input();
        assert_eq!(part_1(&input), 1644);
        assert_eq!(part_2(&input), 229);
    }
}
