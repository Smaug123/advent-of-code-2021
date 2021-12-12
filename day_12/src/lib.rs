pub mod day_12 {

    use std::collections::{HashMap, HashSet};

    #[derive(Debug)]
    pub struct Cave<'a> {
        is_big: bool,
        name: &'a str,
    }

    #[derive(Debug)]
    pub struct CaveSystem<'a> {
        caves: Vec<Cave<'a>>,
        /// The 0th element is the set of caves you can hit from caves[0].
        edges: Vec<HashSet<u16>>,
        start: u16,
        end: u16,
    }

    fn insert_cave<'a>(
        caves: &mut CaveSystem<'a>,
        mapping: &mut HashMap<&'a str, u16>,
        name: &'a str,
    ) -> u16 {
        let index = match mapping.get(name) {
            None => {
                let is_big = name.chars().next().unwrap().is_uppercase();
                let i = caves.caves.len() as u16;
                mapping.insert(name, i);
                caves.caves.push(Cave { is_big, name });
                caves.edges.push(HashSet::new());
                if caves.start == 0 && name == "start" {
                    caves.start = i;
                } else if caves.end == 0 && name == "end" {
                    caves.end = i;
                }
                i
            }
            Some(i) => *i,
        };
        index
    }

    pub(crate) fn parse(s: &str) -> CaveSystem {
        let mut answer = CaveSystem {
            caves: Vec::new(),
            edges: Vec::new(),
            start: 0,
            end: 0,
        };
        let mut mapping: HashMap<&str, u16> = HashMap::new();
        for line in s.trim().split('\n') {
            let mut iter = line.split('-');
            let first_index = match iter.next() {
                None => {
                    panic!("oh no");
                }
                Some(name) => insert_cave(&mut answer, &mut mapping, name),
            };
            let second_index = match iter.next() {
                None => {
                    panic!("oh no");
                }
                Some(name) => insert_cave(&mut answer, &mut mapping, name),
            };
            match iter.next() {
                None => {}
                Some(rest) => {
                    panic!("Expected empty, got {}", rest);
                }
            }
            answer.edges[first_index as usize].insert(second_index);
            answer.edges[second_index as usize].insert(first_index);
        }
        answer
    }

    pub fn input() -> CaveSystem<'static> {
        parse(include_str!("../input.txt"))
    }

    pub fn part_1(data: &CaveSystem) -> u32 {
        let mut paths_count = 0;

        let mut visited: Vec<u8> = (0..data.caves.len()).map(|_| 0).collect();
        visited[data.start as usize] = 1;
        let mut stack: Vec<_> = data.edges[data.start as usize]
            .iter()
            .map(|i| (*i, visited.to_vec()))
            .collect();

        while let Some((current_node, visited)) = stack.pop() {
            if current_node == data.end {
                paths_count += 1;
            }
            let mut visited = visited.to_vec();
            visited[current_node as usize] += 1;
            stack.extend(
                data.edges[current_node as usize]
                    .iter()
                    .cloned()
                    .filter(|&node| data.caves[node as usize].is_big || visited[node as usize] == 0)
                    .map(|next_node| (next_node, visited.to_vec())),
            );
        }

        paths_count
    }

    pub fn part_2(data: &CaveSystem) -> u32 {
        let mut paths_count = 0;

        let mut visited: Vec<u8> = (0..data.caves.len()).map(|_| 0).collect();
        // Record that we can't visit start again
        visited[data.start as usize] = 2;
        let mut stack: Vec<_> = data.edges[data.start as usize]
            .iter()
            .map(|i| (*i, visited.to_vec(), false))
            .collect();

        while let Some((current_node, visited, small_visited_twice)) = stack.pop() {
            // We can only visit the end once.
            if current_node == data.end {
                paths_count += 1;
            } else {
                stack.extend(
                    data.edges[current_node as usize]
                        .iter()
                        .cloned()
                        .filter_map(|next_node| {
                            // We can visit the node if it is big...
                            if data.caves[next_node as usize].is_big {
                                let mut visited = visited.to_vec();
                                visited[current_node as usize] += 1;
                                Some((next_node, visited, small_visited_twice))
                            } else {
                                // It's small, but we might still be able to visit it
                                match visited[next_node as usize] {
                                    0 => {
                                        let mut visited = visited.to_vec();
                                        visited[current_node as usize] += 1;
                                        Some((next_node, visited, small_visited_twice))
                                    }
                                    1 => {
                                        if !small_visited_twice {
                                            let mut visited = visited.to_vec();
                                            visited[current_node as usize] += 1;
                                            Some((next_node, visited, true))
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                }
                            }
                        }),
                );
            }
        }

        paths_count
    }
}

#[cfg(test)]
mod tests {
    use super::day_12::*;

    static TEST_INPUT_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    static TEST_INPUT_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    static TEST_INPUT_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn part1_known() {
        let data = parse(TEST_INPUT_1);
        assert_eq!(part_1(&data), 10);

        let data = parse(TEST_INPUT_2);
        assert_eq!(part_1(&data), 19);

        let data = parse(TEST_INPUT_3);
        assert_eq!(part_1(&data), 226);
    }

    #[test]
    fn part2_known() {
        let data = parse(TEST_INPUT_1);
        assert_eq!(part_2(&data), 36);

        let data = parse(TEST_INPUT_2);
        assert_eq!(part_2(&data), 103);

        let data = parse(TEST_INPUT_3);
        assert_eq!(part_2(&data), 3509);
    }

    #[test]
    fn test_day_12() {
        let input = input();
        assert_eq!(part_1(&input), 5958);
        assert_eq!(part_2(&input), 150426);
    }
}
