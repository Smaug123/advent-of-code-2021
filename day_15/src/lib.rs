pub mod day_15 {

    use std::cmp::min;
    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;

    pub struct EdgeWeightedGraph<V> {
        edges: HashMap<V, HashMap<V, u8>>,
    }

    fn shortest_path<V>(graph: &EdgeWeightedGraph<V>, v1: V, v2: V) -> Option<u32>
    where
        V: Hash + Eq + std::fmt::Debug,
    {
        let mut to_visit: HashSet<&V> = HashSet::with_capacity(graph.edges.len());
        let mut visited: HashSet<&V> = HashSet::with_capacity(graph.edges.len());

        let mut distances: HashMap<&V, u32> = HashMap::with_capacity(graph.edges.len());
        distances.insert(&v1, 0);

        let mut current_node = &v1;
        let mut current_distance = 0u32;

        loop {
            if *current_node == v2 {
                let d = distances.get(&current_node)?;
                return Some(*d);
            }

            visited.insert(current_node);
            to_visit.remove(current_node);

            let outgoing = match graph.edges.get(&current_node) {
                None => {
                    panic!("Vertex did not have an entry in the outgoing edges");
                }
                Some(outgoing) => outgoing,
            };

            for (dest, distance) in outgoing.iter() {
                if !visited.contains(dest) {
                    to_visit.insert(dest);
                    let distance_via_here = current_distance + *distance as u32;
                    distances
                        .entry(dest)
                        .and_modify(|v| *v = min(*v, distance_via_here))
                        .or_insert(distance_via_here);
                }
            }

            match to_visit
                .iter()
                .filter_map(|&v| distances.get(v).map(|d| (v, d)))
                .min_by_key(|(_, &d)| d)
            {
                None => {
                    return None;
                }
                Some((v, &d)) => {
                    current_node = v;
                    current_distance = d;
                }
            }
        }
    }

    pub struct VertexWeightedGraph<V> {
        vertices: HashMap<V, (u8, HashSet<V>)>,
    }

    fn to_edge_weighted<V>(v: &VertexWeightedGraph<V>) -> EdgeWeightedGraph<V>
    where
        V: Eq + Hash + Copy,
    {
        let mut edges: HashMap<V, HashMap<V, u8>> = HashMap::with_capacity(v.vertices.len());

        for (source, (weight, dests)) in v.vertices.iter() {
            let mut m = HashMap::with_capacity(dests.len());
            for dest in dests.iter() {
                m.insert(*dest, v.vertices.get(dest).unwrap().0 + weight);
            }
            edges.insert(*source, m);
        }

        EdgeWeightedGraph { edges }
    }

    pub(crate) fn parse(s: &str) -> VertexWeightedGraph<(u16, u16)> {
        let mut vertices: HashMap<(u16, u16), (u8, HashSet<(u16, u16)>)> = HashMap::new();
        for (i, line) in s.split('\n').enumerate() {
            for (j, height) in line.chars().enumerate() {
                let height = height as u8 - b'0';
                let i_16 = i as u16;
                let j_16 = j as u16;

                let mut outgoing = HashSet::with_capacity(4);
                if i > 0 {
                    outgoing.insert((i_16 - 1, j_16));
                }
                if i < line.len() - 1 {
                    outgoing.insert((i_16 + 1, j_16));
                }
                if j > 0 {
                    outgoing.insert((i_16, j_16 - 1));
                }
                // TODO - deal correctly with non-square grids
                if j < line.len() - 1 {
                    outgoing.insert((i_16, j_16 + 1));
                }
                vertices.insert((i_16, j_16), (height, outgoing));
            }
        }

        VertexWeightedGraph { vertices }
    }

    pub fn input() -> VertexWeightedGraph<(u16, u16)> {
        parse(include_str!("../input.txt"))
    }

    pub fn part_1(data: &VertexWeightedGraph<(u16, u16)>) -> u32 {
        let edged = to_edge_weighted(&data);
        let max = data.vertices.keys().max_by_key(|(x, y)| x + y).unwrap();
        let edge_weight = shortest_path(&edged, (0, 0), *max).unwrap();

        let first_node = data.vertices.get(&(0,0)).unwrap().0 as u32;
        let last_node = data.vertices.get(max).unwrap().0 as u32;

        (edge_weight + first_node + last_node) / 2 - first_node
    }

    pub fn part_2(_data: &VertexWeightedGraph<(u16, u16)>) -> u32 {
        panic!("TODO");
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
