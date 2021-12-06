pub mod day_6 {

    pub(crate) fn parse(s: &str) -> Vec<u8> {
        s.trim()
            .split(',')
            .map(str::parse::<u8>)
            .map(|i| i.unwrap())
            .collect()
    }

    pub fn input() -> Vec<u8> {
        parse(include_str!("../input.txt"))
    }

    fn tick(data: &mut Vec<u8>) {
        let mut new_fish = Vec::new();
        for fish in data.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish.push(8);
            } else {
                *fish -= 1;
            }
        }
        data.extend(new_fish);
    }

    pub(crate) fn execute(data: &[u8], count: u8) -> u64 {
        let mut data = data.to_vec();
        for _ in 0..count {
            tick(&mut data);
        }

        data.len() as u64
    }

    pub fn part_1(data: &[u8]) -> u64 {
        execute(data, 80)
    }

    fn fish_created_given_spawning(fish_cache: &mut [u64], days: u16) -> u64 {
        let possible_result = fish_cache[days as usize];
        if possible_result != 0 {
            return possible_result;
        }
        let result = if days > 9 {
            // We start spawning later...
            fish_created_given_spawning(fish_cache, days - 9)
            // and we create a fish now, which spawns eventually...
                + fish_created_given_spawning(fish_cache, days - 7)
        } else if days > 7 {
            fish_created_given_spawning(fish_cache, days - 7) + 1
        } else {
            2
        };
        fish_cache[days as usize] = result;
        result
    }

    // f(3, 18) = 4
    // f(4, 18) = 2 + 1 + 1
    fn fish_created(fish_cache: &mut [u64], start: u8, days: u16) -> u64 {
        if days == 0 {
            return 1;
        }
        fish_created_given_spawning(fish_cache, days - start as u16)
    }

    pub fn part_2(data: &[u8]) -> u64 {
        let mut fish_cache = vec![0; 257];
        data.iter()
            .map(|i| fish_created(&mut fish_cache, *i, 256))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::day_6::*;

    static TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(execute(&data, 18), 26);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_2(&data), 26984457539);
    }

    #[test]
    fn test_day_6() {
        let input = input();
        assert_eq!(part_1(&input), 365131);
        assert_eq!(part_2(&input), 1650309278600);
    }
}
