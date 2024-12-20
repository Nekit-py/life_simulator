pub mod animals;
pub mod food;
pub mod other;

use crate::{
    traits::{Action, Movable, Positionable},
    Point,
};
use animals::{Boar, Lion};
use food::{Grass, Meat};
use other::{Virus, Wasteland};

use core::option::Option;
use std::{collections::HashMap, fmt};

use super::traits::{Health, LookAround, Tracker};

#[derive(Debug, Clone)]
pub enum Entity {
    Boar(Boar),
    Lion(Lion),
    Meat(Meat),
    Grass(Grass),
    Wasteland(Wasteland),
    Virus(Virus),
}

impl Entity {
    pub fn view(&self) -> char {
        match self {
            Entity::Boar(boar) => boar.view(),
            Entity::Lion(lion) => lion.view(),
            Entity::Meat(meat) => meat.view(),
            Entity::Grass(grass) => grass.view(),
            Entity::Wasteland(wasteland) => wasteland.view(),
            Entity::Virus(virus) => virus.view(),
        }
    }
}

impl Tracker for Entity {
    fn reset_track(&mut self) {
        match self {
            Entity::Boar(boar) => boar.reset_track(),
            Entity::Lion(lion) => lion.reset_track(),
            _ => (),
        }
    }

    fn insert_point(&mut self, point: Point) {
        match self {
            Entity::Boar(boar) => boar.insert_point(point),
            Entity::Lion(lion) => lion.insert_point(point),
            _ => (),
        }
    }

    fn track_contains(&self, point: &Point) -> Option<bool> {
        match self {
            Entity::Boar(boar) => boar.track_contains(point),
            Entity::Lion(lion) => lion.track_contains(point),
            _ => None,
        }
    }
}

impl LookAround for Entity {
    fn calculate_move(
        &mut self,
        height: usize,
        width: usize,
        entities: &Entities,
    ) -> Option<Point> {
        match self {
            Entity::Boar(boar) => boar.calculate_move(height, width, entities),
            Entity::Lion(lion) => lion.calculate_move(height, width, entities),
            _ => None,
        }
    }
    fn choose_priority_point(
        &mut self,
        availabele_points: Vec<Point>,
        entities: &Entities,
    ) -> Option<Point> {
        match self {
            Entity::Boar(boar) => boar.choose_priority_point(availabele_points, entities),
            Entity::Lion(lion) => lion.choose_priority_point(availabele_points, entities),
            _ => None,
        }
    }
}

impl Action for Entity {
    fn action(&mut self, height: usize, width: usize, entities: &mut Entities) {
        match self {
            Entity::Boar(boar) => boar.action(height, width, entities),
            Entity::Lion(lion) => lion.action(height, width, entities),
            _ => {}
        }
    }
    fn calculate_move_effects(&mut self, arrival_point: Option<Point>, entities: &mut Entities) {
        match self {
            Entity::Boar(boar) => boar.calculate_move_effects(arrival_point, entities),
            Entity::Lion(lion) => lion.calculate_move_effects(arrival_point, entities),
            _ => {}
        }
    }
}

impl Movable for Entity {
    fn is_moved(&self) -> bool {
        match self {
            Entity::Boar(boar) => boar.is_moved(),
            Entity::Lion(lion) => lion.is_moved(),
            Entity::Meat(meat) => meat.is_moved(),
            Entity::Grass(grass) => grass.is_moved(),
            Entity::Wasteland(wasteland) => wasteland.is_moved(),
            Entity::Virus(virus) => virus.is_moved(),
        }
    }

    fn move_allowed(&mut self, allow: bool) {
        match self {
            Entity::Boar(boar) => boar.move_allowed(allow),
            Entity::Lion(lion) => lion.move_allowed(allow),
            Entity::Meat(meat) => meat.move_allowed(allow),
            Entity::Grass(grass) => grass.move_allowed(allow),
            Entity::Wasteland(wasteland) => wasteland.move_allowed(allow),
            Entity::Virus(virus) => virus.move_allowed(allow),
        }
    }
}

impl Health for Entity {
    fn is_alive(&self) -> Option<bool> {
        match self {
            Entity::Boar(boar) => boar.is_alive(),
            Entity::Lion(lion) => lion.is_alive(),
            _ => None,
        }
    }
    fn get_health(&self) -> u8 {
        match self {
            Entity::Boar(boar) => boar.get_health(),
            Entity::Lion(lion) => lion.get_health(),
            _ => 0u8,
        }
    }

    fn set_health(&mut self, health: u8) {
        match self {
            Entity::Boar(boar) => boar.set_health(health),
            Entity::Lion(lion) => lion.set_health(health),
            _ => (),
        }
    }
}

impl Positionable for Entity {
    fn get_position(&self) -> Point {
        match self {
            Entity::Boar(boar) => boar.get_position(),
            Entity::Lion(lion) => lion.get_position(),
            Entity::Meat(meat) => meat.get_position(),
            Entity::Grass(grass) => grass.get_position(),
            Entity::Wasteland(wasteland) => wasteland.get_position(),
            Entity::Virus(virus) => virus.get_position(),
        }
    }
    fn set_position(&mut self, point: Point) {
        match self {
            Entity::Boar(boar) => boar.set_position(point),
            Entity::Lion(lion) => lion.set_position(point),
            Entity::Meat(meat) => meat.set_position(point),
            Entity::Grass(grass) => grass.set_position(point),
            Entity::Wasteland(wasteland) => wasteland.set_position(point),
            Entity::Virus(virus) => virus.set_position(point),
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entity::Boar(boar) => write!(f, "{}", boar),
            Entity::Lion(lion) => write!(f, "{}", lion),
            Entity::Meat(meat) => write!(f, "{}", meat),
            Entity::Grass(grass) => write!(f, "{}", grass),
            Entity::Wasteland(wasteland) => write!(f, "{}", wasteland),
            Entity::Virus(virus) => write!(f, "{}", virus),
        }
    }
}

impl Default for Entity {
    fn default() -> Entity {
        Entity::Wasteland(Wasteland::new(Point::default()))
    }
}

#[derive(Debug)]
pub struct Entities {
    collection: HashMap<Point, Entity>,
    animals: u8,
}

impl Entities {
    pub fn new(collection: HashMap<Point, Entity>) -> Self {
        Self {
            collection,
            animals: 0,
        }
    }

    pub fn pop(&mut self, point: &Point) -> Entity {
        let entity = self.collection.remove(point).unwrap();
        match entity {
            Entity::Boar(_) | Entity::Lion(_) => self.animals = self.animals.saturating_sub(1),
            _ => (),
        }
        entity
    }

    //Уменьшает щетчик животных, когда животное умерло
    pub fn animal_died(&mut self) {
        self.animals = self.animals.saturating_sub(1)
    }

    pub fn add(&mut self, entity: Entity) {
        let point_key = entity.get_position();
        match entity {
            Entity::Boar(_) | Entity::Lion(_) => self.animals = self.animals.saturating_add(1),
            _ => (),
        }
        self.collection.insert(point_key, entity);
    }

    pub fn get(&self, point: &Point) -> Option<&Entity> {
        self.collection.get(point)
    }

    pub fn total_animals(&self) -> u8 {
        self.animals
    }
}
