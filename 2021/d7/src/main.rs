fn main() {
    let input = include_str!("../input");
    let crabs: Vec<u32> = input.split(',').map(|s| s.parse().unwrap()).collect();

    first(&crabs);
    second(&crabs);
}

fn first(crabs: &Vec<u32>) -> u32 {
    let max = crabs.iter().max().unwrap();
    let mut min = u32::MAX;

    for i in 0..*max {
        let curr = crabs
            .iter()
            .map(|&v| {
                let r = (v as i32 - i as i32).abs();
                return r as u32;
            })
            .sum();

        min = min.min(curr);
    }

    println!("{}", min);
    return min;
}

fn sum_series(from: u32, to: u32) -> u32 {
    let n = to - from;
    return n * (n + 1) / 2;
}

fn second(crabs: &Vec<u32>) -> u32 {
    let max = crabs.iter().max().unwrap();
    let mut min = u32::MAX;

    for i in 0..*max {
        let curr = crabs.iter().map(|&v| sum_series(v.min(i), v.max(i))).sum();
        min = min.min(curr);
    }

    println!("{}", min);
    return min;
}

#[cfg(test)]
mod tests {
    use crate::{first, second};

    #[test]
    fn test() {
        let input = include_str!("../input.test");
        let crabs: Vec<u32> = input.split(',').map(|s| s.parse().unwrap()).collect();

        assert_eq!(first(&crabs), 37);
        assert_eq!(second(&crabs), 168);
    }
}
