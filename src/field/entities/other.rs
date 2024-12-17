use crate::field::Point;
use crate::traits::{Action, Positionable};
use std::fmt;

pub const VIRUS_VIEW: char = '🦠';
pub const WASTELAND_VIEW: char = '⬛';

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
struct Object {
    view: char,
    position: Point,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.view)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Virus(Object);
impl Virus {
    pub fn new(position: Point) -> Self {
        Self(Object {
            view: VIRUS_VIEW,
            position,
        })
    }

    pub fn view(&self) -> char {
        self.0.view
    }
}

impl Action for Virus {}
impl Action for Wasteland {}

impl Positionable for Virus {
    fn get_position(&self) -> Point {
        self.0.position
    }

    fn set_position(&mut self, point: Point) {
        self.0.position = point;
    }
}

impl fmt::Display for Virus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Wasteland(Object);
impl Wasteland {
    pub fn new(position: Point) -> Self {
        Self(Object {
            view: WASTELAND_VIEW,
            position,
        })
    }

    pub fn view(&self) -> char {
        self.0.view
    }
}

impl Positionable for Wasteland {
    fn get_position(&self) -> Point {
        self.0.position
    }

    fn set_position(&mut self, point: Point) {
        self.0.position = point;
    }
}

impl fmt::Display for Wasteland {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
