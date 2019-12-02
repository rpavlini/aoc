use std::str::FromStr;

enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
}

type Matrix = Vec<Vec<bool>>;

struct Paper {
    matrix: Matrix,
    folds: Vec<Fold>,
}

impl FromStr for Paper {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (points, fold_instr) = s.split_once("\n\n").unwrap();

        let mut rows: usize = 0;
        let mut cols: usize = 0;

        let mut folds = Vec::new();
        for f in fold_instr.lines() {
            let mut f = f.split(" ");
            let d = f.nth(2).unwrap();
            let (dir, amt) = d.split_once("=").unwrap();
            let amt: usize = amt.parse().unwrap();
            let fold = match dir {
                "x" => {
                    cols = cols.max(amt);
                    Fold::X(amt)
                }
                "y" => {
                    rows = rows.max(amt);
                    Fold::Y(amt)
                }
                _ => panic!("unknown dir {}", d),
            };

            folds.push(fold);
        }

        let mut matrix = make_matrix(rows * 2 + 1, cols * 2 + 1);

        for p in points.lines() {
            let (x, y) = p.split_once(",").unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();

            matrix[y][x] = true;
        }

        return Ok(Paper { matrix, folds });
    }
}

fn main() {
    let input = include_str!("../input");
    let paper: Paper = input.parse().unwrap();
    let paper2: Paper = input.parse().unwrap();

    first(paper);
    second(paper2);
}

fn count_points(matrix: &Matrix) -> u32 {
    return matrix
        .iter()
        .map(|r| r.iter())
        .flatten()
        .filter(|v| **v)
        .count() as u32;
}

fn split_matrix(matrix: &Matrix, at: usize, axis: Axis) -> (Matrix, Matrix) {
    match axis {
        Axis::Y => {
            let first: Matrix = matrix.clone().iter().map(|v| v.clone()).take(at).collect();
            let second: Matrix = matrix
                .clone()
                .iter()
                .skip(at + 1)
                .map(|v| v.clone())
                .collect();

            return (first, second);
        }
        Axis::X => {
            let first: Matrix = matrix
                .clone()
                .iter()
                .map(|v| v.clone().iter().take(at).map(|v| *v).collect())
                .collect();
            let second: Matrix = matrix
                .clone()
                .iter()
                .map(|v| v.clone().iter().skip(at + 1).map(|v| *v).collect())
                .collect();

            return (first, second);
        }
    }
}

fn second(paper: Paper) {
    let max_folds = &paper.folds.len();
    let folded = fold(paper, *max_folds);

    print_matrix(&folded);
}

fn first(paper: Paper) -> u32 {
    let folded = fold(paper, 1);

    let pts = count_points(&folded);
    println!("{}", pts);

    return pts;
}

fn fold(paper: Paper, times: usize) -> Matrix {
    let mut matrix = paper.matrix;

    for f in paper.folds.iter().take(times) {
        let (axis, mid) = match f {
            Fold::X(mid) => (Axis::X, *mid),
            Fold::Y(mid) => (Axis::Y, *mid),
        };

        let (a, b) = split_matrix(&matrix, mid, axis);
        let b_mirror = mirror(&b, axis);

        matrix = overlap(&a, &b_mirror);
    }

    return matrix;
}

fn overlap(a: &Matrix, b: &Matrix) -> Matrix {
    let rows = a.len();
    let cols = a[0].len();

    let mut wip = make_matrix(rows, cols);

    for r in 0..rows {
        for c in 0..cols {
            wip[r][c] = a[r][c] || b[r][c];
        }
    }

    return wip;
}

fn mirror(matrix: &Matrix, axis: Axis) -> Matrix {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut wip = make_matrix(rows, cols);

    match axis {
        Axis::X => {
            for (i, row) in matrix.iter().enumerate() {
                for (j, v) in row.iter().rev().enumerate() {
                    wip[i][j] = *v;
                }
            }
        }
        Axis::Y => {
            for (i, row) in matrix.iter().rev().enumerate() {
                for (j, v) in row.iter().enumerate() {
                    wip[i][j] = *v;
                }
            }
        }
    }

    return wip;
}

fn print_matrix(matrix: &Matrix) {
    for r in matrix {
        let compact: Vec<char> = r.iter().map(|&v| if v { '█' } else { ' ' }).collect();
        let mut s = String::new();
        for c in compact {
            s.push(c);
        }
        println!("{}", s);
    }
}

fn make_matrix(rows: usize, cols: usize) -> Matrix {
    let mut matrix = Vec::new();

    for _ in 0..rows {
        let mut row = Vec::new();
        for _ in 0..cols {
            row.push(false);
        }
        matrix.push(row);
    }

    return matrix;
}

#[cfg(test)]
mod tests {
    use crate::{first, Paper};

    #[test]
    fn test() {
        let input = include_str!("../input.test");
        let paper: Paper = input.parse().unwrap();

        assert_eq!(first(paper), 17);
    }
}
