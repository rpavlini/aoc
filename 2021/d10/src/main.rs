use lazy_static::lazy_static;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    let lines: Vec<&str> = input.lines().collect();

    first(&lines);
    second(&lines);
}

lazy_static! {
    static ref PAIRS: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
}

fn calc_err_score(line: &str) -> u32 {
    let mut stack: Vec<char> = Vec::new();
    let mut last_open: Vec<char> = Vec::new();
    let mut illegal: Vec<char> = Vec::new();

    for c in line.chars() {
        if PAIRS.contains_key(&c) {
            last_open.push(c);
            stack.push(c);
        } else {
            if *PAIRS.get(last_open.last().unwrap()).unwrap() == c {
                last_open.pop();
            } else {
                illegal.push(c);
            }
            stack.pop();
        }
    }

    let first_illegal = illegal.iter().next();

    match first_illegal {
        None => 0,
        Some(c) => match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        },
    }
}

// guaranteed to be in correct order
fn calc_completion_score(line: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if PAIRS.contains_key(&c) {
            stack.push(c);
        } else {
            stack.pop();
        }
    }

    let mut score: u64 = 0;
    for c in stack.iter().rev() {
        score *= 5;

        score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        };
    }

    return score;
}

fn first(lines: &Vec<&str>) -> u32 {
    let mut score = 0;

    for l in lines {
        score += calc_err_score(l);
    }

    println!("{}", score);
    return score;
}

fn second(lines: &Vec<&str>) -> u64 {
    let mut scores: Vec<u64> = Vec::new();

    for l in lines {
        let err = calc_err_score(l);
        if err != 0 {
            continue;
        } else {
            let score = calc_completion_score(l);
            scores.push(score);
        }
    }

    scores.sort();
    let mid_score = scores[(scores.len() / 2)];

    println!("{:?}", mid_score);
    return mid_score;
}

#[cfg(test)]
mod tests {
    use crate::{first, second};

    #[test]
    fn test() {
        let input = include_str!("../input.test");
        let lines: Vec<&str> = input.lines().collect();

        assert_eq!(first(&lines), 26397);
        assert_eq!(second(&lines), 288957);
    }
}
