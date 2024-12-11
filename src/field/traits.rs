use core::option::Option;

use crate::{Field, Point};

use super::animals::Direction;

pub trait Positionable {
    fn get_position(&self) -> Point;
}

pub trait Movable {
    fn go_to_direction(&mut self, directions: Vec<Direction>) -> Option<Point>;
    fn made_a_move(&mut self);
    fn mark_as_immovable(&mut self);
    fn is_moved(&self) -> bool;
}

//Проверка возможных направлений движения
pub trait LookAround: Positionable {
    // fn look_around(&self, size: (usize, usize)) -> Vec<Direction>;
    fn look_around(&self, size: (usize, usize)) -> Vec<Direction> {
        //Возможные направления
        let mut directions: Vec<Direction> = vec![];
        //Размеры поля
        let (height, width) = size;
        //Текущая точка кабана
        let cur_pos = self.get_position();
        //текущие Координаты кабана
        let (cur_x, cur_y) = cur_pos.coords();
        if cur_y > 0 {
            directions.push(Direction::Down)
        } else if cur_y < height {
            directions.push(Direction::Up)
        }
        if cur_x > 0 {
            directions.push(Direction::Left)
        } else if cur_x < width {
            directions.push(Direction::Right)
        }
        directions
    }
}
