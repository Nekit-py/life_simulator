use crate::field::Point;
use crate::traits::{Action, LookAround, Movable, Positionable, Tracker};
use std::fmt;

pub const MEAT_VIEW: char = '🍖';
pub const GRASS_VIEW: char = '🌱';

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
struct Food {
    view: char,
    position: Point,
}

impl fmt::Display for Food {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.view)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Meat(Food);
impl Meat {
    pub fn new(position: Point) -> Self {
        Self(Food {
            view: MEAT_VIEW,
            position,
        })
    }

    pub fn view(&self) -> char {
        self.0.view
    }
}

impl Positionable for Meat {
    fn get_position(&self) -> Point {
        self.0.position
    }

    fn set_position(&mut self, point: Point) {
        self.0.position = point;
    }
}

impl fmt::Display for Meat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Grass(Food);
impl Grass {
    pub fn new(position: Point) -> Self {
        Self(Food {
            view: GRASS_VIEW,
            position,
        })
    }

    pub fn view(&self) -> char {
        self.0.view
    }
}

impl fmt::Display for Grass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
impl Positionable for Grass {
    fn get_position(&self) -> Point {
        self.0.position
    }

    fn set_position(&mut self, point: Point) {
        self.0.position = point;
    }
}

impl Action for Meat {}
impl LookAround for Meat {}
impl Tracker for Meat {
    fn reset_track(&mut self) {}
    fn insert_point(&mut self, point: Point) {}
    fn track_contains(&self, point: &Point) -> Option<bool> {
        None
    }
}

impl Movable for Meat {}
impl Action for Grass {}
impl LookAround for Grass {}
impl Tracker for Grass {
    fn reset_track(&mut self) {}
    fn insert_point(&mut self, point: Point) {}
    fn track_contains(&self, point: &Point) -> Option<bool> {
        None
    }
}
impl Movable for Grass {}
