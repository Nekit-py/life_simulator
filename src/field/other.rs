use crate::field::Point;
use std::fmt;

pub const VIRUS_VIEW: char = '🦠';
pub const WASTELAND_VIEW: char = '⬛';

#[allow(dead_code)]
struct Object {
    view: char,
    postition: Point,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.view)?;
        Ok(())
    }
}

pub struct Virus(Object);
impl Virus {
    pub fn new(postition: Point) -> Self {
        Self(Object {
            view: VIRUS_VIEW,
            postition,
        })
    }
}

impl fmt::Display for Virus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

pub struct Wasteland(Object);
impl Wasteland {
    pub fn new(postition: Point) -> Self {
        Self(Object {
            view: WASTELAND_VIEW,
            postition,
        })
    }
}

impl fmt::Display for Wasteland {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
