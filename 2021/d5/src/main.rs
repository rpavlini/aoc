use std::{collections::HashMap, str::FromStr};

type Point = (u32, u32);

struct Line {
    points: (Point, Point),
}

impl Line {
    fn is_straight(&self) -> bool {
        let (start, end) = self.points;
        return start.0 == end.0 || start.1 == end.1;
    }

    fn iter(&self) -> LinePointsIter {
        LinePointsIter {
            points: self.points,
            prev: None,
        }
    }
}

struct LinePointsIter {
    points: (Point, Point),
    prev: Option<Point>,
}

impl Iterator for LinePointsIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev.is_some() && self.prev.unwrap() == self.points.1 {
            return None;
        }

        if self.prev.is_none() {
            self.prev = Some(self.points.0);
            return Some(self.points.0);
        }

        let prev = self.prev.unwrap();
        let end = self.points.1;

        let dx = end.0 as i32 - prev.0 as i32;
        let dy = end.1 as i32 - prev.1 as i32;

        let next = (
            if dx == 0 {
                prev.0
            } else {
                if prev.0 > end.0 {
                    prev.0 - 1
                } else {
                    prev.0 + 1
                }
            },
            if dy == 0 {
                prev.1
            } else {
                if prev.1 > end.1 {
                    prev.1 - 1
                } else {
                    prev.1 + 1
                }
            },
        );

        self.prev = Some(next);

        Some(next)
    }
}

impl FromStr for Line {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let start: Vec<u32> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect();
        let end: Vec<u32> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect();

        Ok(Line {
            points: ((start[0], start[1]), (end[0], end[1])),
        })
    }
}

fn main() {
    let input = include_str!("../input");
    let lines: Vec<Line> = input.lines().map(|l| l.parse().unwrap()).collect();

    first(&lines);
    second(&lines);
}

fn first(lines: &Vec<Line>) -> u32 {
    let straight = lines.iter().filter(|l| l.is_straight());
    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();

    for l in straight {
        for p in l.iter() {
            *grid.entry(p).or_insert(0) += 1;
        }
    }

    let overlap_count = grid.values().filter(|v| **v >= 2).count();

    println!("{}", overlap_count);
    overlap_count as u32
}

fn second(lines: &Vec<Line>) -> u32 {
    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();

    for l in lines {
        for p in l.iter() {
            *grid.entry(p).or_insert(0) += 1;
        }
    }

    let overlap_count = grid.values().filter(|v| **v >= 2).count();

    println!("{}", overlap_count);
    overlap_count as u32
}

#[cfg(test)]
mod tests {

    use crate::{first, second, Line};

    #[test]
    fn test() {
        let input = include_str!("../input.test");
        let lines: Vec<Line> = input.lines().map(|l| l.parse().unwrap()).collect();

        assert_eq!(first(&lines), 5);
        assert_eq!(second(&lines), 12);
    }
}
