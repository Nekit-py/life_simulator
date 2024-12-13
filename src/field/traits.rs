use super::animals::Direction;
use crate::Point;
use core::option::Option;

pub trait Positionable {
    fn get_position(&self) -> Point;
    fn set_position(&mut self, point: Point);
}

pub trait Action: Positionable + std::fmt::Display + Sized {
    fn action(&mut self, height: usize, width: usize) {}
}

pub trait Movable {
    fn move_to(&mut self, directions: Vec<Direction>) -> Option<Point>;
    fn mark_as_movable(&mut self);
    fn mark_as_immovable(&mut self);
    fn is_moved(&self) -> bool;
}

//Проверка возможных направлений движения
pub trait LookAround: Positionable {
    fn look_around(&self, size: (usize, usize)) -> Vec<Direction> {
        //Возможные направления
        let mut directions: Vec<Direction> = vec![];
        //Размеры поля
        let (height, width) = size;
        //Текущая точка кабана
        let cur_pos = self.get_position();
        //текущие Координаты животного
        let (cur_x, cur_y) = cur_pos.coords();

        if cur_y > 0 {
            directions.push(Direction::Up);
        }
        if cur_y < height - 1 {
            // Индексация начинается с 0
            directions.push(Direction::Down);
        }

        // Проверка по оси X (ширина)
        if cur_x > 0 {
            directions.push(Direction::Left);
        }
        if cur_x < width - 1 {
            // Индексация начинается с 0
            directions.push(Direction::Right);
        }
        directions
    }
}
