pub mod day_18 {

    #[derive(Debug, PartialEq, Eq)]
    pub enum PairEntry {
        Number(u8),
        Pair((usize, usize)),
    }

    // Incredibly sad phantom lifetype parameter so that IntoIterator works
    #[derive(Debug, PartialEq, Eq)]
    pub struct Pair<'a> {
        pairs: Vec<PairEntry>,
        first: usize,
        phantom: std::marker::PhantomData<&'a ()>,
    }

    pub struct PairIterator<'a> {
        current_pos: Vec<usize>,
        pairs: &'a [PairEntry],
    }

    impl<'a> Iterator for PairIterator<'a> {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }

    impl<'a> IntoIterator for Pair<'a> {
        type Item = u8;
        type IntoIter = PairIterator<'a>;

        fn into_iter(self) -> PairIterator<'a> {
            PairIterator {
                current_pos: vec![self.first],
                pairs: &self.pairs,
            }
        }
    }

    impl std::fmt::Display for Pair<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut print_stack = vec![Ok(self.first)];
            while let Some(to_print) = print_stack.pop() {
                match to_print {
                    Ok(index) => match self.pairs[index] {
                        PairEntry::Number(n) => {
                            write!(f, "{}", n)?;
                        }
                        PairEntry::Pair((p1, p2)) => {
                            write!(f, "[")?;
                            print_stack.push(Err(']'));
                            print_stack.push(Ok(p2));
                            print_stack.push(Err(','));
                            print_stack.push(Ok(p1));
                        }
                    },
                    Err(c) => {
                        write!(f, "{}", c)?;
                    }
                }
            }

            Ok(())
        }
    }

    pub(crate) fn concatenate<'a>(p1: &Pair<'a>, p2: &Pair<'a>) -> Pair<'a> {
        let mut pairs = Vec::with_capacity(p1.pairs.len() + p2.pairs.len() + 1);
        for entry in p1.pairs.iter() {
            match entry {
                PairEntry::Number(i) => {
                    pairs.push(PairEntry::Number(*i));
                }
                PairEntry::Pair((fst, snd)) => {
                    pairs.push(PairEntry::Pair((*fst, *snd)));
                }
            }
        }

        for entry in p2.pairs.iter() {
            match entry {
                PairEntry::Number(i) => {
                    pairs.push(PairEntry::Number(*i));
                }
                PairEntry::Pair((fst, snd)) => {
                    pairs.push(PairEntry::Pair((
                        *fst + p1.pairs.len(),
                        *snd + p1.pairs.len(),
                    )));
                }
            }
        }

        pairs.push(PairEntry::Pair((p1.pairs.len() - 1, pairs.len() - 1)));
        let first = pairs.len() - 1;

        Pair {
            pairs,
            first,
            phantom: std::marker::PhantomData,
        }
    }

    pub(crate) fn parse_line(s: &str) -> Pair {
        let mut chars = s.chars();

        match chars.next() {
            Some('[') => {}
            Some(c) => {
                panic!("Expected open '[', got {}", c);
            }
            None => {
                panic!("Expected open '[', got nothing");
            }
        }
        let mut open_stack = 1;
        let mut pair_entries = Vec::new();
        let mut unclaimed_entries = Vec::new();

        while open_stack > 0 {
            match chars.next().unwrap() {
                ']' => {
                    open_stack -= 1;
                    let pair_1 = unclaimed_entries.pop().unwrap();
                    let pair_2 = unclaimed_entries.pop().unwrap();
                    pair_entries.push(PairEntry::Pair((pair_2, pair_1)));
                    unclaimed_entries.push(pair_entries.len() - 1);
                }
                '[' => {
                    open_stack += 1;
                }
                ',' => {}
                c => {
                    if ('0'..='9').contains(&c) {
                        pair_entries.push(PairEntry::Number(c as u8 - b'0'));
                        unclaimed_entries.push(pair_entries.len() - 1);
                    } else {
                        panic!("Unrecognised character: {}", c);
                    }
                }
            }
        }

        let first = pair_entries.len() - 1;
        Pair {
            pairs: pair_entries,
            first: first,
            phantom: std::marker::PhantomData,
        }
    }

    pub(crate) fn parse(s: &str) -> Vec<Pair> {
        s.split('\n').map(parse_line).collect()
    }

    pub fn input() -> Vec<Pair<'static>> {
        parse(include_str!("../input.txt"))
    }

    fn first_to_left(p: &Pair, entry: usize) -> Option<usize> {
        todo!()
    }

    pub(crate) fn explode(p1: &mut Pair, entry: usize) {}

    pub(crate) fn split(p1: &mut Pair, entry: usize) {
        match p1.pairs[entry] {
            PairEntry::Pair(_) => {
                panic!("Can't split a pair!");
            }
            PairEntry::Number(n) => {
                let left = n / 2;
                let right = n - left;
                p1.pairs.push(PairEntry::Number(left));
                p1.pairs.push(PairEntry::Number(right));
                p1.pairs[entry] = PairEntry::Pair((p1.pairs.len() - 2, p1.pairs.len() - 1));
            }
        }
    }

    pub(crate) fn add<'a>(p1: &Pair<'a>, p2: &Pair<'a>) -> Pair<'a> {
        let concatenated = concatenate(p1, p2);
        todo!()
    }

    pub(crate) fn magnitude(p1: &Pair) -> u32 {
        panic!("");
    }

    pub(crate) fn final_sum<'a>(p1: &[Pair<'a>]) -> Pair<'a> {
        panic!("");
    }

    pub fn part_1(data: &[Pair]) -> u32 {
        println!("{:?}", data);
        panic!("TODO");
    }

    pub fn part_2(data: &[Pair]) -> u64 {
        panic!("TODO");
    }
}

#[cfg(test)]
mod tests {
    use super::day_18::*;

    static TEST_INPUT_1: &str = "[1,1]
[2,2]
[3,3]
[4,4]";

    static TEST_INPUT_2: &str = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]";

    static TEST_INPUT_3: &str = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";

    static TEST_INPUT_4: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

    #[test]
    fn test_parse() {
        let test_cases = [
            "[[[[4,3],4],4],[7,[[8,4],9]]]",
            "[1,1]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        ];
        for &case in test_cases.iter() {
            assert_eq!(format!("{}", parse_line(case)), case);
        }
    }

    #[test]
    fn test_concatenate() {
        let actual = concatenate(&parse_line("[1,2]"), &parse_line("[[3,4],5]"));
        let expected = parse_line("[[1,2],[[3,4],5]]");
        assert_eq!(format!("{}", actual), format!("{}", expected));
    }

    #[test]
    fn part1_known() {
        let lhs = parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let rhs = parse_line("[1,1]");
        let answer = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(add(&lhs, &rhs), answer);

        assert_eq!(
            final_sum(&parse(&TEST_INPUT_1)),
            parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        );
        assert_eq!(
            final_sum(&parse(&TEST_INPUT_2)),
            parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]")
        );
        assert_eq!(
            final_sum(&parse(&TEST_INPUT_3)),
            parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]")
        );
        assert_eq!(
            final_sum(&parse(&TEST_INPUT_4)),
            parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );

        assert_eq!(magnitude(&parse_line("[9,1]")), 29);
        assert_eq!(magnitude(&parse_line("[[9,1],[1,9]]")), 129);

        assert_eq!(magnitude(&parse_line("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(
            magnitude(&parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
            1384
        );
        assert_eq!(magnitude(&parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
        assert_eq!(magnitude(&parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]")), 791);
        assert_eq!(
            magnitude(&parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]")),
            1137
        );
        assert_eq!(
            magnitude(&parse_line(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
            3488
        );

        assert_eq!(
            part_1(&parse(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            )),
            4140
        );
    }

    #[test]
    fn part2_known() {
        //let data = parse(&TEST_INPUT);
        //assert_eq!(part_2(&data), 112);
    }

    #[test]
    fn test_day_18() {
        let input = input();
        assert_eq!(part_1(&input), 4057);
        assert_eq!(part_2(&input), 4683);
    }
}
