use std::{
    char,
    fmt::{Display, Write},
};

#[derive(Clone, Copy)]
pub enum Particle {
    Air,
    Sand,
    Rock,
}

impl Display for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}

impl From<Particle> for char {
    fn from(value: Particle) -> Self {
        match value {
            Particle::Air => '🟦',
            Particle::Sand => '🟫',
            Particle::Rock => '⬜',
        }
    }
}
