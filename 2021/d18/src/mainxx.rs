use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Atom(char),
    Op(char),
    Eof,
}

struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut tokens: Vec<Token> = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c),
            })
            .collect();

        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

#[derive(Debug)]
enum S {
    Atom(char),
    Cons(char, Vec<S>),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?;
                }
                write!(f, ")")
            }
        }
    }
}

fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => S::Atom(it),
        Token::Op('(') => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        Token::Op(op) => {
            let ((), r_bp) = prefix_binding_power(op);
            let rhs = expr_bp(lexer, r_bp);
            S::Cons(op, vec![rhs])
        }
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        if let Some((l_bp, ())) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            lhs = S::Cons(op, vec![lhs]);
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp);
            lhs = S::Cons(op, vec![lhs, rhs]);

            continue;
        }

        break;
    }

    lhs
}

fn postfix_binding_power(op: char) -> Option<(u8, ())> {
    let res = match op {
        '!' => (7, ()),
        _ => return None,
    };

    Some(res)
}

fn prefix_binding_power(op: char) -> ((), u8) {
    match op {
        '+' | '-' => ((), 5),
        _ => panic!("bad op: {:?}", op),
    }
}

fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    let res = match op {
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
        '.' => (10, 9),
        _ => return None,
    };

    Some(res)
}

fn main() {}

#[test]
fn tests() {
    let s = expr("1");
    assert_eq!(s.to_string(), "1");

    let s = expr("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

    let s = expr("f . g . h");
    assert_eq!(s.to_string(), "(. f (. g h))");

    let s = expr(" 1 + 2 + f . g . h * 3 * 4");
    assert_eq!(s.to_string(), "(+ (+ 1 2) (* (* (. f (. g h)) 3) 4))");

    let s = expr("--1 * 2");
    assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

    let s = expr("--f . g");
    assert_eq!(s.to_string(), "(- (- (. f g)))");

    let s = expr("-9!");
    assert_eq!(s.to_string(), "(- (! 9))");

    let s = expr("f . g !");
    assert_eq!(s.to_string(), "(! (. f g))");

    let s = expr("(((0)))");
    assert_eq!(s.to_string(), "0");

    let s = expr("(((0 + 2) + 3) + 4)");
    println!("{:?}", s);
}
