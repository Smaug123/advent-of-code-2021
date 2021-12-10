pub mod day_10 {

    pub(crate) fn parse(s: &str) -> Vec<&str> {
        s.trim().split('\n').collect()
    }

    pub fn input() -> Vec<&'static str> {
        parse(include_str!("../input.txt"))
    }

    const fn closing(c: char) -> Option<char> {
        if c == '[' {
            Some(']')
        } else if c == '{' {
            Some('}')
        } else if c == '(' {
            Some(')')
        } else if c == '<' {
            Some('>')
        } else {
            None
        }
    }

    /// Return Ok(first illegal char), or Err(ending) for an incomplete string.
    /// The function takes the stack with the earliest element being the first one pushed.
    fn syntax_error<F, T>(s: &str, incomplete_handle: F) -> Result<char, T>
    where
        F: FnOnce(Vec<char>) -> T,
    {
        let mut stack = Vec::new();
        for c in s.chars() {
            match closing(c) {
                Some(closing) => {
                    stack.push(closing);
                }
                None => match stack.pop() {
                    None => {
                        return Ok(c);
                    }
                    Some(popped) => {
                        if popped != c {
                            return Ok(c);
                        }
                    }
                },
            }
        }
        let answer = incomplete_handle(stack);
        Err(answer)
    }

    pub fn part_1(data: &[&str]) -> u32 {
        data.iter()
            .map(|line| match syntax_error(line, |_| {}) {
                Ok(')') => 3,
                Ok(']') => 57,
                Ok('}') => 1197,
                Ok('>') => 25137,
                Ok(_) => panic!("Oh no"),
                Err(_) => 0,
            })
            .sum()
    }

    pub fn part_2(data: &[&str]) -> u64 {
        let mut scores: Vec<u64> = data
            .iter()
            .filter_map(|line| {
                match syntax_error(line, |completion| {
                    Some(
                        completion
                            .iter()
                            .rev()
                            .map(|c| match c {
                                ')' => 1,
                                ']' => 2,
                                '}' => 3,
                                '>' => 4,
                                _ => {
                                    panic!("Unexpected");
                                }
                            })
                            .fold(0, |state, new_val| state * 5 + new_val),
                    )
                }) {
                    Ok(_) => None,
                    Err(v) => v,
                }
            })
            .collect();
        // This can be done in linear time using two heaps, but life is too short
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::day_10::*;

    static TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_1(&data), 26397);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT);

        assert_eq!(part_2(&data), 288957);
    }

    #[test]
    fn test_day_10() {
        let input = input();
        assert_eq!(part_1(&input), 392043);
        assert_eq!(part_2(&input), 1605968119);
    }
}
