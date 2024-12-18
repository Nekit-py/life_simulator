use crate::entities::food::{GRASS_VIEW, MEAT_VIEW};
use crate::entities::other::{VIRUS_VIEW, WASTELAND_VIEW};
use crate::field::Point;
use crate::traits::{Action, Health, LookAround, Movable, Positionable, Satiety, Tracker};
use core::option::Option;
use rand::seq::SliceRandom;
use rand::thread_rng;
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

impl LookAround for Boar {
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

impl Tracker for Boar {
    fn reset_track(&mut self) {
        if self.0.track.len() == 3 {
            self.0.track.clear();
            self.0.track.insert(self.0.position);
        }
    }

    fn insert_point(&mut self, point: Point) {
        self.reset_track();
        self.0.track.insert(point);
    }

    fn track_contains(&self, point: &Point) -> Option<bool> {
        Some(self.0.track.contains(point))
    }
}

impl Movable for Boar {}

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
    fn calculate_move_effects(&mut self, arrival_point: Option<Point>, entities: &Entities) {
        //Смотрим какая сущность лежит в точке, которую мы пришли
        match arrival_point {
            Some(arrival_point) => {
                if let Some(arrival_entity) = entities.get(&arrival_point) {
                    let arrival_entity = arrival_entity.view();

                    if arrival_entity == GRASS_VIEW {
                        self.eat();
                    }

                    if arrival_entity == WASTELAND_VIEW {
                        self.starve();
                    }

                    if arrival_entity == VIRUS_VIEW {
                        self.take_damage(Some(3));
                    }

                    if self.is_hungry() {
                        self.take_damage(None);
                    }

                    if self.is_fed() {
                        self.heal();
                    }
                }
            }
            None => {
                if self.is_hungry() {
                    self.take_damage(None);
                }
                self.starve();
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

impl LookAround for Lion {
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

                if entity_view == MEAT_VIEW || entity_view == BOAR_VIEW {
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
impl Movable for Lion {}

impl Tracker for Lion {
    fn reset_track(&mut self) {
        if self.0.track.len() >= 3 {
            self.0.track.clear();
            self.0.track.insert(self.0.position);
        }
    }

    fn insert_point(&mut self, point: Point) {
        self.reset_track();
        self.0.track.insert(point);
    }

    fn track_contains(&self, point: &Point) -> Option<bool> {
        Some(self.0.track.contains(point))
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
    fn calculate_move_effects(&mut self, arrival_point: Option<Point>, entities: &Entities) {
        //Смотрим какая сущность лежит в точке, которую мы пришли
        match arrival_point {
            Some(arrival_point) => {
                if let Some(arrival_entity) = entities.get(&arrival_point) {
                    let arrival_entity = arrival_entity.view();

                    if arrival_entity == BOAR_VIEW || arrival_entity == MEAT_VIEW {
                        self.eat();
                    }

                    if arrival_entity == WASTELAND_VIEW {
                        self.starve();
                    }

                    if arrival_entity == VIRUS_VIEW {
                        self.take_damage(Some(3));
                    }

                    if self.is_hungry() {
                        self.take_damage(None);
                    }

                    if self.is_fed() {
                        self.heal();
                    }
                }
            }
            None => {
                if self.is_hungry() {
                    self.take_damage(None);
                }
                self.starve();
            }
        }
        println!("{:?}", self);
    }
}

impl fmt::Display for Lion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
