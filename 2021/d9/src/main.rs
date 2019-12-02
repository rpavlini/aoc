use std::collections::{HashSet, VecDeque};

const ADJ_PTS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point(usize, usize); // row, col

fn main() {
    let input = include_str!("../input");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect())
        .collect();

    first(&grid);
    second(&grid);
}

fn first(grid: &Vec<Vec<u32>>) -> u32 {
    let mut nums = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let curr = grid[i][j];
            let point = Point(i, j);
            let adj = get_adjacent(&point, &grid);
            if curr < *adj.iter().min().unwrap() {
                nums.push(curr);
            }
        }
    }

    let sum = nums.iter().map(|n| n + 1).sum();

    println!("{}", sum);
    return sum;
}

fn second(grid: &Vec<Vec<u32>>) -> u32 {
    let mut pts = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let curr = grid[i][j];
            let point = Point(i, j);
            let adj = get_adjacent(&point, &grid);
            if curr < *adj.iter().min().unwrap() {
                pts.push(point);
            }
        }
    }

    let mut basins: Vec<HashSet<Point>> = Vec::new();
    let mut excluded: HashSet<Point> = HashSet::new();

    for p in pts {
        let basin = bfs_increasing(&p, grid, &excluded);

        for p in &basin {
            excluded.insert(*p);
        }
        basins.push(basin);
    }

    basins.sort_by(|a, b| return b.len().cmp(&a.len()));

    let largest_3 = &basins[0..3];

    let ret: usize = largest_3.iter().map(|v| v.len()).product();

    println!("{:#?}", ret);
    return ret as u32;
}

fn is_in_bounds(point: &(isize, isize), grid: &Vec<Vec<u32>>) -> bool {
    return point.0 >= 0
        && point.0 < grid.len() as isize
        && point.1 >= 0
        && point.1 < grid[0].len() as isize;
}

fn is_increasing(a: &Point, b: &Point, grid: &Vec<Vec<u32>>) -> bool {
    return grid[b.0][b.1] > grid[a.0][a.1] && grid[b.0][b.1] != 9;
}

fn bfs_increasing(
    start: &Point,
    grid: &Vec<Vec<u32>>,
    excluded: &HashSet<Point>,
) -> HashSet<Point> {
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_front(*start);

    let mut ret: HashSet<Point> = HashSet::new();
    ret.insert(*start);

    let mut visited: HashSet<Point> = HashSet::new();

    while queue.len() > 0 {
        let curr = queue.pop_back().unwrap();
        let adj = get_increasing_adj_pts(&curr, grid);

        for p in adj {
            if !visited.contains(&p) && !excluded.contains(&p) {
                ret.insert(p);
                visited.insert(p);
                queue.push_front(p);
            }
        }
    }

    return ret;
}

fn get_increasing_adj_pts(point: &Point, grid: &Vec<Vec<u32>>) -> Vec<Point> {
    return get_adjacent_pts(point, grid)
        .iter()
        .filter(|p| is_increasing(point, p, grid))
        .map(|p| *p)
        .collect();
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

fn get_adjacent(point: &Point, grid: &Vec<Vec<u32>>) -> Vec<u32> {
    let adj_pts = get_adjacent_pts(point, grid);

    return adj_pts.iter().map(|p| grid[p.0][p.1]).collect();
}

#[cfg(test)]
mod tests {
    use crate::{first, second};

    #[test]
    fn test() {
        let input = include_str!("../input.test");
        let grid: Vec<Vec<u32>> = input
            .lines()
            .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect())
            .collect();

        assert_eq!(first(&grid), 15);
        assert_eq!(second(&grid), 1134);
    }
}
