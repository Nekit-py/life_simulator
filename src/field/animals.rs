const BOAR_VIEW: char = '🐗';
const LION_VIEW: char = '🦁';
use crate::field::Point;
use std::fmt;

#[allow(dead_code)]
struct Animal {
    hunger: u8,
    health: u8,
    view: char,
    position: Point,
}

impl fmt::Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.view)?;
        Ok(())
    }
}

pub struct Boar(Animal);

impl Boar {
    pub fn new(position: Point) -> Self {
        Boar(Animal {
            view: BOAR_VIEW,
            position,
            health: 10,
            hunger: 5,
        })
    }
}

impl fmt::Display for Boar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

pub struct Lion(Animal);

impl Lion {
    pub fn new(position: Point) -> Self {
        Lion(Animal {
            view: LION_VIEW,
            position,
            health: 15,
            hunger: 7,
        })
    }
}

impl fmt::Display for Lion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
