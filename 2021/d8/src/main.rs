use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

type DigitMap = HashMap<u32, HashSet<char>>;

struct Entry<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
}

static UNIQ_LENGTHS: [usize; 4] = [2, 3, 4, 7];

impl<'a> From<&'a str> for Entry<'a> {
    fn from(s: &'a str) -> Entry<'a> {
        let mut split = s.split(" | ");
        let input = split.next().unwrap();
        let output = split.next().unwrap();

        return Entry {
            input: input.split(" ").collect(),
            output: output.split(" ").collect(),
        };
    }
}

fn main() {
    let input = include_str!("../input");
    let entries: Vec<Entry> = input.lines().map(|l| l.into()).collect();

    first(&entries);
    second(&entries);
}

fn first(entries: &Vec<Entry>) -> usize {
    let uniq = entries
        .iter()
        .map(|e| &e.output)
        .flatten()
        .filter(|s| UNIQ_LENGTHS.contains(&s.len()))
        .count();

    println!("{}", uniq);
    return uniq;
}

fn deduce(signals: &Vec<&str>) -> DigitMap {
    // find 1,4,7,8
    // use 1 & 4 to find 2,3,5 from those with length 5
    // use 1 & 4 to find 0,6,9 from those with length 6
    let mut digits: DigitMap = HashMap::new();

    for s in signals {
        let digit_set = HashSet::from_iter(s.chars());
        match s.len() {
            2 => digits.insert(1, digit_set),
            3 => digits.insert(7, digit_set),
            4 => digits.insert(4, digit_set),
            7 => digits.insert(8, digit_set),
            _ => continue,
        };
    }

    if digits.len() != 4 {
        panic!("failed to deduce unique digits");
    }

    for s in signals {
        let digit_set = HashSet::from_iter(s.chars());
        let int_one = digit_set.intersection(digits.get(&1).unwrap());
        let int_four = digit_set.intersection(digits.get(&4).unwrap());

        match s.len() {
            5 => {
                if int_one.count() == 2 {
                    digits.insert(3, digit_set);
                } else if &int_four.count() == &2 {
                    digits.insert(2, digit_set);
                } else {
                    digits.insert(5, digit_set);
                }
            }
            6 => {
                if &int_one.count() == &1 {
                    digits.insert(6, digit_set);
                } else if &int_four.count() == &3 {
                    digits.insert(0, digit_set);
                } else {
                    digits.insert(9, digit_set);
                }
            }
            _ => continue,
        };
    }

    return digits;
}

fn sum(map: &DigitMap, chars: &Vec<&str>) -> u64 {
    let mut value = String::new();

    for o in chars {
        let chars = HashSet::from_iter(o.chars());

        let digit = map
            .iter()
            .find(|(_k, v)| v.difference(&chars).count() == 0 && chars.len() == v.len())
            .unwrap();
        value.push(digit.0.to_string().chars().next().unwrap());
    }

    let dig = value.parse::<u64>().unwrap();
    return dig;
}

fn second(entries: &Vec<Entry>) -> u64 {
    let mut total: u64 = 0;

    for e in entries {
        let signals = e.output.iter().chain(e.input.iter()).map(|s| *s);
        let map = deduce(&signals.collect());
        let sum = sum(&map, &e.output);

        total += sum;
    }

    println!("{}", total);
    return total;
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Entry};

    #[test]
    fn test() {
        let input = include_str!("../input.test");

        let entries: Vec<Entry> = input.lines().map(|l| l.into()).collect();

        assert_eq!(first(&entries), 26);
        assert_eq!(second(&entries), 61229);
    }
}
