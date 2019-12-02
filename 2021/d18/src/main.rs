use core::fmt;
use std::fmt::Formatter;

fn main() {
    let input = include_str!("../input");

    first();
    second();
}

#[derive(Debug)]
enum Token {
    Lparen,
    Rparen,
    Num(u32),
}

struct Tokenizer {
    tokens: Vec<Token>,
}

impl Tokenizer {
    fn new(s: &str) -> Self {
        let tokens = s
            .chars()
            .filter_map(|c| match c {
                '[' => Some(Token::Lparen),
                ']' => Some(Token::Rparen),
                '0'..='9' => Some(Token::Num(c.to_digit(10).unwrap())),
                _ => None,
            })
            .collect();

        Tokenizer { tokens }
    }

    fn iter(&self) -> impl Iterator<Item = &Token> {
        self.tokens.iter()
    }
}

#[derive(PartialEq, Eq, Clone)]
enum AST {
    Num(u32),
    Pair(Box<AST>, Box<AST>),
}

fn parse<'a>(tokens: &mut impl Iterator<Item = &'a Token>) -> AST {
    match tokens.next().unwrap() {
        Token::Num(n) => AST::Num(*n),
        _ => {
            let left = Box::new(parse(tokens));
            let right = Box::new(parse(tokens));

            tokens.next();
            return AST::Pair(left, right);
        }
    }
}

// fn leftmost_pair(ast: &AST) -> Option<&AST> {
//     match ast {
//         AST::Num(_) => None,
//         AST::Pair(left, right) => match (**left, **right) {
//             (AST::Num(_), AST::Num(_)) => Some(&AST::Pair(*left, *right)),
//             _ => {
//                 let is_left = leftmost_pair(left);
//                 return is_left.or_else(|| leftmost_pair(right));
//             }
//         },
//     }
// }

// fn explode_helper(ast: AST, lvl: u32, parent: AST, top: AST) -> AST {
//     if lvl == 5 {}
// }

// fn explode(ast: AST) -> AST {}

fn traverse_inorder(ast: &AST) -> Vec<&AST> {
    let mut stack = Vec::new();
    let mut path = Vec::new();

    stack.push(ast);

    while stack.len() > 0 {
        let curr = stack.pop().unwrap();
        match curr {
            AST::Num(_) => path.push(curr),
            AST::Pair(left, right) => {
                stack.push(left);
                stack.push(right);
            }
        }
    }

    return path;
}

fn first() -> u32 {
    return 0;
}

fn second() -> u32 {
    return 0;
}

impl fmt::Debug for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::Num(n) => {
                write!(f, " {} ", n);
            }
            AST::Pair(left, right) => {
                write!(f, "(");
                write!(f, "{:?}", left);
                write!(f, "{:?}", right);
                write!(f, ")");
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, parse, second, traverse_inorder, Tokenizer, AST};

    #[test]
    fn test() {
        let input = include_str!("../input.test");

        let nums: Vec<AST> = input
            .lines()
            .map(|l| Tokenizer::new(l))
            .map(|mut t| parse(&mut t.iter()))
            .collect();

        println!("{:?}", nums);
        println!("{:?}", traverse_inorder(&nums[0]));

        assert_eq!(first(), 4140);
        assert_eq!(second(), 0);
    }
}
