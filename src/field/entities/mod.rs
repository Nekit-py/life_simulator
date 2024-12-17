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

use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use super::traits::LookAround;

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

impl LookAround for Entity {
    fn calculate_move(&self, height: usize, width: usize, entities: &Entities) -> Option<Point> {
        match self {
            Entity::Boar(boar) => boar.calculate_move(height, width, entities),
            Entity::Lion(lion) => lion.calculate_move(height, width, entities),
            _ => None,
        }
    }
    fn choose_priority_point(
        &self,
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
    fn action(&mut self, height: usize, width: usize, entities: &Entities) {
        match self {
            Entity::Boar(boar) => boar.action(height, width, entities),
            Entity::Lion(lion) => lion.action(height, width, entities),
            _ => {}
        }
    }
    fn calculate_move_effects(&mut self, entities: &Entities) {
        match self {
            Entity::Boar(boar) => boar.calculate_move_effects(entities),
            Entity::Lion(lion) => lion.calculate_move_effects(entities),
            _ => {}
        }
    }
}

impl Movable for Entity {
    fn get_track(&mut self) -> Option<&mut HashSet<Point>> {
        match self {
            Entity::Boar(boar) => boar.get_track(),
            Entity::Lion(lion) => lion.get_track(),
            _ => None,
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

pub struct Entities {
    collection: HashMap<Point, Entity>,
}

impl Entities {
    pub fn new(collection: HashMap<Point, Entity>) -> Self {
        Self { collection }
    }

    pub fn pop(&mut self, point: &Point) -> Entity {
        self.collection.remove(point).unwrap()
    }

    pub fn add(&mut self, entity: Entity) {
        let point_key = entity.get_position();
        self.collection.insert(point_key, entity);
    }

    pub fn get(&self, point: &Point) -> &Entity {
        self.collection.get(&point).unwrap()
    }
}
