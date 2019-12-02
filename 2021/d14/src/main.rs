use std::{collections::HashMap, str::FromStr};

struct Template {
    initial: String,
    insertions: HashMap<(char, char), char>,
}

impl FromStr for Template {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (init, instrs) = s.split_once("\n\n").unwrap();

        let insertions: HashMap<(char, char), char> = instrs
            .lines()
            .map(|l| {
                let (from, to) = l.split_once(" -> ").unwrap();
                let mut chars = from.chars();
                let a = chars.next().unwrap();
                let b = chars.next().unwrap();
                return ((a, b), to.chars().next().unwrap());
            })
            .collect();

        Ok(Template {
            initial: String::from_str(init).unwrap(),
            insertions,
        })
    }
}

fn main() {
    let input = include_str!("../input");
    let template: Template = input.parse().unwrap();

    first(&template);
    second(&template);
}

fn get_pairs(s: &str) -> Vec<(char, char)> {
    s.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|e| return (e[0], e[1]))
        .collect()
}

fn expand(template: &Template, iterations: u32) -> HashMap<(char, char), u64> {
    let pairs = get_pairs(&template.initial);
    let mut expansions: HashMap<(char, char), u64> = HashMap::new();

    for p in pairs {
        *expansions.entry(p).or_insert(0) += 1;
    }

    for _ in 0..iterations {
        let mut new_expansions: HashMap<(char, char), u64> = HashMap::new();

        for (k, n) in expansions {
            let chr = template.insertions.get(&k);
            match chr {
                Some(new) => {
                    *new_expansions.entry((k.0, *new)).or_insert(0) += n;
                    *new_expansions.entry((*new, k.1)).or_insert(0) += n;
                }
                _ => continue,
            }
        }

        expansions = new_expansions;
    }

    return expansions;
}

fn count_chars(template: &Template, expansions: &HashMap<(char, char), u64>) -> HashMap<char, u64> {
    let mut char_count: HashMap<char, u64> = HashMap::new();

    let first_char = template.initial.chars().next().unwrap();

    *char_count.entry(first_char).or_insert(0) += 1;

    for (x, n) in expansions {
        let chr = x.1;
        *char_count.entry(chr).or_insert(0) += n;
    }

    return char_count;
}

fn calc_strength(counts: &HashMap<char, u64>) -> u64 {
    let mut sorted: Vec<&u64> = counts.values().collect();
    sorted.sort();

    let least = sorted.first().unwrap();
    let most = sorted.last().unwrap();
    return **most - **least;
}

fn first(template: &Template) -> u64 {
    let expansions = expand(template, 10);

    let strength = calc_strength(&count_chars(template, &expansions));

    println!("{}", strength);
    return strength;
}

fn second(template: &Template) -> u64 {
    let expansions = expand(template, 40);

    let strength = calc_strength(&count_chars(template, &expansions));

    println!("{}", strength);
    return strength;
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Template};

    #[test]
    fn test() {
        let input = include_str!("../input.test");

        let template: Template = input.parse().unwrap();

        assert_eq!(first(&template), 1588);
        assert_eq!(second(&template), 2188189693529);
    }
}
