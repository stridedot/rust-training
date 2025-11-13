use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul};

use anyhow::Result;

#[derive(Debug)]
struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

fn main() -> Result<()> {
    let a = Matrix {
        data: vec![1, 2, 3, 4, 5, 6],
        rows: 2,
        cols: 3,
    };
    let b = Matrix {
        data: vec![10, 11, 20, 21, 30, 31],
        rows: 3,
        cols: 2,
    };

    let c = multiply(&a, &b);
    println!("{:?}", c);
    assert_eq!(c.data, vec![140, 146, 320, 335]);

    Ok(())
}

// a:
// 1 2 3
// 4 5 6

// b:
// 10 11
// 20 21
// 30 31
fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Clone + Debug + Default,
{
    assert_eq!(a.cols, b.rows);

    let mut data = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] +=
                    a.data[i * a.cols + k].clone() * b.data[k * b.cols + j].clone();
            }
        }
    }
    Matrix {
        data,
        rows: a.rows,
        cols: b.cols,
    }
}
