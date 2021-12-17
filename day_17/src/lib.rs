pub mod day_17 {

    use std::cmp::{max, Ordering};

    pub struct Data<T> {
        min_x: T,
        min_y: T,
        max_x: T,
        max_y: T,
    }

    fn chomp_str<I>(chars: &mut I, expected: &str)
    where
        I: Iterator<Item = char>,
    {
        for e in expected.chars() {
            let actual = chars.next();
            match actual {
                None => {
                    panic!("Expected a char, got none");
                }
                Some(actual) => {
                    if actual != e {
                        panic!("Expected {}, got {}", e, actual);
                    }
                }
            }
        }
    }

    fn chomp_int<I>(chars: &mut I) -> i32
    where
        I: Iterator<Item = char>,
    {
        let fst = chars.next().unwrap();
        let (sign, start) = if fst == '-' {
            let fst = chars.next().unwrap();
            (-1, fst.to_digit(10).unwrap())
        } else {
            (1, fst.to_digit(10).unwrap())
        };
        let mut ans = start as i32;

        for c in chars {
            if ('0'..='9').contains(&c) {
                ans = ans * 10 + (c as u8 - b'0') as i32;
            } else {
                return ans * sign;
            }
        }

        ans * sign
    }

    pub(crate) fn parse(s: &str) -> Data<i32> {
        let mut chars = s.chars();
        chomp_str(&mut chars, "target area: x=");
        let min_x = chomp_int(&mut chars);
        chomp_str(&mut chars, ".");
        let max_x = chomp_int(&mut chars);
        chomp_str(&mut chars, " y=");
        let min_y = chomp_int(&mut chars);
        chomp_str(&mut chars, ".");
        let max_y = chomp_int(&mut chars);

        Data {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn input() -> Data<i32> {
        parse(include_str!("../input.txt"))
    }

    #[derive(Clone)]
    struct Vector<T> {
        x: T,
        y: T,
    }

    struct State<T> {
        position: Vector<T>,
        velocity: Vector<T>,
    }

    fn step(state: &State<i32>) -> State<i32> {
        State {
            position: Vector {
                x: state.position.x + state.velocity.x,
                y: state.position.y + state.velocity.y,
            },
            velocity: Vector {
                x: match 0.cmp(&state.velocity.x) {
                    Ordering::Less => state.velocity.x - 1,
                    Ordering::Greater => state.velocity.x + 1,
                    Ordering::Equal => 0,
                },
                y: state.velocity.y - 1,
            },
        }
    }

    fn is_in_area<T>(data: &Data<T>, position: &Vector<T>) -> bool
    where
        T: Ord,
    {
        data.min_x <= position.x
            && position.x <= data.max_x
            && data.min_y <= position.y
            && position.y <= data.max_y
    }

    fn hits_target(data: &Data<i32>, velocity: &Vector<i32>) -> Option<i32> {
        // Continue until the x-velocity becomes zero.
        let mut state = State {
            position: Vector { x: 0, y: 0 },
            velocity: velocity.clone(),
        };

        let mut max_y_coord = 0;
        let mut hit_area = false;

        while state.velocity.x != 0 || state.position.y >= data.min_y {
            state = step(&state);
            if !hit_area && is_in_area(data, &state.position) {
                hit_area = true;
            }
            max_y_coord = max(max_y_coord, state.position.y);
            if state.velocity.x > 0 && state.position.x > data.max_x {
                break;
            } else if state.velocity.x < 0 && state.position.x < data.min_x {
                break;
            }
        }

        if hit_area {
            Some(max_y_coord)
        } else {
            None
        }
    }

    fn sqrt(n: i32) -> i32 {
        (n as f32).sqrt() as i32
    }

    pub fn part_1(data: &Data<i32>) -> u32 {
        let mut max_y = 0;
        // Which x-direction do we need to fire in?
        if data.min_x > 0 {
            // If we start with x-velocity n, then we reach at most coordinate n(n+1)/2
            // so we only need to start from sqrt(2 v)
            for x in sqrt(2 * data.min_x)..=data.max_x {
                for y in data.min_y..=data.max_x {
                    match hits_target(data, &Vector { x, y }) {
                        None => {}
                        Some(max_y_new) => {
                            max_y = max(max_y_new, max_y);
                        }
                    }
                }
            }
        } else {
            panic!("Expected positive x - please implement if this happens");
        }

        max_y as u32
    }

    pub fn part_2(data: &Data<i32>) -> u64 {
        let mut count = 0;
        // Which x-direction do we need to fire in?
        if data.min_x > 0 {
            for x in sqrt(2 * data.min_x)..=data.max_x {
                for y in data.min_y..=data.max_x {
                    match hits_target(data, &Vector { x, y }) {
                        None => {}
                        Some(_) => {
                            count += 1;
                        }
                    }
                }
            }
        } else {
            panic!("Expected positive x - please implement if this happens");
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::day_17::*;

    static TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part1_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_1(&data), 45);
    }

    #[test]
    fn part2_known() {
        let data = parse(&TEST_INPUT);
        assert_eq!(part_2(&data), 112);
    }

    #[test]
    fn test_day_17() {
        let input = input();
        assert_eq!(part_1(&input), 2278);
        assert_eq!(part_2(&input), 996);
    }
}
