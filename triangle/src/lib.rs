#[derive(PartialEq)]
pub enum Triangle {
    Equilateral,
    Isosceles,
    Scalene,
}

impl Triangle {
    pub fn build(mut sides: [u32; 3]) -> Result<Self, String> {
        sides.sort();
        let sides = (sides[0], sides[1], sides[2]);
        match sides {
            (0, 0, 0) => Err("Invalid triangle".into()),
            (a, b, c) if a + b < c => Err("Invalid triangle".into()),
            (a, b, c) if a == b && b == c => Ok(Triangle::Equilateral),
            (a, b, c) if a == b || b == c || a == c => Ok(Triangle::Isosceles),
            (_, _, _) => Ok(Triangle::Scalene),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        *self == Triangle::Equilateral
    }

    pub fn is_isosceles(&self) -> bool {
        *self == Triangle::Isosceles
    }

    pub fn is_scalene(&self) -> bool {
        *self == Triangle::Scalene
    }
}
