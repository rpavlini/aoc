use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

const ADJ_PTS: [(isize, isize); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
];

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point(usize, usize); // row, col

fn main() {
    let input = include_str!("../input");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect())
        .collect();

    first(&grid, 100);
    second(&grid);
}

fn first(grid: &Vec<Vec<u32>>, iterations: u32) -> u32 {
    let mut grid = grid.clone();

    let mut flashes = 0;
    let size = grid.len();

    for _ in 0..iterations {
        let mut visited: HashSet<Point> = HashSet::new();
        for r in 0..size {
            for c in 0..size {
                let p = Point(r, c);
                if !visited.contains(&p) {
                    grid[r][c] += 1;
                    if grid[r][c] == 10 {
                        flashes += 1;
                        let count = flood_fill(&mut grid, p, &mut visited);
                        flashes += count;
                    }
                }
            }
        }
    }

    println!("{}", flashes);

    return flashes;
}

fn all_flashing(grid: &Vec<Vec<u32>>) -> bool {
    return grid.iter().map(|r| r.iter()).flatten().all(|&v| v == 0);
}

fn second(grid: &Vec<Vec<u32>>) -> u32 {
    let mut grid = grid.clone();

    let mut flashes = 0;
    let size = grid.len();

    while !all_flashing(&grid) {
        flashes += 1;
        let mut visited: HashSet<Point> = HashSet::new();
        for r in 0..size {
            for c in 0..size {
                let p = Point(r, c);
                if !visited.contains(&p) {
                    grid[r][c] += 1;
                    if grid[r][c] == 10 {
                        flood_fill(&mut grid, p, &mut visited);
                    }
                }
            }
        }
    }

    println!("{}", flashes);

    return flashes;
}

fn is_in_bounds(point: &(isize, isize), grid: &Vec<Vec<u32>>) -> bool {
    return point.0 >= 0
        && point.0 < grid.len() as isize
        && point.1 >= 0
        && point.1 < grid[0].len() as isize;
}

fn flood_fill(grid: &mut Vec<Vec<u32>>, init: Point, visited: &mut HashSet<Point>) -> u32 {
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut count = 0;

    queue.push_front(init);
    grid[init.0][init.1] = 0;

    visited.insert(init);

    while queue.len() > 0 {
        let curr = queue.pop_back().unwrap();
        let adj = get_adjacent_pts(&curr, grid);

        for p in adj {
            if !visited.contains(&p) {
                let val = &mut grid[p.0][p.1];
                *val += 1;
                if *val == 10 {
                    count += 1;
                    visited.insert(p);
                    queue.push_front(p);
                    *val = 0;
                }
            }
        }
    }

    return count;
}

fn get_adjacent_pts(point: &Point, grid: &Vec<Vec<u32>>) -> Vec<Point> {
    let pts: Vec<(isize, isize)> = ADJ_PTS
        .iter()
        .map(|(dx, dy)| (point.0 as isize + dx, point.1 as isize + dy))
        .collect();

    let valid_pts = pts.iter().filter(|p| is_in_bounds(p, &grid));
    return valid_pts
        .map(|p| Point(p.0 as usize, p.1 as usize))
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::{first, second};

    #[test]
    fn test() {
        let input = include_str!("../input.test");
        let mut grid: Vec<Vec<u32>> = input
            .lines()
            .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect())
            .collect();

        assert_eq!(first(&mut grid, 10), 204);
        assert_eq!(first(&mut grid, 100), 1656);
        assert_eq!(second(&mut grid), 195);
    }
}
