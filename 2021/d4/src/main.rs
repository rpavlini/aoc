use std::{collections::HashSet, iter::FromIterator, str::FromStr};

struct Grid {
    nums: Vec<u32>,
}

impl Grid {
    fn is_winner(&self, drawn_nums: &HashSet<u32>) -> bool {
        for i in 0..5 {
            let target = self.get_col(i);
            if target.intersection(drawn_nums).count() == 5 {
                return true;
            }
        }

        for i in 0..5 {
            let target = self.get_row(i);
            if target.intersection(drawn_nums).count() == 5 {
                return true;
            }
        }

        return false;
    }

    fn get_col(&self, idx: u32) -> HashSet<u32> {
        let mut set = HashSet::with_capacity(5);

        for i in 0..5 {
            let offset = 5 * i + idx;
            set.insert(self.nums[offset as usize]);
        }

        return set;
    }

    fn get_row(&self, idx: u32) -> HashSet<u32> {
        let offset_start = 5 * idx;
        let offset_end = 5 * idx + 5;

        let slice = self.nums[(offset_start as usize)..(offset_end as usize)].iter();

        return HashSet::from_iter(slice.cloned());
    }

    fn sum_unmatched(&self, drawn_nums: &HashSet<u32>) -> u32 {
        return self.nums.iter().filter(|n| !drawn_nums.contains(n)).sum();
    }
}

fn main() {
    let input = include_str!("../input");

    let mut parts = input.split("\n\n");

    let drawn_nums: Vec<u32> = parts
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let grids = parts.map(|g| g.parse().unwrap()).collect::<Vec<Grid>>();

    first(&drawn_nums, &grids);
    second(&drawn_nums, &grids);
}

fn first(drawn: &Vec<u32>, grids: &Vec<Grid>) -> u32 {
    for n in 5..drawn.len() {
        let nums: HashSet<u32> = HashSet::from_iter(drawn[0..n].iter().cloned());

        for g in grids {
            if g.is_winner(&nums) {
                let sum = g.sum_unmatched(&nums) * drawn[n - 1];
                println!("{}", sum);
                return sum;
            }
        }
    }

    return 0;
}

fn second(drawn: &Vec<u32>, grids: &Vec<Grid>) -> u32 {
    let mut evicted: HashSet<usize> = HashSet::new();

    for n in 5..drawn.len() {
        let nums: HashSet<u32> = HashSet::from_iter(drawn[0..n].iter().cloned());

        for (i, g) in grids.iter().enumerate() {
            if evicted.contains(&i) {
                continue;
            }

            if g.is_winner(&nums) {
                evicted.insert(i);
                if evicted.len() == grids.len() {
                    let sum = g.sum_unmatched(&nums) * drawn[n - 1];
                    println!("{}", sum);
                    return sum;
                }
            }
        }
    }

    return 0;
}

impl FromStr for Grid {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u32> = s
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect();

        Ok(Grid { nums })
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Grid};

    #[test]
    fn test() {
        let input = include_str!("../input.test");

        let mut parts = input.split("\n\n");

        let drawn_nums: Vec<u32> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        let grids = parts.map(|g| g.parse().unwrap()).collect::<Vec<Grid>>();

        assert_eq!(first(&drawn_nums, &grids), 4512);
        assert_eq!(second(&drawn_nums, &grids), 1924);
    }
}
