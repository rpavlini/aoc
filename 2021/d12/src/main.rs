use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type Matrix = Vec<Vec<bool>>;

fn main() {
    let input = include_str!("../input");

    let graph1: Graph = input.parse().unwrap();
    let graph2: Graph = input.parse().unwrap();

    first(&graph1);
    second(&graph2);
}

fn first(graph: &Graph) -> u32 {
    let paths = graph.traverse_small_once();

    println!("{}", paths);
    return paths;
}

fn second(graph: &Graph) -> u32 {
    let paths = graph.traverse_small_twice();

    println!("{}", paths);
    return paths;
}

fn is_lowercase(s: &str) -> bool {
    return s.to_lowercase() == s;
}

#[derive(Debug)]
struct Graph {
    matrix: Matrix,
    small: HashSet<usize>,
    start: usize,
    end: usize,
}

impl Graph {
    fn traverse_small_twice(&self) -> u32 {
        let initial = &self.matrix[self.start];
        let mut paths = 0;

        for (n, c) in initial.clone().iter().enumerate() {
            if *c {
                let mut visited: HashMap<usize, u32> = HashMap::new();
                for i in 0..self.matrix.len() {
                    visited.insert(i, 0);
                }
                let mut visited = visited.clone();
                visited.insert(self.start, 2);
                visited.insert(self.end, 1);

                paths += self.dfs_twice(n, &mut visited);
            }
        }

        return paths;
    }

    fn traverse_small_once(&self) -> u32 {
        let initial = &self.matrix[self.start];
        let mut sum = 0;

        for (n, c) in initial.iter().enumerate() {
            if *c {
                let mut visited: HashSet<usize> = HashSet::new();
                visited.insert(self.start);
                sum += self.dfs_once(n, &mut visited);
            }
        }

        return sum;
    }

    fn dfs_once(&self, curr: usize, visited: &mut HashSet<usize>) -> u32 {
        if curr == self.end {
            return 1;
        }

        visited.insert(curr);
        let mut ret = 0;

        for (i, conn) in self.matrix[curr].iter().enumerate() {
            if !conn || (visited.contains(&i) && self.small.contains(&i)) {
                continue;
            }
            let mut visited: HashSet<usize> = visited.clone();
            ret += self.dfs_once(i, &mut visited)
        }

        return ret;
    }

    fn dfs_twice(&self, curr: usize, visited: &mut HashMap<usize, u32>) -> u32 {
        if curr == self.end {
            return 1;
        }

        *visited.get_mut(&curr).unwrap() += 1;
        let small_twice = self.small.iter().any(|&f| {
            if f == self.start || f == self.end {
                return false;
            }
            return *visited.get(&f).unwrap() == 2;
        });
        let mut ret = 0;

        for (i, conn) in self.matrix[curr].iter().enumerate() {
            if !conn {
                continue;
            }

            let visited_count = *visited.get(&i).unwrap();

            if small_twice && visited_count == 1 && i != self.end && self.small.contains(&i) {
                continue;
            }

            if visited_count == 2 && self.small.contains(&i) {
                continue;
            }

            let mut visited: HashMap<usize, u32> = visited.clone();

            ret += self.dfs_twice(i, &mut visited);
        }

        return ret;
    }
}

impl FromStr for Graph {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let connections: Vec<(&str, &str)> = input
            .lines()
            .map(|l| {
                let mut s = l.split('-');
                let a = s.next().unwrap();
                let b = s.next().unwrap();

                return (a, b);
            })
            .collect();

        let mut nodes: HashSet<&str> = HashSet::new();

        let mut start = 0;
        let mut end = 0;

        for c in &connections {
            nodes.insert(c.0);
            nodes.insert(c.1);
        }

        let mut nodes: Vec<&str> = nodes.iter().map(|s| *s).collect();
        nodes.sort();

        let node_map: HashMap<&str, usize> =
            nodes.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        let mut small: HashSet<usize> = HashSet::new();

        let mut matrix = Vec::new();

        for _ in nodes.iter() {
            let n = nodes.len();
            let mut r = Vec::with_capacity(n);
            for _ in 0..n {
                r.push(false);
            }
            matrix.push(r);
        }

        for c in &connections {
            let a = *node_map.get(c.0).unwrap();
            let b = *node_map.get(c.1).unwrap();

            matrix[a][b] = true;
            matrix[b][a] = true;

            if c.0 == "start" {
                start = a;
            }
            if c.1 == "end" {
                end = b;
            }
            if c.0 == "end" {
                end = a;
            }
            if c.1 == "start" {
                start = b;
            }

            if is_lowercase(c.0) {
                small.insert(a);
            }
            if is_lowercase(c.1) {
                small.insert(b);
            }
        }

        Ok(Graph {
            matrix,
            small,
            start,
            end,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Graph};

    #[test]
    fn test() {
        let input1 = include_str!("../input.test");
        let input2 = include_str!("../input2.test");
        let input3 = include_str!("../input3.test");

        let g1: Graph = input1.parse().unwrap();
        let g2: Graph = input2.parse().unwrap();
        let g3: Graph = input3.parse().unwrap();

        let g4: Graph = input1.parse().unwrap();
        let g5: Graph = input2.parse().unwrap();
        let g6: Graph = input3.parse().unwrap();

        assert_eq!(first(&g1), 10);
        assert_eq!(first(&g2), 19);
        assert_eq!(first(&g3), 226);

        assert_eq!(second(&g4), 36);
        assert_eq!(second(&g5), 103);
        assert_eq!(second(&g6), 3509);
    }
}
