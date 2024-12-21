use crate::entities::Entities;
use crate::field::Point;
use crate::traits::{Action, Health, LookAround, Movable, Positionable, Satiety, Tracker};
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

impl Satiety for Virus {
    fn get_hunger(&self) -> u8 {
        0
    }

    fn set_hunger(&mut self, hunger: u8) {}

    fn is_hungry(&self) -> bool {
        false
    }

    fn is_fed(&self) -> bool {
        false
    }
}

impl Health for Virus {
    fn get_health(&self) -> u8 {
        0
    }

    fn set_health(&mut self, health: u8) {}

    fn heal(&mut self) {}

    fn is_alive(&self) -> Option<bool> {
        None
    }
}

impl Tracker for Virus {
    fn reset_track(&mut self) {}
    fn insert_point(&mut self, point: Point) {}
    fn track_contains(&self, point: &Point) -> Option<bool> {
        None
    }
}

impl Action for Virus {}
impl LookAround for Virus {
    fn choose_priority_point(
        &mut self,
        available_points: Vec<Point>,
        entities: &Entities,
    ) -> Option<Point> {
        None
    }
}
impl Movable for Virus {
    fn is_moved(&self) -> bool {
        false
    }

    fn move_allowed(&mut self, allow: bool) {}
}

impl Action for Wasteland {}
impl LookAround for Wasteland {
    fn choose_priority_point(
        &mut self,
        available_points: Vec<Point>,
        entities: &Entities,
    ) -> Option<Point> {
        None
    }
}
impl Movable for Wasteland {
    fn is_moved(&self) -> bool {
        false
    }

    fn move_allowed(&mut self, allow: bool) {}
}

impl Tracker for Wasteland {
    fn reset_track(&mut self) {}
    fn insert_point(&mut self, point: Point) {}
    fn track_contains(&self, point: &Point) -> Option<bool> {
        None
    }
}

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

impl Satiety for Wasteland {
    fn get_hunger(&self) -> u8 {
        0
    }

    fn set_hunger(&mut self, hunger: u8) {}

    fn is_hungry(&self) -> bool {
        false
    }

    fn is_fed(&self) -> bool {
        false
    }
}

impl Health for Wasteland {
    fn get_health(&self) -> u8 {
        0
    }

    fn set_health(&mut self, health: u8) {}

    fn heal(&mut self) {}

    fn is_alive(&self) -> Option<bool> {
        None
    }
}

impl fmt::Display for Wasteland {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
