pub mod day_4 {

    #[derive(Clone, Debug)]
    pub struct Board {
        arr: [[(u8, bool); 5]; 5],
    }

    #[derive(Clone, Debug)]
    pub struct Data {
        boards: Vec<Board>,
        draws: Vec<u8>,
    }

    fn chomp_board<'a, I>(input: &mut I) -> Option<Board>
    where
        I: Iterator<Item = &'a str>,
    {
        match input.next() {
            None => None,
            Some("") => {
                let answer = [[(0, false); 5]; 5];
                for mut row in answer {
                    let numbers = input
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(str::parse::<u8>)
                        .map(|i| i.unwrap());
                    for (j, entry) in numbers.enumerate() {
                        row[j] = (entry, false);
                    }
                }
                Some(Board { arr: answer })
            }
            Some(l) => {
                panic!("Expected an empty line, got {}", l);
            }
        }
    }

    pub(crate) fn parse(s: &str) -> Data {
        let mut input = s.split('\n');
        if let Some(first_line) = input.next() {
            let draws = first_line
                .split(',')
                .map(str::parse::<u8>)
                .map(|i| i.unwrap())
                .collect::<Vec<u8>>();
            let mut boards = Vec::new();
            loop {
                match chomp_board(&mut input) {
                    None => {
                        return Data { boards, draws };
                    }
                    Some(board) => {
                        boards.push(board);
                    }
                }
            }
        } else {
            panic!("Unexpectedly no first line");
        }
    }

    pub fn input() -> Data {
        parse(include_str!("../input.txt"))
    }

    fn draw_one(number: u8, board: &mut Board) -> bool {
        for row in 0..5 {
            for col in 0..5 {
                if board.arr[row][col].0 == number {
                    board.arr[row][col].1 = true;
                    // Check row
                    let mut row_full = true;
                    for i in 0..5 {
                        row_full = row_full && board.arr[row][i].1;
                    }
                    if row_full {
                        return true;
                    }
                    // Check column
                    for i in 0..5 {
                        if !board.arr[i][col].1 {
                            return false;
                        }
                    }
                    return true;
                }
            }
        }
        false
    }

    /// Draw the given number, update the input boards, and return the
    /// winning one if one has just won.
    fn draw(number: u8, boards: &mut [Board], seen_arr: &mut [bool]) -> Option<usize> {
        let mut ans = None;
        for (i, (board, seen)) in boards.iter_mut().zip(seen_arr).enumerate() {
            if !*seen && draw_one(number, board) {
                *seen = true;
                ans = Some(i);
            }
        }
        ans
    }

    fn sum_unmarked(board: &Board) -> u32 {
        let mut ans = 0;
        for row in 0..5 {
            for col in 0..5 {
                let (elt, has_seen) = board.arr[row][col];
                if !has_seen {
                    ans += elt as u32;
                }
            }
        }
        ans
    }

    pub fn part_1(data: &Data) -> u32 {
        let mut data = data.clone();
        let mut seen = vec![false; data.boards.len()];
        for number in data.draws.iter() {
            match draw(*number, &mut data.boards, &mut seen) {
                None => {}
                Some(board) => {
                    return sum_unmarked(&data.boards[board]) * (*number as u32);
                }
            }
        }
        panic!("Expected a winning board");
    }

    pub fn part_2(mut data: Data) -> u32 {
        let mut seen: Vec<bool> = vec![false; data.boards.len()];
        let mut last_won = None;
        for number in data.draws.iter() {
            match draw(*number, &mut data.boards, &mut seen) {
                None => {}
                Some(board) => {
                    seen[board] = true;
                    last_won = Some((board, *number));
                    if seen.iter().all(|&i| i) {
                        return sum_unmarked(&data.boards[board]) * (*number as u32);
                    }
                }
            }
        }
        let (last_won_index, number) = last_won.unwrap();
        sum_unmarked(&data.boards[last_won_index]) * (number as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::day_4::*;

    static TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_1(&data), 4512);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_2(data), 1924);
    }

    #[test]
    fn test_day_4() {
        let input = input();
        assert_eq!(part_1(&input), 64084);
        assert_eq!(part_2(input), 12833);
    }
}
