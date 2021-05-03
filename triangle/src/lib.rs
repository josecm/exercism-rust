#[derive(PartialEq)]
pub enum Triangle {
    Equilateral,
    Scalene,
    Isosceles,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|&s| s == 0) {
            return None;
        }

        let pairs = sides.iter().zip(sides.iter().cycle().skip(1));

        if pairs
            .clone()
            .zip(sides.iter().cycle().skip(2))
            .map(|((s1, s2), &s3)| s1 + s2 <= s3)
            .any(|c| c)
        {
            return None;
        }

        match pairs.map(|(s1, s2)| s1 == s2).filter(|&eq| eq).count() {
            3 => Some(Triangle::Equilateral),
            1 => Some(Triangle::Isosceles),
            0 => Some(Triangle::Scalene),
            _ => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        *self == Triangle::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        *self == Triangle::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        *self == Triangle::Isosceles
    }
}
