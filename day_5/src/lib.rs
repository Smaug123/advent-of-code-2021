pub mod day_5 {

    use std::cmp::{max, min};
    use std::fmt;

    pub struct Coordinate {
        x: u16,
        y: u16,
    }

    pub struct Line {
        start: Coordinate,
        end: Coordinate,
    }

    fn parse_pair(s: &str) -> Coordinate {
        let mut iterator = s.split(',').map(str::parse::<u16>).map(|i| i.unwrap());
        let x = iterator.next().unwrap();
        let y = iterator.next().unwrap();
        match iterator.next() {
            None => {}
            Some(s) => panic!("Expected end of split, got {}", s),
        }
        Coordinate { x, y }
    }

    fn parse_line(s: &str) -> Line {
        let mut iterator = s.split_whitespace();
        let start = parse_pair(iterator.next().unwrap());
        match iterator.next() {
            None => panic!("Expected an arrow but the string ran out"),
            Some("->") => {}
            Some(s) => panic!("Expected an arrow, got {}", s),
        }
        let end = parse_pair(iterator.next().unwrap());
        match iterator.next() {
            None => {}
            Some(s) => panic!("Expected end of split, got {}", s),
        }
        Line { start, end }
    }

    pub(crate) fn parse(s: &str) -> Vec<Line> {
        s.trim().split('\n').map(parse_line).collect()
    }

    pub fn input() -> Vec<Line> {
        parse(include_str!("../input.txt"))
    }

    struct Board<T> {
        entries: Vec<Option<(T, Option<T>)>>,
        max_y: usize,
    }

    impl<T> Board<T> {
        fn new(max_x: usize, max_y: usize) -> Board<T> {
            let entries = std::iter::repeat_with(|| None)
                .take((max_x + 1) * (max_y + 1))
                .collect();
            Board { entries, max_y }
        }
    }

    fn elt_at<T>(board: &Board<T>, x: usize, y: usize) -> &Option<(T, Option<T>)> {
        &board.entries[board.max_y * x + y]
    }

    impl<T> fmt::Display for Board<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for x in 0..=(self.entries.len() / self.max_y) {
                for y in 0..=self.max_y {
                    let entry = elt_at(self, x, y);
                    match *entry {
                        None => {
                            write!(f, ".")?;
                        }
                        Some((_, None)) => {
                            write!(f, "1")?;
                        }
                        _ => {
                            write!(f, "2")?;
                        }
                    }
                }
                writeln!(f)?;
            }
            fmt::Result::Ok(())
        }
    }

    fn elt_at_mut<T>(board: &mut Board<T>, x: usize, y: usize) -> &mut Option<(T, Option<T>)> {
        &mut board.entries[board.max_y * x + y]
    }

    fn set_at<T>(board: &mut Board<T>, x: u16, y: u16, new_val: T)
    where
        T: Copy,
    {
        let entry = elt_at_mut(board, x as usize, y as usize);
        match *entry {
            None => {
                *entry = Some((new_val, None));
            }
            Some((existing, None)) => {
                *entry = Some((existing, Some(new_val)));
            }
            Some((_, Some(_))) => {}
        }
    }

    fn do_it(board: &mut Board<u16>, data: &[Line], enable_diagonal: bool) {
        for (line_num, line) in data.iter().enumerate() {
            if line.start.x == line.end.x {
                let bottom = min(line.start.y, line.end.y);
                let top = max(line.start.y, line.end.y);
                let x = line.start.x;
                for y in bottom..=top {
                    set_at(board, x, y, line_num as u16);
                }
            } else if line.start.y == line.end.y {
                let y = line.start.y;
                let bottom = min(line.start.x, line.end.x);
                let top = max(line.start.x, line.end.x);
                for x in bottom..=top {
                    set_at(board, x, y, line_num as u16);
                }
            } else if enable_diagonal {
                let (left_point, right_point) = if line.start.x < line.end.x {
                    (&line.start, &line.end)
                } else {
                    (&line.end, &line.start)
                };

                if left_point.y < right_point.y {
                    for i in 0..=right_point.y - left_point.y {
                        set_at(board, left_point.x + i, left_point.y + i, line_num as u16);
                    }
                } else {
                    for i in 0..=left_point.y - right_point.y {
                        set_at(board, left_point.x + i, left_point.y - i, line_num as u16);
                    }
                }
            }
        }
    }

    fn count_multiple<T>(board: &Board<T>) -> usize {
        board
            .entries
            .iter()
            .filter(|i| matches!(i, Some((_, Some(_)))))
            .count()
    }

    pub fn part_1(data: &[Line]) -> usize {
        let max_x = data
            .iter()
            .map(|line| max(line.start.x, line.end.x))
            .max()
            .unwrap() as usize;
        let max_y = data
            .iter()
            .map(|line| max(line.start.y, line.end.y))
            .max()
            .unwrap() as usize;
        let mut board = Board::new(max_x, max_y);

        do_it(&mut board, data, false);
        count_multiple(&board)
    }

    pub fn part_2(data: &[Line]) -> usize {
        let max_x = data
            .iter()
            .map(|line| max(line.start.x, line.end.x))
            .max()
            .unwrap() as usize;
        let max_y = data
            .iter()
            .map(|line| max(line.start.y, line.end.y))
            .max()
            .unwrap() as usize;
        let mut board = Board::new(max_x, max_y);

        do_it(&mut board, data, true);
        count_multiple(&board)
    }
}

#[cfg(test)]
mod tests {
    use super::day_5::*;

    static TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_1(&data), 5);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_2(&data), 12);
    }

    #[test]
    fn test_day_5() {
        let input = input();
        assert_eq!(part_1(&input), 7318);
        assert_eq!(part_2(&input), 19939);
    }
}
