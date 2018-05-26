extern crate num;
use num::Num;
use num::Zero;

use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene,
}

#[derive(Debug)]
pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
    kind: TriangleType,
}

impl<T: Num + Copy + PartialOrd> Triangle<T> {
    pub fn build(mut sides: [T; 3]) -> Result<Self, String> {
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
        let sides = (sides[0], sides[1], sides[2]);
        match sides {
            (a, b, c) if Zero::is_zero(&a) || Zero::is_zero(&b) || Zero::is_zero(&c) => {
                Err("Invalid triangle".into())
            }
            (a, b, c) if a + b < c => Err("Invalid triangle".into()),
            (a, b, c) if a == b && b == c => Ok(Self::construct(sides, TriangleType::Equilateral)),
            (a, b, c) if a == b || b == c || a == c => {
                Ok(Self::construct(sides, TriangleType::Isosceles))
            }
            sides => Ok(Self::construct(sides, TriangleType::Scalene)),
        }
    }
    pub fn is_equilateral(&self) -> bool {
        self.kind == TriangleType::Equilateral
    }

    pub fn is_isosceles(&self) -> bool {
        self.kind == TriangleType::Isosceles
    }

    pub fn is_scalene(&self) -> bool {
        self.kind == TriangleType::Scalene
    }

    fn construct(sides: (T, T, T), kind: TriangleType) -> Self {
        Triangle {
            a: sides.0,
            b: sides.1,
            c: sides.2,
            kind: kind,
        }
    }
}
