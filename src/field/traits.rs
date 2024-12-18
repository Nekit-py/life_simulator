use rand::seq::SliceRandom;

use crate::entities::food::GRASS_VIEW;
use crate::entities::other::WASTELAND_VIEW;
use crate::Point;
use core::option::Option;
use rand::thread_rng;
use std::collections::HashSet;

use super::entities::Entities;

pub trait Positionable {
    fn get_position(&self) -> Point;
    fn set_position(&mut self, point: Point);
}

// pub trait Action: Movable + Satiety + Health + std::fmt::Display {
pub trait Action: Movable + std::fmt::Display {
    fn action(&mut self, height: usize, width: usize, entities: &Entities) {
        self.move_to(height, width, entities);
        self.calculate_move_effects(entities);
    }

    fn calculate_move_effects(&mut self, entities: &Entities) {}
}

pub trait Satiety {
    fn get_hunger(&self) -> u8;

    // Метод для установки уровня голода
    fn set_hunger(&mut self, hunger: u8);

    // Метод для голодания
    fn starve(&mut self) {
        let new_hunger = self.get_hunger().saturating_sub(1);
        self.set_hunger(new_hunger.max(0));
    }

    // Метод для поедания
    fn eat(&mut self) {
        // let new_hunger = self.get_hunger() + 3;
        let new_hunger = self.get_hunger().saturating_add(3);
        self.set_hunger(new_hunger.min(10));
    }

    fn is_hungry(&self) -> bool;
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

pub trait Movable: LookAround {
    fn get_track(&mut self) -> Option<&mut HashSet<Point>>;

    ///Следование в выбранном направлении на 1 клетку
    ///Должен проверять, что есть в области видимостьи:
    /// - если еда, то к еде,
    /// - если лев, к нему не идем, если мы кабан и наоборот)),
    /// - в остальных случаях рандомное направление за исклюеним пройденного пути
    fn move_to(&mut self, height: usize, width: usize, entities: &Entities) {
        // Сохраняем текущую позицию, чтобы освободить `self` для дальнейшей работы
        let position = self.get_position();

        // Получаем изменяемую ссылку на трек и обновляем её
        {
            let track = self.get_track().unwrap();
            if track.len() == 3 {
                track.clear();
                track.insert(position);
            }
        } // Изменяемая ссылка на `track` завершается здесь

        // Перемещаемся по приоритетной точке
        if let Some(point_to_move) = self.calculate_move(height, width, entities) {
            // Добавляем новую позицию в трек
            let track = self.get_track().unwrap(); // Создаём новую изменяемую ссылку на трек

            if !track.contains(&point_to_move) {
                track.insert(point_to_move);
                self.set_position(point_to_move); // Обновляем позицию
            } else {
                //Сопрный else
                track.clear();
            }
        }
    }
}

///Проверка возможных направлений движения
pub trait LookAround: Positionable {
    fn calculate_move(&self, height: usize, width: usize, entities: &Entities) -> Option<Point> {
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
        self.choose_priority_point(available_points, entities)
    }

    /// В приоритете идем к еде
    fn choose_priority_point(
        &self,
        available_points: Vec<Point>,
        entities: &Entities,
    ) -> Option<Point> {
        if available_points.is_empty() {
            return None;
        }

        let mut empty_cells = Vec::with_capacity(4);

        for point in available_points {
            if let Some(entity) = entities.get(&point) {
                let entity_view = entity.view();

                if entity_view == GRASS_VIEW {
                    return Some(entity.get_position());
                } else if entity_view == WASTELAND_VIEW {
                    empty_cells.push(entity.get_position())
                }
            }
        }

        if !empty_cells.is_empty() {
            let mut rng = thread_rng();
            return empty_cells.choose(&mut rng).copied();
        }
        None
    }
}
