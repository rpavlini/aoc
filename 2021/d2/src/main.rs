use std::str::FromStr;

enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Direction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let dir = parts[0];
        let amt = parts[1].parse::<u32>()?;

        Ok(match dir {
            "forward" => Direction::Forward(amt),
            "down" => Direction::Down(amt),
            "up" => Direction::Up(amt),
            _ => panic!("unknown dir"),
        })
    }
}

fn main() {
    let input = include_str!("../input");
    let directions: Vec<Direction> = input.lines().map(|l| l.parse().unwrap()).collect();

    first(&directions);
    second(&directions);
}

fn first(directions: &Vec<Direction>) -> u32 {
    let (hor, depth) = directions
        .iter()
        .fold((0, 0), |(hor, depth), curr| match curr {
            Direction::Forward(x) => (hor + x, depth),
            Direction::Up(x) => (hor, depth - x),
            Direction::Down(x) => (hor, depth + x),
        });

    println!("{}", hor * depth);
    hor * depth
}

fn second(directions: &Vec<Direction>) -> u32 {
    let (_aim, hor, depth) =
        directions
            .iter()
            .fold((0, 0, 0), |(aim, hor, depth), curr| match curr {
                Direction::Forward(x) => (aim, hor + x, depth + aim * x),
                Direction::Up(x) => (aim - x, hor, depth),
                Direction::Down(x) => (aim + x, hor, depth),
            });

    println!("{}", hor * depth);
    hor * depth
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Direction};

    #[test]
    fn test() {
        let input = include_str!("../input.test");
        let directions: Vec<Direction> = input.lines().map(|l| l.parse().unwrap()).collect();

        assert_eq!(first(&directions), 150);
        assert_eq!(second(&directions), 900);
    }
}
