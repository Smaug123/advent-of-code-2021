pub mod day_21 {

    use std::cmp::max;

    fn chomp_str<I>(input: &mut I, expected: &str)
    where
        I: Iterator<Item = char>,
    {
        for c in expected.chars() {
            let other = input.next().unwrap();
            if other != c {
                panic!("Expected {}, got {}", c, other);
            }
        }
    }

    fn to_int<I>(input: &mut I) -> u8
    where
        I: Iterator<Item = char>,
    {
        input.fold(0, |state, c| state * 10 + ((c as u8) - b'0'))
    }

    pub(crate) fn parse(s: &str) -> (u8, u8) {
        let mut lines = s.trim_end().split('\n');
        let first_line = lines.next().unwrap();
        let second_line = lines.next().unwrap();
        match lines.next() {
            None => {}
            Some(l) => {
                panic!("Unexpected line: {}", l);
            }
        }
        let mut chars = first_line.chars();
        chomp_str(&mut chars, "Player 1 starting position: ");
        let first = to_int(&mut chars);

        let mut chars = second_line.chars();
        chomp_str(&mut chars, "Player 2 starting position: ");
        let second = to_int(&mut chars);

        (first, second)
    }

    #[derive(Debug)]
    struct GameState {
        player_1_score: u16,
        player_2_score: u16,
        player_1_pos: u8,
        player_2_pos: u8,
        die: u8,
        die_count: u16,
    }

    // The bool is true if player 1 won, false if player 2 won.
    fn turn(state: GameState, top_score: u16) -> Result<(GameState, bool), GameState> {
        let to_move = 3 * (state.die as u16) + 3;
        let new_die = (state.die + 2) % 100 + 1;

        let player_1_pos = ((state.player_1_pos as u16 + to_move - 1) % 10) as u8 + 1;
        let player_1_score = state.player_1_score + player_1_pos as u16;

        if player_1_score >= 1000 {
            return Ok((
                GameState {
                    player_1_score,
                    player_2_score: state.player_2_score,
                    player_1_pos,
                    player_2_pos: state.player_2_pos,
                    die: new_die,
                    die_count: state.die_count + 3,
                },
                true,
            ));
        }

        let to_move = 3 * (new_die as u16) + 3;
        let new_die = (new_die + 2) % 100 + 1;

        let player_2_pos = ((state.player_2_pos as u16 + to_move - 1) % 10) as u8 + 1;
        let player_2_score = state.player_2_score + player_2_pos as u16;

        let new_state = GameState {
            player_1_score,
            player_2_score,
            player_1_pos,
            player_2_pos,
            die: new_die,
            die_count: state.die_count + 6,
        };

        if player_2_score > top_score {
            Ok((new_state, false))
        } else {
            Err(new_state)
        }
    }

    pub fn input() -> (u8, u8) {
        parse(include_str!("../input.txt"))
    }

    pub fn part_1(data: &(u8, u8)) -> u64 {
        let mut state = GameState {
            player_1_score: 0,
            player_2_score: 0,
            player_1_pos: data.0,
            player_2_pos: data.1,
            die: 1,
            die_count: 0,
        };

        loop {
            match turn(state, 1000) {
                Err(new_state) => {
                    state = new_state;
                }
                Ok((state, player_1_won)) => {
                    return (if player_1_won {
                        state.player_2_score
                    } else {
                        state.player_1_score
                    }) as u64
                        * state.die_count as u64;
                }
            }
        }
    }

    struct Store {
        per_position: Vec<(u64, u64)>,
    }

    impl Store {
        fn make() -> Store {
            Store {
                per_position: vec![(0, 0); 21 * 21 * 10 * 10],
            }
        }

        fn get(&self, pos1: u8, pos2: u8, score1: u8, score2: u8) -> Option<(u64, u64)> {
            match self.per_position[(pos1 - 1) as usize
                + (pos2 - 1) as usize * 10
                + score1 as usize * 100
                + score2 as usize * 2100]
            {
                (0, 0) => None,
                v => Some(v),
            }
        }

        fn set(&mut self, pos1: u8, pos2: u8, score1: u8, score2: u8, value: (u64, u64)) {
            self.per_position[(pos1 - 1) as usize
                + (pos2 - 1) as usize * 10
                + score1 as usize * 100
                + score2 as usize * 2100] = value;
        }
    }

    const TUPLES: [(u8, u8, u8); 27] = [
        (1, 1, 1),
        (1, 1, 2),
        (1, 1, 3),
        (1, 2, 1),
        (1, 2, 2),
        (1, 2, 3),
        (1, 3, 1),
        (1, 3, 2),
        (1, 3, 3),
        (2, 1, 1),
        (2, 1, 2),
        (2, 1, 3),
        (2, 2, 1),
        (2, 2, 2),
        (2, 2, 3),
        (2, 3, 1),
        (2, 3, 2),
        (2, 3, 3),
        (3, 1, 1),
        (3, 1, 2),
        (3, 1, 3),
        (3, 2, 1),
        (3, 2, 2),
        (3, 2, 3),
        (3, 3, 1),
        (3, 3, 2),
        (3, 3, 3),
    ];
    const fn sum(t: &(u8, u8, u8)) -> u8 {
        t.0 + t.1 + t.2
    }

    fn part_2_inner(store: &mut Store, pos1: u8, pos2: u8, score1: u8, score2: u8) -> (u64, u64) {
        if let Some(answer) = store.get(pos1, pos2, score1, score2) {
            return answer;
        }

        let answer = TUPLES
            .iter()
            .map(|t| {
                let final_pos = (pos1 + sum(t) - 1) % 10 + 1;
                let score1 = score1 + final_pos;
                if score1 >= 21 {
                    (1, 0)
                } else {
                    let (sub2, sub1) = part_2_inner(store, pos2, final_pos, score2, score1);
                    (sub1, sub2)
                }
            })
            .fold((0, 0), |(state1, state2), (v1, v2)| {
                (state1 + v1, state2 + v2)
            });

        store.set(pos1, pos2, score1, score2, answer);
        answer
    }

    pub fn part_2(data: &(u8, u8)) -> u64 {
        let (player1, player2) = part_2_inner(&mut Store::make(), data.0, data.1, 0, 0);
        max(player1, player2)
    }
}

#[cfg(test)]
mod tests {
    use super::day_21::*;

    static TEST_INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn part1_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_1(&data), 739785);
    }

    #[test]
    fn part2_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_2(&data), 444356092776315);
    }

    #[test]
    fn test_day_21() {
        let input = input();
        assert_eq!(part_1(&input), 906093);
        assert_eq!(part_2(&input), 274291038026362);
    }
}
