fn main() {
    let input = include_str!("../input");
    let nums: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();

    first(&nums);
    second(&nums);
}

fn first(nums: &Vec<u32>) -> u32 {
    let incr = nums.windows(2).fold(0, |acc, curr| {
        if curr[1] > curr[0] {
            return acc + 1;
        }
        return acc;
    });

    println!("{}", incr);
    incr
}

fn second(nums: &Vec<u32>) -> u32 {
    let (incr, _sum) = nums
        .windows(3)
        .fold((0, u32::MAX), |(incr, prev_sum), curr| {
            let sum = curr.iter().sum();
            if sum > prev_sum {
                return (incr + 1, sum);
            }
            return (incr, sum);
        });

    println!("{}", incr);
    incr
}

#[cfg(test)]
mod tests {
    use crate::{first, second};

    #[test]
    fn test() {
        let input = include_str!("../input.test");
        let nums: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();

        assert_eq!(first(&nums), 7);
        assert_eq!(second(&nums), 5);
    }
}
