pub mod day_2 {

    pub enum Direction {
        Forward,
        Down,
        Up,
    }

    pub struct Movement {
        direction: Direction,
        distance: u32,
    }

    fn chomp<I>(input: &mut I, s: &str)
    where
        I: Iterator<Item = char>,
    {
        for expected in s.chars() {
            if input.next() != Some(expected) {
                panic!("Unexpected character");
            }
        }
    }

    fn parse_int<I>(input: &mut I) -> u32
    where
        I: Iterator<Item = char>,
    {
        let mut answer = 0;
        for c in input {
            answer = answer * 10 + char::to_digit(c, 10).unwrap();
        }
        answer
    }

    pub fn parse_movement(s: &str) -> Movement {
        let mut iter = s.chars();
        let direction = match iter.next() {
            None => {
                panic!("Expected a nonempty string");
            }
            Some('f') => {
                chomp(&mut iter, "orward ");
                Direction::Forward
            }
            Some('u') => {
                chomp(&mut iter, "p ");
                Direction::Up
            }
            Some('d') => {
                chomp(&mut iter, "own ");
                Direction::Down
            }
            Some(c) => {
                panic!("Unknown character: {}", c);
            }
        };
        Movement {
            direction,
            distance: parse_int(&mut iter),
        }
    }

    pub fn input() -> Vec<Movement> {
        let input = include_str!("../input.txt");
        input
            .trim()
            .split('\n')
            .map(parse_movement)
            .collect::<Vec<Movement>>()
    }

    pub struct Position {
        x: u32,
        y: u32,
    }

    const fn step(position: Position, movement: &Movement) -> Position {
        match movement.direction {
            Direction::Forward => Position {
                x: position.x + movement.distance,
                y: position.y,
            },
            Direction::Up => Position {
                x: position.x,
                y: position.y - movement.distance,
            },
            Direction::Down => Position {
                x: position.x,
                y: position.y + movement.distance,
            },
        }
    }

    pub fn part_1(movements: &[Movement]) -> u32 {
        let start_pos = Position { x: 0, y: 0 };
        let final_pos = movements.iter().fold(start_pos, |pos, dir| step(pos, dir));
        final_pos.x * final_pos.y
    }

    pub struct Position2 {
        x: u32,
        depth: u32,
        aim: u32,
    }

    const fn step_2(position: Position2, movement: &Movement) -> Position2 {
        match movement.direction {
            Direction::Forward => Position2 {
                aim: position.aim,
                x: position.x + movement.distance,
                depth: position.depth + movement.distance * position.aim,
            },
            Direction::Up => Position2 {
                x: position.x,
                depth: position.depth,
                aim: position.aim - movement.distance,
            },
            Direction::Down => Position2 {
                x: position.x,
                depth: position.depth,
                aim: position.aim + movement.distance,
            },
        }
    }

    pub fn part_2(movements: &[Movement]) -> u32 {
        let start_pos = Position2 {
            x: 0,
            depth: 0,
            aim: 0,
        };
        let final_pos = movements
            .iter()
            .fold(start_pos, |pos, dir| step_2(pos, dir));
        final_pos.x * final_pos.depth
    }
}

#[cfg(test)]
mod tests {
    use super::day_2::*;

    #[test]
    fn part1_known() {
        let input = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .map(parse_movement);

        assert_eq!(part_1(&input), 150);
    }

    #[test]
    fn part2_known() {
        let input = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .map(parse_movement);
        println!("{}", part_2(&input));
        assert_eq!(part_2(&input), 900);
    }

    #[test]
    fn test_day_2() {
        let input = input();
        assert_eq!(part_1(&input), 1451208);
        assert_eq!(part_2(&input), 1620141160);
    }
}
