extern crate pascals_triangle;

use pascals_triangle::*;

fn main() {
    let pt = PascalsTriangle::new(1);
    print!("{:?}", pt.rows());
}
