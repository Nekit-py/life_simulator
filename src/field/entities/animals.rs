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
const TRACK_LIMIT: usize = 3;
const MAX_AVAILABLE_POINTS: usize = 4;

///Абстрактная структура животное
#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
struct Animal {
    hunger: u8,
    health: u8,
    view: char,
    track: HashSet<Point>,
    moved: bool,
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
        let mut track = HashSet::with_capacity(TRACK_LIMIT);
        track.insert(position);

        Boar(Animal {
            view: BOAR_VIEW,
            track,
            moved: false,
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
        &mut self,
        available_points: Vec<Point>,
        entities: &Entities,
    ) -> Option<Point> {
        if available_points.is_empty() {
            return None;
        }

        let mut empty_cells = Vec::with_capacity(MAX_AVAILABLE_POINTS);

        //Если из доступных точек появляется еда, то сразу ее возвращаем,
        //иначе копим вектор доступных для хода пустырей
        for point in available_points {
            if let Some(entity) = entities.get(&point) {
                let entity_view = entity.view();
                let entity_position = entity.get_position();

                match entity_view {
                    GRASS_VIEW => return Some(entity_position),
                    WASTELAND_VIEW | VIRUS_VIEW => match self.track_contains(&point) {
                        Some(false) => empty_cells.push(entity_position),
                        _ => continue,
                    },
                    _ => {
                        self.insert_point(point);
                    }
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
        if self.0.track.len() >= TRACK_LIMIT {
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

impl Movable for Boar {
    fn is_moved(&self) -> bool {
        self.0.moved
    }

    fn move_allowed(&mut self, allow: bool) {
        self.0.moved = allow
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
    fn calculate_move_effects(&mut self, arrival_point: Option<Point>, entities: &mut Entities) {
        if let Some(point) = arrival_point {
            if let Some(arrival_entity) = entities.get(&point) {
                match arrival_entity.view() {
                    GRASS_VIEW => self.eat(),
                    WASTELAND_VIEW => self.starve(),
                    VIRUS_VIEW => {
                        self.starve();
                        self.take_damage(Some(3));
                    }
                    _ => {}
                }
            }
        } else {
            self.starve();
        }
        self.life_cycle(entities);
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
        let mut track = HashSet::with_capacity(TRACK_LIMIT);
        track.insert(position);
        Lion(Animal {
            view: LION_VIEW,
            track,
            moved: false,
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
        &mut self,
        available_points: Vec<Point>,
        entities: &Entities,
    ) -> Option<Point> {
        if available_points.is_empty() {
            return None;
        }

        let mut empty_cells = Vec::with_capacity(MAX_AVAILABLE_POINTS);

        //Если из доступных точек появляется еда, то сразу ее возвращаем,
        //иначе копим вектор доступных для хода пустырей
        for point in available_points {
            if let Some(entity) = entities.get(&point) {
                let entity_view = entity.view();
                let entity_position = entity.get_position();

                match entity_view {
                    MEAT_VIEW | BOAR_VIEW => return Some(entity_position),
                    WASTELAND_VIEW | VIRUS_VIEW => match self.track_contains(&point) {
                        Some(false) => empty_cells.push(entity_position),
                        _ => continue,
                    },
                    _ => {
                        self.insert_point(point);
                    }
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
impl Movable for Lion {
    fn is_moved(&self) -> bool {
        self.0.moved
    }

    fn move_allowed(&mut self, allow: bool) {
        self.0.moved = allow
    }
}

impl Tracker for Lion {
    fn reset_track(&mut self) {
        if self.0.track.len() >= TRACK_LIMIT {
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
    // fn calculate_move_effects(&mut self, arrival_point: Option<Point>, entities: &Entities) {
    fn calculate_move_effects(&mut self, arrival_point: Option<Point>, entities: &mut Entities) {
        if let Some(point) = arrival_point {
            if let Some(arrival_entity) = entities.get(&point) {
                match arrival_entity.view() {
                    MEAT_VIEW => self.eat(),
                    BOAR_VIEW => {
                        self.eat();
                        entities.animal_died();
                    }
                    WASTELAND_VIEW => self.starve(),
                    VIRUS_VIEW => {
                        self.starve();
                        self.take_damage(Some(3));
                    }
                    _ => {}
                }
            }
        } else {
            self.starve();
        }
        self.life_cycle(entities);
    }
}

impl fmt::Display for Lion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
