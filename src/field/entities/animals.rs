use crate::field::Point;
use crate::traits::{Action, Health, LookAround, Movable, Positionable, Satiety};
use std::collections::HashSet;
use std::fmt;

use super::Entities;

pub const BOAR_VIEW: char = '🐗';
pub const LION_VIEW: char = '🦁';

///Абстрактная структура животное
#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
struct Animal {
    hunger: u8,
    health: u8,
    view: char,
    track: HashSet<Point>,
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
        let mut track = HashSet::with_capacity(3);
        track.insert(position);

        Boar(Animal {
            view: BOAR_VIEW,
            track,
            position,
            health: 15,
            hunger: 5,
        })
    }
    pub fn view(&self) -> char {
        self.0.view
    }
}

impl LookAround for Boar {}

impl Movable for Boar {
    fn get_track(&mut self) -> Option<&mut HashSet<Point>> {
        Some(&mut self.0.track)
    }
}

impl Satiety for Boar {
    fn get_hunger(&self) -> u8 {
        self.0.hunger
    }

    fn set_hunger(&mut self, hunger: u8) {
        self.0.hunger = hunger
    }

    fn is_hungry(&self) -> bool {
        self.0.hunger == 0
    }

    fn is_fed(&self) -> bool {
        self.0.hunger == 10
    }
}

impl Health for Boar {
    fn get_health(&self) -> u8 {
        self.0.health
    }

    fn set_health(&mut self, health: u8) {
        self.0.health = health
    }

    fn is_alive(&self) -> Option<bool> {
        Some(self.0.health != 0)
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

impl Action for Boar {
    ///Рассчет последствий хода (голодаем получаем урон и т.п.)
    fn calculate_move_effects(&mut self, entities: &Entities) {
        let arrival_point = self.get_position();
        //Смотрим какая сущность лежит в точку, которую мы пришли
        if let Some(arrival_entity) = entities.get(&arrival_point) {
            let arrival_entity = arrival_entity.view();

            if arrival_entity == '🌱' {
                self.eat();
            }

            if arrival_entity == '⬛' {
                self.starve();
            }

            if arrival_entity == '🦠' {
                self.take_damage(Some(3));
            }

            if self.is_hungry() {
                self.take_damage(None);
            }

            if self.is_fed() {
                self.heal();
            }
        }
        println!("{:?}", self);
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

impl Lion {
    pub fn new(position: Point) -> Self {
        let mut track = HashSet::with_capacity(3);
        track.insert(position);
        Lion(Animal {
            view: LION_VIEW,
            track,
            position,
            health: 15,
            hunger: 7,
        })
    }

    pub fn view(&self) -> char {
        self.0.view
    }
}

impl LookAround for Lion {}
impl Movable for Lion {
    fn get_track(&mut self) -> Option<&mut HashSet<Point>> {
        Some(&mut self.0.track)
    }
}

impl Satiety for Lion {
    fn get_hunger(&self) -> u8 {
        self.0.hunger
    }

    fn set_hunger(&mut self, hunger: u8) {
        self.0.hunger = hunger
    }

    fn is_hungry(&self) -> bool {
        self.0.hunger == 0
    }

    fn is_fed(&self) -> bool {
        self.0.hunger == 10
    }
}

impl Health for Lion {
    fn get_health(&self) -> u8 {
        self.0.health
    }

    fn set_health(&mut self, health: u8) {
        self.0.health = health
    }

    fn is_alive(&self) -> Option<bool> {
        Some(self.0.health != 0)
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
    fn calculate_move_effects(&mut self, entities: &Entities) {
        let arrival_point = self.get_position();
        //Смотрим какая сущность лежит в точку, которую мы пришли
        if let Some(arrival_entity) = entities.get(&arrival_point) {
            let arrival_entity = arrival_entity.view();
            if arrival_entity == '🍖' || arrival_entity == '🐗' {
                self.eat();
            }

            if arrival_entity == '⬛' {
                self.starve();
            }

            if arrival_entity == '🦠' {
                self.take_damage(Some(3));
            }
        }
    }
}

impl fmt::Display for Lion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
