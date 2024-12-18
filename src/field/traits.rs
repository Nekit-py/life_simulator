use rand::seq::SliceRandom;

use super::entities::Entities;
use crate::entities::food::GRASS_VIEW;
use crate::entities::other::WASTELAND_VIEW;
use crate::Point;
use core::option::Option::{self, Some};
use rand::thread_rng;

pub trait Positionable {
    fn get_position(&self) -> Point;
    fn set_position(&mut self, point: Point);
}

pub trait Action: Movable + std::fmt::Display {
    fn action(&mut self, height: usize, width: usize, entities: &Entities) {
        let arrival_point = self.move_to(height, width, entities);
        self.calculate_move_effects(arrival_point, entities);
    }

    fn calculate_move_effects(&mut self, arrival_point: Option<Point>, entities: &Entities) {}
}

pub trait Satiety {
    fn get_hunger(&self) -> u8;

    // Метод для установки уровня голода
    fn set_hunger(&mut self, hunger: u8);

    // Метод для голодания
    fn starve(&mut self) {
        let new_hunger = self.get_hunger().saturating_sub(1);
        self.set_hunger(new_hunger);
    }

    // Метод для поедания
    fn eat(&mut self) {
        let new_hunger = self.get_hunger().saturating_add(3);
        self.set_hunger(new_hunger.min(10));
    }

    fn is_hungry(&self) -> bool;

    fn is_fed(&self) -> bool;
}

pub trait Health {
    fn get_health(&self) -> u8;

    fn set_health(&mut self, health: u8);

    fn take_damage(&mut self, val: Option<u8>) {
        let new_hunger = match val {
            Some(val) => self.get_health().saturating_sub(val),
            None => self.get_health().saturating_sub(1),
        };
        self.set_health(new_hunger.max(0));
    }

    fn heal(&mut self) {
        let new_health = self.get_health().saturating_add(1);
        self.set_health(new_health.min(15));
    }

    fn is_alive(&self) -> Option<bool>;
}

pub trait Tracker {
    fn reset_track(&mut self);
    fn insert_point(&mut self, point: Point);
    fn track_contains(&self, point: &Point) -> Option<bool>;
}

pub trait Movable: LookAround {
    ///Следование в выбранном направлении на 1 клетку
    ///Должен проверять, что есть в области видимостьи:
    /// - если еда, то к еде,
    /// - если лев, к нему не идем, если мы кабан и наоборот)),
    /// - в остальных случаях рандомное направление за исклюеним пройденного пути
    fn move_to(&mut self, height: usize, width: usize, entities: &Entities) -> Option<Point> {
        self.reset_track();

        // Перемещаемся по приоритетной точке
        if let Some(point_to_move) = self.calculate_move(height, width, entities) {
            self.insert_point(point_to_move);
            self.set_position(point_to_move); // Обновляем позицию
            return Some(point_to_move);
        }
        None
    }
}

///Проверка возможных направлений движения
pub trait LookAround: Positionable + Tracker {
    fn calculate_move(
        &mut self,
        height: usize,
        width: usize,
        entities: &Entities,
    ) -> Option<Point> {
        //Возможные направления
        let mut available_points: Vec<Point> = Vec::with_capacity(4);
        //Текущая точка животного
        let cur_pos = self.get_position();
        //текущие координаты животного
        let (cur_x, cur_y) = cur_pos.coords();

        if cur_y > 0 {
            available_points.push(Point::new(cur_x, cur_y - 1));
        }

        if cur_y < height - 1 {
            // Индексация начинается с 0
            available_points.push(Point::new(cur_x, cur_y + 1));
        }

        // Проверка по оси X (ширина)
        if cur_x > 0 {
            available_points.push(Point::new(cur_x - 1, cur_y));
        }
        if cur_x < width - 1 {
            // Индексация начинается с 0
            available_points.push(Point::new(cur_x + 1, cur_y));
        }

        println!("Доступные точки для хода: {:?}", &available_points);
        match self.choose_priority_point(available_points, entities) {
            Some(point_to_move) => {
                println!("Выбранная точка для хода: {:?}", point_to_move);
                Some(point_to_move)
            }
            _ => {
                println!("Нет доступных ходов");
                None
            }
        }
    }

    /// В приоритете идем к еде
    fn choose_priority_point(
        // &self,
        &mut self,
        available_points: Vec<Point>,
        entities: &Entities,
    ) -> Option<Point> {
        if available_points.is_empty() {
            return None;
        }

        let mut empty_cells = Vec::with_capacity(4);

        //Если из доступных точек появляется еда, то сразу ее возвращаем,
        //иначе копим вектор доступных для хода пустырей
        for point in available_points {
            if let Some(entity) = entities.get(&point) {
                let entity_view = entity.view();

                if entity_view == GRASS_VIEW {
                    return Some(entity.get_position());
                } else if entity_view == WASTELAND_VIEW {
                    match self.track_contains(&point) {
                        Some(false) => empty_cells.push(entity.get_position()),
                        _ => continue,
                    }
                } else {
                    self.insert_point(point);
                }
            }
        }

        //Если не найдена еда и вектор пустырей заполнен хотя бы 1 элементом
        if !empty_cells.is_empty() {
            let mut rng = thread_rng();
            return empty_cells.choose(&mut rng).copied();
        }
        None
    }
}
