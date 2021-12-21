pub mod day_13 {

    use ::array::array::*;
    use std::cmp::max;
    use std::fmt;

    #[derive(Debug, Clone)]
    pub enum Axis {
        X,
        Y,
    }

    impl std::fmt::Display for Axis {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Axis::X => {
                    write!(f, "X")
                }
                Axis::Y => {
                    write!(f, "Y")
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Data {
        paper: Array<bool>,
        reversed_fold_list: Vec<(Axis, u16)>,
    }

    impl std::fmt::Display for Data {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "{}", self.paper)?;

            for (axis, value) in self.reversed_fold_list.iter().rev() {
                writeln!(f, "{}={}", axis, value)?;
            }

            fmt::Result::Ok(())
        }
    }

    fn chomp_int<I>(iter: &mut I) -> Option<u16>
    where
        I: Iterator<Item = char>,
    {
        let mut answer = 0;
        let mut is_empty = true;
        loop {
            match iter.next() {
                None => {
                    return if is_empty { None } else { Some(answer) };
                }
                Some(c) => match c.to_digit(10) {
                    None => {
                        return Some(answer);
                    }
                    Some(d) => {
                        answer = answer * 10 + (d as u16);
                    }
                },
            }
            is_empty = false;
        }
    }

    fn chomp_string<I>(iter: &mut I, expected: &str)
    where
        I: Iterator<Item = char>,
    {
        for c in expected.chars() {
            match iter.next() {
                None => {
                    panic!("Expected '{}', got nothing", c);
                }
                Some(actual) => {
                    if actual != c {
                        panic!("Expected '{}', got '{}'", c, actual);
                    }
                }
            }
        }
    }

    fn should_end<T, I>(iter: &mut I)
    where
        I: Iterator<Item = T>,
        T: std::fmt::Display,
    {
        match iter.next() {
            None => {}
            Some(c) => {
                panic!("Expected end of iterator, got {}", c);
            }
        }
    }

    pub(crate) fn parse(s: &str) -> Data {
        let mut coordinates = Vec::new();
        let mut lines = s.split('\n');

        let mut max_x = 0;
        let mut max_y = 0;

        // Consume coordinates
        for line in &mut lines {
            let mut chars = line.chars();
            match chomp_int(&mut chars) {
                None => {
                    // Move to consuming folds
                    break;
                }
                Some(x) => {
                    if x > max_x {
                        max_x = x;
                    }
                    let y = chomp_int(&mut chars).unwrap();
                    if y > max_y {
                        max_y = y;
                    }
                    coordinates.push((x, y));
                    should_end(&mut chars);
                }
            }
        }

        let mut folds = Vec::new();
        // Consume folds
        for line in lines {
            let mut chars = line.chars();
            chomp_string(&mut chars, "fold along ");
            let axis = match chars.next() {
                Some('x') => Axis::X,
                Some('y') => Axis::Y,
                None => {
                    panic!("Expected an axis, got nothing");
                }
                Some(c) => {
                    panic!("Expected an axis, got '{}'", c);
                }
            };
            chomp_string(&mut chars, "=");
            let value = chomp_int(&mut chars).unwrap();
            should_end(&mut chars);
            folds.push((axis, value));
        }

        let row_len = max(
            max_x as usize + 1,
            folds
                .iter()
                .filter_map(|(axis, v)| match axis {
                    Axis::X => Some(*v * 2),
                    Axis::Y => None,
                })
                .max()
                .unwrap() as usize
                + 1,
        );
        let col_len = max(
            max_y as usize + 1,
            folds
                .iter()
                .filter_map(|(axis, v)| match axis {
                    Axis::X => None,
                    Axis::Y => Some(*v * 2),
                })
                .max()
                .unwrap() as usize
                + 1,
        );

        let elts = (0..(row_len * col_len)).map(|_| false).collect();

        let mut paper = Array::make(elts, row_len as usize);

        for (col, row) in coordinates.iter() {
            paper.set(*row as usize, *col as usize, true);
        }

        folds.reverse();

        Data {
            reversed_fold_list: folds,
            paper,
        }
    }

    pub fn input() -> Data {
        parse(include_str!("../input.txt"))
    }

    fn fold_once(data: &mut Data) -> Option<()> {
        let (axis, value) = data.reversed_fold_list.pop()?;
        let value = value as usize;

        match axis {
            Axis::X => {
                for col in (value + 1)..=(2 * value) {
                    for row in 0..data.paper.col_len() {
                        let new_value = *data.paper.get_unsafe(row, 2 * value - col)
                            || *data.paper.get_unsafe(row, col);
                        data.paper.set(row, 2 * value - col, new_value);
                        data.paper.set(row, col, false);
                    }
                }
            }

            Axis::Y => {
                for row in (value + 1)..=(2 * value) {
                    for col in 0..data.paper.row_len() {
                        let value_one = *data.paper.get_unsafe(2 * value - row, col);
                        let value_two = *data.paper.get_unsafe(row, col);
                        let new_value = value_one || value_two;
                        data.paper.set(2 * value - row, col, new_value);
                        data.paper.set(row, col, false);
                    }
                }
            }
        }

        Some(())
    }

    pub fn part_1(data: &Data) -> u32 {
        let mut data = data.clone();
        fold_once(&mut data).unwrap();

        data.paper.iter().fold(0, |i, &v| if v { i + 1 } else { i })
    }

    pub fn part_2(data: &Data) -> String {
        let mut data = data.clone();
        let max_x_fold = data
            .reversed_fold_list
            .iter()
            .filter_map(|(axis, v)| match axis {
                Axis::X => Some(*v),
                Axis::Y => None,
            })
            .next()
            .unwrap();
        let max_y_fold = data
            .reversed_fold_list
            .iter()
            .filter_map(|(axis, v)| match axis {
                Axis::X => None,
                Axis::Y => Some(*v),
            })
            .next()
            .unwrap();

        while let Some(()) = fold_once(&mut data) {}

        // +1 for the newlines
        let mut result_str = String::with_capacity(((max_x_fold + 1) * max_y_fold) as usize);

        for row in 0..max_y_fold {
            for col in 0..max_x_fold {
                let character = if *data.paper.get_unsafe(row as usize, col as usize) {
                    '#'
                } else {
                    '.'
                };
                result_str.push(character);
            }
            result_str.push('\n');
        }

        result_str
    }
}

#[cfg(test)]
mod tests {
    use super::day_13::*;

    static TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);
        assert_eq!(part_1(&data), 17);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);
        let expected = "#####
#...#
#...#
#...#
#####
.....
.....\n";
        assert_eq!(part_2(&data), expected);
    }

    #[test]
    fn test_day_13() {
        let input = input();
        assert_eq!(part_1(&input), 693);
        let expected = "#..#..##..#....####.###...##..####.#..#.
#..#.#..#.#.......#.#..#.#..#....#.#..#.
#..#.#....#......#..#..#.#..#...#..#..#.
#..#.#....#.....#...###..####..#...#..#.
#..#.#..#.#....#....#.#..#..#.#....#..#.
.##...##..####.####.#..#.#..#.####..##..\n";
        assert_eq!(part_2(&input), expected);
    }
}
