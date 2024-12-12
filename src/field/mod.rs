pub mod animals;
pub mod food;
pub mod other;
pub mod traits;

use crate::traits::{Action, Positionable};
use animals::{Boar, Lion};
use core::default;
use core::fmt::Display;
use food::{Grass, Meat};
use other::{Virus, Wasteland};
use rand::thread_rng;
use rand::Rng;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Entity {
    Boar(Boar),
    Lion(Lion),
    Meat(Meat),
    Grass(Grass),
    Wasteland(Wasteland),
    Virus(Virus),
}

impl Action for Entity {
    fn action(&mut self, height: usize, width: usize) {
        match self {
            Entity::Boar(boar) => boar.action(height, width),
            Entity::Lion(lion) => lion.action(height, width),
            Entity::Meat(meat) => meat.action(height, width),
            Entity::Grass(grass) => grass.action(height, width),
            Entity::Wasteland(wasteland) => wasteland.action(height, width),
            Entity::Virus(virus) => virus.action(height, width),
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)?;
        Ok(())
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub struct Field {
    height: usize, //y
    width: usize,  //x
    matrix: Vec<Vec<Entity>>,
}

impl Field {
    ///Создание поля заполенного "пустырями"
    pub fn new(height: usize, width: usize) -> Self {
        let mut rng = thread_rng();
        let mut matrix: Vec<Vec<Entity>> = vec![];

        for x in 0..width {
            let mut row = vec![];
            for y in 0..height {
                let point = Point::new(x, y);
                match rng.gen_range(1..=100) {
                    1..=3 => row.push(Entity::Virus(Virus::new(point))),
                    10..=14 => row.push(Entity::Lion(Lion::new(point))),
                    18..=35 => row.push(Entity::Boar(Boar::new(point))),
                    41..=80 => row.push(Entity::Grass(Grass::new(point))),
                    _ => row.push(Entity::Wasteland(Wasteland::new(point))),
                }
            }
            matrix.push(row);
        }

        Self {
            height,
            width,
            matrix,
        }
    }

    ///Назначить на точку сущность
    pub fn assign_to_point(&mut self, entity: &mut Entity) {
        let (cur_x, cur_y) = entity.get_position().coords();
        self.matrix[cur_x][cur_y] = Entity::Wasteland(Wasteland::new(Point::new(cur_x, cur_y)));
        entity.action(self.height, self.width);
        let (to_x, to_y) = entity.get_position().coords();
        self.matrix[to_x][to_y] = std::mem::take(entity);
    }

    ///Антипаттерн полиморфизму? + Рассмотреть мапу
    pub fn start_new_life(&mut self) {
        for row in self.matrix.iter_mut() {
            for entity in row.iter_mut() {
                self.assign_to_point(entity);
            }
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.matrix {
            let row_display: Vec<String> = row.iter().map(|entity| format!("{}", entity)).collect();
            writeln!(f, "{}", row_display.join(""))?;
        }
        Ok(())
    }
}
