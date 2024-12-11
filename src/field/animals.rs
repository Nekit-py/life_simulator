use crate::field::Point;
use crate::traits::{LookAround, Movable, Positionable};
use core::option::Option;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::fmt;

const BOAR_VIEW: char = '🐗';
const LION_VIEW: char = '🦁';

///Нпарвления вижения
#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

///Абстрактная структура животное
#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
struct Animal {
    hunger: u8,
    health: u8,
    view: char,
    shifted: bool,
    position: Point,
}

///Отображение животного по моджи
impl fmt::Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.view)?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
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

impl Movable for Boar {
    ///Следование в случайном направлении на 1 клетку
    ///возвращает опционально точку, в которую будет перемещено животное(если такая существует)
    fn move_to(&mut self, directions: Vec<Direction>) -> Option<Point> {
        if directions.is_empty() {
            return None;
        }
        let (cur_x, cur_y) = self.0.position.coords();
        let mut rng = thread_rng();
        let direction = directions.choose(&mut rng).unwrap();
        match direction {
            Direction::Up => Some(Point::new(cur_x, cur_y + 1)),
            Direction::Down => Some(Point::new(cur_x, cur_y - 1)),
            Direction::Left => Some(Point::new(cur_x - 1, cur_y)),
            Direction::Right => Some(Point::new(cur_x + 1, cur_y)),
        }
    }

    fn made_a_move(&mut self) {
        self.0.shifted = true;
    }

    fn mark_as_immovable(&mut self) {
        self.0.shifted = false;
    }

    fn is_moved(&self) -> bool {
        self.0.shifted
    }
}

impl LookAround for Boar {}
impl LookAround for Lion {}

impl fmt::Display for Boar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
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
impl Movable for Lion {
    fn move_to(&mut self, directions: Vec<Direction>) -> Option<Point> {
        if directions.is_empty() {
            return None;
        }
        let (cur_x, cur_y) = self.0.position.coords();
        let mut rng = thread_rng();
        let direction = directions.choose(&mut rng).unwrap();
        match direction {
            Direction::Up => Some(Point::new(cur_x, cur_y + 1)),
            Direction::Down => Some(Point::new(cur_x, cur_y - 1)),
            Direction::Left => Some(Point::new(cur_x - 1, cur_y)),
            Direction::Right => Some(Point::new(cur_x + 1, cur_y)),
        }
    }

    fn made_a_move(&mut self) {
        self.0.shifted = true;
    }

    fn mark_as_immovable(&mut self) {
        self.0.shifted = false;
    }

    fn is_moved(&self) -> bool {
        self.0.shifted
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
