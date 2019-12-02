fn main() {
    let input = include_str!("../input");
    let state: Vec<u32> = input.split(",").map(|s| s.parse().unwrap()).collect();

    first(&state);
    second(&state);
}

fn step(state: &Vec<u32>, iterations: u32) -> usize {
    let mut days: Vec<usize> = vec![0; 9];

    for fish in state {
        days[*fish as usize] += 1;
    }

    for _i in 0..iterations {
        let new = days[0];

        days.rotate_left(1);
        days[6] += new;
    }

    let count = days.iter().sum();

    return count;
}

fn first(state: &Vec<u32>) -> usize {
    let count = step(state, 80);
    println!("{}", count);
    return count;
}

fn second(state: &Vec<u32>) -> usize {
    let count = step(state, 256);
    println!("{}", count);
    return count;
}

#[cfg(test)]
mod tests {
    use crate::step;

    #[test]
    fn test() {
        let input = include_str!("../input.test");
        let state: Vec<u32> = input.split(",").map(|s| s.parse().unwrap()).collect();

        assert_eq!(step(&state, 18), 26);
        assert_eq!(step(&state, 80), 5934);
        assert_eq!(step(&state, 256), 26984457539);
    }
}
