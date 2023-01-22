pub mod day_25 {

    use ::array::array::*;

    #[derive(Clone)]
    pub enum Square {
        Down,
        Right,
        Empty,
    }

    impl std::fmt::Display for Square {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Square::Down => {
                    write!(f, "v")?;
                }
                Square::Right => {
                    write!(f, ">")?;
                }
                Square::Empty => {
                    write!(f, ".")?;
                }
            }
            Ok(())
        }
    }

    impl std::fmt::Debug for Square {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Square::Down => {
                    write!(f, "v")?;
                }
                Square::Right => {
                    write!(f, ">")?;
                }
                Square::Empty => {
                    write!(f, ".")?;
                }
            }
            Ok(())
        }
    }

    fn parse_char(c: char) -> Square {
        match c {
            'v' => Square::Down,
            '>' => Square::Right,
            '.' => Square::Empty,
            _ => {
                panic!("Expected a square, got {}", c);
            }
        }
    }

    pub(crate) fn parse(s: &str) -> Array<Square> {
        let lines = s.trim_end().split('\n');

        Array::from_rows(lines.map(|line| line.chars().map(parse_char)))
    }

    pub fn input() -> Array<Square> {
        parse(include_str!("../input.txt"))
    }

    fn move_east(arr: &Array<Square>, output: &mut Array<Square>) -> bool {
        let mut moved = false;
        for row in 0..arr.col_len() {
            for col in 0..arr.row_len() {
                if let Square::Right = arr.get_unsafe(row, col) {
                    let dest_col = if col == arr.row_len() - 1 { 0 } else { col + 1 };
                    if let Square::Empty = arr.get_unsafe(row, dest_col) {
                        output.set(row, col, Square::Empty);
                        output.set(row, dest_col, Square::Right);
                        moved = true;
                    }
                }
            }
        }
        moved
    }

    fn move_south(arr: &Array<Square>, output: &mut Array<Square>) -> bool {
        let mut moved = false;
        for row in 0..arr.col_len() {
            for col in 0..arr.row_len() {
                let dest_row = if row == arr.col_len() - 1 { 0 } else { row + 1 };

                if let Square::Down = arr.get_unsafe(row, col) {
                    if let Square::Empty = arr.get_unsafe(dest_row, col) {
                        output.set(row, col, Square::Empty);
                        output.set(dest_row, col, Square::Down);
                        moved = true;
                    }
                }
            }
        }
        moved
    }

    fn step(arr: &mut Array<Square>) -> bool {
        let mut output = arr.clone();
        let moved = move_east(arr, &mut output);
        for row in 0..arr.col_len() {
            for col in 0..arr.row_len() {
                arr.set(row, col, output.get_unsafe(row, col).clone());
            }
        }
        move_south(&output, arr) || moved
    }

    pub fn part_1(data: &Array<Square>) -> u64 {
        let mut arr = data.clone();
        let mut count = 0;
        let mut changed = true;
        while changed {
            count += 1;
            changed = step(&mut arr);
        }

        count
    }
}
#[cfg(test)]
mod tests {
    use super::day_25::*;

    static TEST_INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn part1_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_1(&data), 58);
    }

    #[test]
    fn test_day_25() {
        let input = input();
        assert_eq!(part_1(&input), 389);
    }
}
