use super::entities::Entities;
use crate::Point;
use core::option::Option::{self, Some};

pub trait Positionable {
    fn get_position(&self) -> Point;
    fn set_position(&mut self, point: Point);
}

pub trait Action: Movable + std::fmt::Display {
    fn action(&mut self, height: usize, width: usize, entities: &Entities) {
        if !self.is_moved() {
            let arrival_point = self.move_to(height, width, entities);
            self.calculate_move_effects(arrival_point, entities);
            self.move_allowed(false);
        } else {
            self.move_allowed(true);
        }
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

    ///Проверка соверашился ли ход
    fn is_moved(&self) -> bool;

    ///Зарзрешает или запрещает делать ход
    fn move_allowed(&mut self, allow: bool);
}

///Проверка возможных направлений движения
pub trait LookAround: Positionable + Tracker {
    fn calculate_move(
        &mut self,
        height: usize,
        width: usize,
        entities: &Entities,
    ) -> Option<Point> {
        let cur_pos = self.get_position();
        let (cur_x, cur_y) = cur_pos.coords();

        // Собираем доступные точки, используя итераторы
        let available_points: Vec<Point> = [
            (cur_x, cur_y.saturating_sub(1)), // вверх
            (cur_x, cur_y + 1),               // вниз
            (cur_x.saturating_sub(1), cur_y), // влево
            (cur_x + 1, cur_y),               // вправо
        ]
        .iter()
        .filter(|&&(x, y)| x < width && y < height) // Проверяем границы
        .map(|&(x, y)| Point::new(x, y)) // Преобразуем к Point
        .collect();

        // Выбираем приоритетную точку
        self.choose_priority_point(available_points, entities)
    }
    /// В приоритете идем к еде
    /// TODO: Постараться убрать дубль реализации
    fn choose_priority_point(
        &mut self,
        available_points: Vec<Point>,
        entities: &Entities,
    ) -> Option<Point>;
}
