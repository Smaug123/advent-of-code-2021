pub mod day_15 {

    use ::array::array::*;
    use std::cmp::min;

    fn surrounding<T>(data: &Array<T>, row: usize, col: usize, output: &mut Vec<(usize, usize)>) {
        if row > 0 {
            output.push((row - 1, col));
        }
        if row < data.col_len() - 1 {
            output.push((row + 1, col));
        }
        if col > 0 {
            output.push((row, col - 1));
        }
        if col < data.row_len() - 1 {
            output.push((row, col + 1));
        }
    }

    fn solve(data: &Array<u8>) -> u32 {
        let mut visited: Array<bool> = Array::make_default(data.row_len(), data.col_len(), false);

        let mut distances: Array<u32> =
            Array::make_default(data.row_len(), data.col_len(), u32::MAX);

        let mut to_visit = vec![(0, 0)];

        distances.set(0, 0, 0);

        let mut where_to_go = Vec::with_capacity(4);

        'outer: while let Some((current_row, current_col)) = to_visit.pop() {
            if current_row + 1 == data.col_len() && current_col + 1 == data.row_len() {
                return *distances.get_unsafe(current_row, current_col);
            }

            match visited.get_mut(current_row, current_col) {
                None => {
                    panic!("Bounds error");
                }
                Some(slot) => {
                    if *slot {
                        continue 'outer;
                    } else {
                        *slot = true;
                    }
                }
            }

            let current_distance = *distances.get_unsafe(current_row, current_col);

            where_to_go.resize(0, (0, 0));
            surrounding(data, current_row, current_col, &mut where_to_go);

            for &(row, col) in where_to_go.iter() {
                if !visited.get_unsafe(row, col) {
                    to_visit.push((row, col));
                    let distance =
                        data.get_unsafe(row, col) + data.get_unsafe(current_row, current_col);
                    let distance_via_here = current_distance + distance as u32;
                    distances.apply_at(row, col, |i| min(distance_via_here, i))
                }
            }

            to_visit.sort_by_key(|v| u32::MAX - distances.get_unsafe(v.0, v.1));
        }

        panic!("Should have finished");
    }

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
        let edge_weight = solve(data);

        let first_node = *data.get_unsafe(0, 0) as u32;
        let last_node = *data.get_unsafe(data.row_len() - 1, data.col_len() - 1) as u32;

        (edge_weight + first_node + last_node) / 2 - first_node
    }

    pub(crate) fn quintuple(arr: &Array<u8>) -> Array<u8> {
        let row_len = 5 * arr.row_len();
        let elts = vec![0u8; row_len * row_len];
        let mut result = Array::make(elts, row_len);

        for row in 0..arr.col_len() {
            for col in 0..arr.row_len() {
                for x in 0..5 {
                    for y in 0..5 {
                        result.set(
                            arr.row_len() * x + row,
                            arr.col_len() * y + col,
                            (arr.get_unsafe(row, col) + (x as u8) + (y as u8) - 1) % 9 + 1,
                        );
                    }
                }
            }
        }

        result
    }

    pub fn part_2(data: &Array<u8>) -> u32 {
        let data = quintuple(data);
        let edge_weight = solve(&data);

        let first_node = *data.get_unsafe(0, 0) as u32;
        let last_node = *data.get_unsafe(data.row_len() - 1, data.col_len() - 1) as u32;

        (edge_weight + first_node + last_node) / 2 - first_node
    }
}

#[cfg(test)]
mod tests {
    use super::day_15::*;

    static TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);
        assert_eq!(part_1(&data), 40);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);
        assert_eq!(part_2(&data), 315);
    }

    #[test]
    fn test_day_15() {
        let input = input();
        assert_eq!(part_1(&input), 523);
        assert_eq!(part_2(&input), 2876);
    }
}
