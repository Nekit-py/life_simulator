use crate::field::Point;
use crate::traits::{Action, LookAround, Movable, Positionable};
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

impl LookAround for Boar {}

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

    fn set_position(&mut self, point: Point) {
        self.0.position = point;
    }
}

impl Movable for Boar {
    ///Следование в случайном направлении на 1 клетку
    ///возвращает опционально точку, в которую будет перемещено животное(если такая существует)
    ///Должен проверять, что есть в области видимостьи:
    /// - если травка, то идем к травке,
    /// - если лев, то от него,
    /// - в остальных случаях рандомное направление за исклюеним пройденного пути
    fn move_to(&mut self, directions: Vec<Direction>) -> Option<Point> {
        if directions.is_empty() {
            return None;
        }
        let (cur_x, cur_y) = self.0.position.coords();
        let mut rng = thread_rng();
        let direction = directions.choose(&mut rng).unwrap();
        println!("Выбрано направление -> {:?}", direction);
        match direction {
            Direction::Up => Some(Point::new(cur_x, cur_y - 1)),
            Direction::Down => Some(Point::new(cur_x, cur_y + 1)),
            Direction::Left => Some(Point::new(cur_x - 1, cur_y)),
            Direction::Right => Some(Point::new(cur_x + 1, cur_y)),
        }
    }

    fn mark_as_movable(&mut self) {
        self.0.shifted = true;
    }

    //Поставить флаг, что сущность не двигалась
    fn mark_as_immovable(&mut self) {
        self.0.shifted = false;
    }

    //Проверяет совершалось ли движения
    fn is_moved(&self) -> bool {
        self.0.shifted
    }
}

impl Action for Boar {
    fn action(&mut self, height: usize, width: usize) {
        println!();
        println!("Текущая позиция: {:?}", self.get_position());
        if self.is_moved() {
            self.mark_as_immovable();
        } else {
            let available_directions = self.look_around((height, width));
            println!(
                "Доступные ходы {} -> {:?}",
                self.0.view, available_directions
            );

            if let Some(point_to_move) = self.move_to(available_directions) {
                self.set_position(point_to_move);
                // println!("Новая позиция: {:?}", self.get_position());
                self.mark_as_movable();
            }
        }
    }
}

impl fmt::Display for Boar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Lion(Animal);

impl LookAround for Lion {}

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

    fn mark_as_movable(&mut self) {
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

    fn set_position(&mut self, point: Point) {
        self.0.position = point;
    }
}

impl Action for Lion {
    fn action(&mut self, height: usize, width: usize) {
        if !self.is_moved() {
            let available_directions = { self.look_around((height, width)) };
            if let Some(point_to_move) = self.move_to(available_directions) {
                self.set_position(point_to_move);
                self.mark_as_movable();
            }
        } else {
            self.mark_as_immovable();
        }
    }
}

impl fmt::Display for Lion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
