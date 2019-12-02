use std::str::FromStr;

fn main() {
    let input = include_str!("../input");
    let nums: Vec<&str> = input.lines().collect();

    first(&nums);
    second(&nums);
}

fn first(nums: &Vec<&str>) -> usize {
    let num_bits = nums[0].len();

    let mut gamma = String::new();
    let mut eps = String::new();

    for b in 0..num_bits {
        let bit = most_bits(&nums, b).unwrap();

        match bit {
            '0' => {
                gamma.push('0');
                eps.push('1');
            }
            '1' => {
                gamma.push('1');
                eps.push('0');
            }
            _ => panic!("asd"),
        }
    }

    println!("{}", to_bin(&gamma) * to_bin(&eps));
    to_bin(&gamma) * to_bin(&eps)
}

fn most_bits(vec: &Vec<&str>, idx: usize) -> Option<char> {
    let mut zeros = 0;
    let mut ones = 0;

    for e in vec {
        match e.chars().nth(idx).unwrap() {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => panic!("not 0 or 1"),
        }
    }

    if zeros > ones {
        return Some('0');
    } else if ones > zeros {
        return Some('1');
    }
    return None;
}

fn to_bin(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

fn second(nums: &Vec<&str>) -> usize {
    let oxy = reduce(&nums, '1', false);
    let co2 = reduce(&nums, '0', true);

    println!("{}", to_bin(&oxy) * to_bin(&co2));
    to_bin(&oxy) * to_bin(&co2)
}

fn reduce(nums: &Vec<&str>, target: char, reverse: bool) -> String {
    let mut wip = nums.clone();

    let mut idx = 0;

    while wip.len() != 1 {
        let mut bit = most_bits(&wip, idx).unwrap_or(target);

        let wtf = most_bits(&wip, idx);

        if reverse {
            if bit == '1' {
                bit = '0'
            } else {
                bit = '1'
            }
        }

        if wtf.is_none() {
            bit = target;
        }

        wip = wip
            .iter()
            .map(|s| *s)
            .filter(|n| {
                return n.chars().nth(idx).unwrap() == bit;
            })
            .collect();
        // println!("{:?}", wip);
        idx += 1;
    }

    return String::from_str(wip[0]).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{first, second};

    #[test]
    fn test() {
        let input = include_str!("../input.test");
        let nums: Vec<&str> = input.lines().collect();

        assert_eq!(first(&nums), 198);
        assert_eq!(second(&nums), 230);
    }
}
