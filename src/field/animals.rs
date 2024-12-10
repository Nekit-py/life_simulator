const BOAR_VIEW: char = '🐗';
const LION_VIEW: char = '🦁';
use crate::field::Point;
use crate::traits::{Movable, Positionable};
use crate::Field;
use std::fmt;

///Нпарвления хода
pub enum Direction {
    Left,
    Right,
    Uo,
    Down,
}

#[allow(dead_code)]
struct Animal {
    hunger: u8,
    health: u8,
    view: char,
    shifted: bool,
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
            shifted: false,
            position,
            health: 10,
            hunger: 5,
        })
    }
}

impl Positionable for Boar {
    fn get_position(&self) -> Point {
        self.0.position
    }
}

// impl Movable for Boar {
//     fn up(&mut self, field: &mut Field) {
//         let (height, width) = field.size();
//         if self.0.position.y < height {
//             todo!();
//         }
//     }
// }

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
            shifted: false,
            position,
            health: 15,
            hunger: 7,
        })
    }
}

impl Positionable for Lion {
    fn get_position(&self) -> Point {
        self.0.position
    }
}

impl fmt::Display for Lion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
