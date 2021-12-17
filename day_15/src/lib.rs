pub mod day_15 {

    use std::cmp::min;
    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;

    trait EdgeIter<'a, V, I>
    where
        I: IntoIterator<Item = (&'a V, u8)>,
        V: 'a,
    {
        fn edges<'v>(&'a self, v: &'v V) -> Option<I>;
        fn vertex_count(&self) -> usize;
    }

    pub struct EdgeWeightedGraph<V> {
        edges: HashMap<V, HashMap<V, u8>>,
    }

    impl<'a, V>
        EdgeIter<
            'a,
            V,
            std::iter::Map<
                std::collections::hash_map::Iter<'a, V, u8>,
                fn((&'a V, &'a u8)) -> (&'a V, u8),
            >,
        > for EdgeWeightedGraph<V>
    where
        V: Eq + Hash + 'a,
    {
        fn edges<'v>(
            &'a self,
            v: &'v V,
        ) -> Option<
            std::iter::Map<
                std::collections::hash_map::Iter<'a, V, u8>,
                fn((&'a V, &'a u8)) -> (&'a V, u8),
            >,
        > {
            let iter = self.edges.get(v)?;
            Some(iter.iter().map(|(k, v)| (k, *v)))
        }
        fn vertex_count(&self) -> usize {
            self.edges.len()
        }
    }

    fn shortest_path<'a, V, I, J>(graph: &'a I, v1: V, v2: V) -> Option<u32>
    where
        V: Hash + Eq + std::fmt::Debug + 'a,
        J: IntoIterator<Item = (&'a V, u8)>,
        I: EdgeIter<'a, V, J>,
    {
        let mut to_visit: HashSet<&V> = HashSet::with_capacity(graph.vertex_count());
        let mut visited: HashSet<&V> = HashSet::with_capacity(graph.vertex_count());

        let mut distances: HashMap<&V, u32> = HashMap::with_capacity(graph.vertex_count());
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

            let outgoing = match graph.edges(current_node) {
                None => {
                    panic!("Vertex did not have an entry in the outgoing edges");
                }
                Some(outgoing) => outgoing,
            };

            for (dest, distance) in outgoing {
                if !visited.contains(dest) {
                    to_visit.insert(dest);
                    let distance_via_here = current_distance + distance as u32;
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

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
    pub(crate) struct Point {
        x: u16,
        y: u16,
    }

    struct PointHasher {
        state: u32,
    }

    impl std::hash::Hasher for PointHasher {
        fn finish(&self) -> u64 {
            self.state as u64
        }

        fn write_u16(&mut self, coord: u16) {
            self.state = self.state * 65536 + coord as u32;
        }

        fn write(&mut self, _bytes: &[u8]) {
            panic!("Never!");
        }
    }

    struct BuildPointHasher {}

    impl std::hash::BuildHasher for BuildPointHasher {
        type Hasher = PointHasher;
        fn build_hasher(&self) -> PointHasher {
            PointHasher { state: 0 }
        }
    }

    pub struct VertexWeightedGraph<V> {
        vertices: HashMap<V, (u8, HashSet<V, BuildPointHasher>), BuildPointHasher>,
    }

    impl<'a, V>
        EdgeIter<
            'a,
            V,
            std::iter::Map<std::collections::hash_set::Iter<'a, V>, fn(&'a V) -> (&'a V, u8)>,
        > for VertexWeightedGraph<V>
    where
        V: 'a + Eq + Hash,
    {
        fn edges<'v>(
            &'a self,
            v: &'v V,
        ) -> Option<std::iter::Map<std::collections::hash_set::Iter<'a, V>, fn(&'a V) -> (&'a V, u8)>>
        {
            let (weight, edges) = self.vertices.get(v)?;
            Some(
                edges
                    .iter()
                    .map(|i| (i, weight + self.vertices.get(i).unwrap().0)),
            )
        }
        fn vertex_count(&self) -> usize {
            self.vertices.len()
        }
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

    // TODO: commonise with day 9
    #[derive(Debug, Clone)]
    pub struct Array<T> {
        row_len: usize,
        elts: Vec<T>,
    }

    impl<T> Array<T> {
        fn col_len(&self) -> usize {
            self.elts.len() / self.row_len
        }
        fn get(&self, row: usize, col: usize) -> T
        where
            T: Copy,
        {
            self.elts[row * self.row_len + col]
        }
        fn set(&mut self, row: usize, col: usize, val: T)
        where
            T: Copy,
        {
            self.elts[row * self.row_len + col] = val;
        }
    }

    pub(crate) fn parse(s: &str) -> Array<u8> {
        let mut answer = Array {
            row_len: 0,
            elts: Vec::new(),
        };
        for line in s.split('\n') {
            if answer.row_len == 0 {
                answer.row_len = line.len();
            }
            answer
                .elts
                .extend(line.chars().map(|c| char::to_digit(c, 10).unwrap() as u8));
        }
        answer
    }

    pub(crate) fn to_vertex_graph(arr: &Array<u8>) -> VertexWeightedGraph<Point> {
        let mut vertices: HashMap<Point, (u8, HashSet<Point, BuildPointHasher>), BuildPointHasher> =
            HashMap::with_hasher(BuildPointHasher {});

        for row in 0..arr.col_len() {
            for col in 0..arr.row_len {
                let height = arr.get(row, col);
                let row_16 = row as u16;
                let col_16 = col as u16;

                let mut outgoing = HashSet::with_capacity_and_hasher(4, BuildPointHasher {});
                if row > 0 {
                    outgoing.insert(Point {
                        x: row_16 - 1,
                        y: col_16,
                    });
                }
                if row < arr.col_len() - 1 {
                    outgoing.insert(Point {
                        x: row_16 + 1,
                        y: col_16,
                    });
                }
                if col > 0 {
                    outgoing.insert(Point {
                        x: row_16,
                        y: col_16 - 1,
                    });
                }
                if col < arr.row_len - 1 {
                    outgoing.insert(Point {
                        x: row_16,
                        y: col_16 + 1,
                    });
                }
                vertices.insert(
                    Point {
                        x: row_16,
                        y: col_16,
                    },
                    (height, outgoing),
                );
            }
        }

        VertexWeightedGraph { vertices }
    }

    pub fn input() -> Array<u8> {
        parse(include_str!("../input.txt"))
    }

    pub fn part_1(data: &Array<u8>) -> u32 {
        let edged = to_edge_weighted(&to_vertex_graph(data));
        let max = Point {
            x: data.row_len as u16 - 1,
            y: data.col_len() as u16 - 1,
        };
        let edge_weight = shortest_path(&edged, Point { x: 0, y: 0 }, max).unwrap();

        let first_node = data.get(0, 0) as u32;
        let last_node = data.get(data.row_len - 1, data.col_len() - 1) as u32;

        (edge_weight + first_node + last_node) / 2 - first_node
    }

    pub(crate) fn quintuple(arr: &Array<u8>) -> Array<u8> {
        let row_len = 5 * arr.row_len;
        let elts = vec![0u8; row_len * row_len];
        let mut result = Array { row_len, elts };

        for row in 0..arr.col_len() {
            for col in 0..arr.row_len {
                for x in 0..5 {
                    for y in 0..5 {
                        result.set(
                            arr.row_len * x + row,
                            arr.col_len() * y + col,
                            (arr.get(row, col) + (x as u8) + (y as u8) - 1) % 9 + 1,
                        );
                    }
                }
            }
        }

        result
    }

    pub fn part_2(data: &Array<u8>) -> u32 {
        let data = quintuple(data);
        let edged = to_edge_weighted(&to_vertex_graph(&data));
        let max = Point {
            x: data.row_len as u16 - 1,
            y: data.col_len() as u16 - 1,
        };
        let edge_weight = shortest_path(&edged, Point { x: 0, y: 0 }, max).unwrap();

        let first_node = data.get(0, 0) as u32;
        let last_node = data.get(data.row_len - 1, data.col_len() - 1) as u32;

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
