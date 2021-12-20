pub mod day_20 {

    // TODO: unify with day 9
    #[derive(Debug, Clone)]
    pub struct Array<T> {
        row_len: usize,
        elts: Vec<T>,
    }

    impl<T> Array<T> {
        fn col_len(&self) -> usize {
            self.elts.len() / self.row_len
        }
        fn get(&self, row: usize, col: usize) -> Option<T>
        where
            T: Copy,
        {
            let index = row * self.row_len + col;
            if row < self.row_len && col < self.col_len() && index < self.elts.len() {
                Some(self.elts[index])
            } else {
                None
            }
        }
        fn set(&mut self, row: usize, col: usize, val: T)
        where
            T: Copy,
        {
            self.elts[row * self.row_len + col] = val;
        }
    }

    impl std::fmt::Display for Array<bool> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in 0..self.row_len {
                for col in 0..self.col_len() {
                    write!(
                        f,
                        "{}",
                        if self.get(row, col).unwrap() {
                            'X'
                        } else {
                            '.'
                        }
                    )?;
                }
                writeln!(f, "")?;
            }
            Ok(())
        }
    }

    pub struct Data {
        pub(crate) key: [bool; 512],
        pub(crate) image: Array<bool>,
    }

    fn parse_char(c: char) -> bool {
        if c == '#' {
            true
        } else if c == '.' {
            false
        } else {
            panic!("Unexpected char '{}'", c);
        }
    }

    pub(crate) fn parse(s: &str) -> Data {
        let mut lines = s.trim_end().split('\n');
        let first_line = lines.next().unwrap();
        let mut key = [false; 512];
        for (i, entry) in first_line.chars().map(parse_char).enumerate() {
            key[i] = entry;
        }

        match lines.next().unwrap() {
            "" => {}
            s => {
                panic!("Expected empty line, got {}", s);
            }
        }

        let mut image_elts = Vec::new();
        let mut row_len = 0;

        for line in lines {
            if row_len == 0 {
                row_len = line.len();
            }
            image_elts.reserve(row_len);
            for c in line.chars() {
                image_elts.push(parse_char(c));
            }
        }

        Data {
            image: Array {
                row_len,
                elts: image_elts,
            },
            key,
        }
    }

    pub fn input() -> Data {
        parse(include_str!("../input.txt"))
    }

    // row and col are given with respect to the result image, not the source image
    pub(crate) fn pixel_at(
        key: &[bool; 512],
        source_image: &Array<bool>,
        row: usize,
        col: usize,
        background: bool,
    ) -> bool {
        let mut ans_index = 0;
        for row in (row as isize) - 1..=(row as isize) + 1 {
            for col in (col as isize) - 1..=(col as isize) + 1 {
                let new_bit = if row >= 1 && col >= 1 {
                    match source_image.get((row - 1) as usize, (col - 1) as usize) {
                        None => background,
                        Some(v) => v,
                    }
                } else {
                    background
                };

                ans_index *= 2;
                if new_bit {
                    ans_index += 1;
                } else {
                }
            }
        }

        key[ans_index]
    }

    fn step(key: &[bool; 512], image: &Array<bool>, background: bool) -> (Array<bool>, bool) {
        let new_len = (image.col_len() + 2) * (image.row_len + 2);
        let mut new_elts = Vec::with_capacity(new_len);
        new_elts.resize(new_len, background);

        let mut result = Array {
            row_len: image.row_len + 2,
            elts: new_elts,
        };

        for col in 0..result.row_len {
            for row in 0..result.col_len() {
                result.set(row, col, pixel_at(&key, &image, row, col, background));
            }
        }

        (result, if background { key[511] } else { key[0] })
    }

    pub fn part_1(data: &Data) -> u64 {
        let (mut image, mut background) = step(&data.key, &data.image, false);
        for _ in 0..1 {
            let (new_image, new_background) = step(&data.key, &image, background);
            image = new_image;
            background = new_background;
        }
        image.elts.iter().filter(|&i| *i).count() as u64
    }

    pub fn part_2(data: &Data) -> u64 {
        let (mut image, mut background) = step(&data.key, &data.image, false);
        for _ in 0..49 {
            let (new_image, new_background) = step(&data.key, &image, background);
            image = new_image;
            background = new_background;
        }
        image.elts.iter().filter(|&i| *i).count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::day_20::*;

    static TEST_INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn part1_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_1(&data), 35);
    }

    #[test]
    fn part2_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_2(&data), 3351);
    }

    #[test]
    fn test_day_20() {
        let input = input();
        assert_eq!(part_1(&input), 5326);
        assert_eq!(part_2(&input), 17096);
    }
}
