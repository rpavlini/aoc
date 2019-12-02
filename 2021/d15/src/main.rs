use std::collections::{BinaryHeap, HashMap, HashSet};

type Matrix = Vec<Vec<u32>>;

type Node = (usize, usize); // row, col

const ADJ_NODES: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn is_in_bounds(node: &(isize, isize), matrix: &Matrix) -> bool {
    return node.0 >= 0
        && node.0 < matrix.len() as isize
        && node.1 >= 0
        && node.1 < matrix[0].len() as isize;
}

fn get_adjacent_nodes(node: &Node, matrix: &Matrix) -> Vec<Node> {
    let nodes: Vec<(isize, isize)> = ADJ_NODES
        .iter()
        .map(|(dx, dy)| (node.0 as isize + dx, node.1 as isize + dy))
        .collect();

    let valid_nodes = nodes.iter().filter(|p| is_in_bounds(p, &matrix));
    return valid_nodes.map(|n| (n.0 as usize, n.1 as usize)).collect();
}

fn main() {
    let input = include_str!("../input");
    let matrix: Matrix = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    first(&matrix);
    second(&matrix);
}

fn dp(matrix: &Matrix) -> u32 {
    let n = matrix.len();

    let mut dpm: Matrix = matrix.clone();

    for i in 1..n {
        dpm[0][i] += dpm[0][i - 1];
    }

    for i in 1..n {
        dpm[i][0] += dpm[i - 1][0];
    }

    for i in 1..n {
        for j in 1..n {
            let above = dpm[i][j - 1];
            let left = dpm[i - 1][j];
            let curr = dpm[i][j];

            dpm[i][j] = *vec![curr + above, curr + left].iter().min().unwrap();
        }
    }

    println!("{}, {}", dpm[n - 1][n - 1], dpm[0][0]);
    let cost = dpm[n - 1][n - 1] - dpm[0][0];

    return cost;
}

fn print_matrix(matrix: &Matrix) {
    for r in matrix {
        let compact: Vec<u32> = r.iter().map(|&v| v).collect();
        let mut s = String::new();
        for c in compact {
            s.push_str(&c.to_string());
        }
        println!("{}", s);
    }
}

fn matrix_from_tile(matrix: &Matrix, repeat: u32) -> Matrix {
    let mut wip: Matrix = Vec::new();

    let n = matrix.len() as u32;

    for i in 0..n {
        let mut row = matrix[i as usize].clone();

        for j in n..(repeat - 1) * n {
            let target_j = j - n;
            let curr = row[target_j as usize];
            row.push(curr % 9 + 1);
        }

        wip.push(row);
    }

    for i in n..(repeat - 1) * n {
        let target = i - n;
        let mut row = wip[target as usize].clone();

        for j in 0..n * (repeat - 1) {
            row[j as usize] = row[j as usize] % 9 + 1;
        }

        wip.push(row);
    }

    return wip;
}

#[derive(Debug)]
struct DNode(u32, Node);

impl Ord for DNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // self.0.cmp(&other.0)
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for DNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DNode {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

impl Eq for DNode {}

fn dijkstra(matrix: &Matrix) -> HashMap<Node, u32> {
    let mut unvisited: HashSet<Node> = HashSet::new();
    let mut dist = HashMap::new();
    let mut heap: BinaryHeap<DNode> = BinaryHeap::new();

    let len = matrix.len();

    for i in 0..len {
        for j in 0..len {
            unvisited.insert((i, j));
            dist.insert((i, j), u32::MAX);
        }
    }

    heap.push(DNode(0, (0, 0)));

    while unvisited.len() > 0 {
        let curr = heap.pop().unwrap();
        if !unvisited.contains(&curr.1) {
            continue;
        }

        let neighbors = get_adjacent_nodes(&curr.1, matrix);

        for (i, j) in neighbors {
            if !unvisited.contains(&(i, j)) {
                continue;
            }
            let new_cost = matrix[i][j] + curr.0;
            let old_cost = *dist.get(&(j, i)).unwrap();

            if new_cost < old_cost {
                dist.insert((i, j), new_cost);
            }
            heap.push(DNode(new_cost, (i, j)));
        }
        unvisited.remove(&curr.1);
    }

    return dist;
}

fn first(matrix: &Matrix) -> u32 {
    let n = matrix.len() - 1;
    let distances = dijkstra(matrix);
    let cost = *distances.get(&(n, n)).unwrap();

    println!("{}", cost);

    return cost;
}

fn second(matrix: &Matrix) -> u32 {
    let full_matrix = matrix_from_tile(matrix, 6);
    let n = full_matrix.len() - 1;
    let distances = dijkstra(&full_matrix);

    let cost = *distances.get(&(n, n)).unwrap();

    println!("{}", cost);

    return cost;
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Matrix};

    #[test]
    fn test() {
        let input = include_str!("../input.test");

        let matrix: Matrix = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        assert_eq!(first(&matrix), 40);
        assert_eq!(second(&matrix), 315);
    }
}
