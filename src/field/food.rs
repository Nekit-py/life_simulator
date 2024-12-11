use crate::field::Point;
use std::fmt;

const MEAT_VIEW: char = '🍖';
const GRASS_VIEW: char = '🌱';

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Food {
    view: char,
    postition: Point,
}

impl fmt::Display for Food {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.view)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Meat(Food);
impl Meat {
    pub fn new(postition: Point) -> Self {
        Self(Food {
            view: MEAT_VIEW,
            postition,
        })
    }
}

impl fmt::Display for Meat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Grass(Food);
impl Grass {
    pub fn new(postition: Point) -> Self {
        Self(Food {
            view: GRASS_VIEW,
            postition,
        })
    }
}

impl fmt::Display for Grass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
