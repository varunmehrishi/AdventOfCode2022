use crate::error::Error;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Cube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Cube {
    pub fn neighbors(&self) -> [Self; 6] {
        [
            self.n(1, 0, 0),
            self.n(0, 1, 0),
            self.n(0, 0, 1),
            self.n(-1, 0, 0),
            self.n(0, -1, 0),
            self.n(0, 0, -1),
        ]
    }

    fn n(&self, dx: i32, dy: i32, dz: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }
}

impl TryFrom<String> for Cube {
    type Error = crate::error::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut it = s.trim().split(',');
        let x = it
            .next()
            .ok_or(Error::Generic(format!("Could not parse {s}")))?
            .parse()?;
        let y = it
            .next()
            .ok_or(Error::Generic(format!("Could not parse {s}")))?
            .parse()?;
        let z = it
            .next()
            .ok_or(Error::Generic(format!("Could not parse {s}")))?
            .parse()?;
        Ok(Cube { x, y, z })
    }
}
