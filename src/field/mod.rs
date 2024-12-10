pub mod animals;
pub mod food;
pub mod other;

use animals::{Boar, Lion};
use food::{Grass, Meat};
use other::{Virus, Wasteland};
use rand::Rng;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::fmt;

// trait Movable {
//     fn move_to(matrix: &mut Vec<Vec<char>>, point: Point) -> Point;
// }

pub enum Entity {
    Boar(Boar),
    Lion(Lion),
    Meat(Meat),
    Grass(Grass),
    Wasteland(Wasteland),
    Virus(Virus),
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

#[derive(Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
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
}

pub struct Field {
    height: usize,
    width: usize,
    matrix: Vec<Vec<Entity>>,
}

impl Field {
    pub fn new(height: usize, width: usize) -> Self {
        let mut matrix = vec![];
        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
                let p = Point::new(x, y);
                row.push(Entity::Wasteland(Wasteland::new(p)));
            }
            matrix.push(row);
        }
        Self {
            height,
            width,
            matrix,
        }
    }

    pub fn fill(&mut self, rng: &mut ThreadRng) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x, y);
                match rng.gen_range(1..=100) {
                    1..=3 => self.matrix[y][x] = Entity::Virus(Virus::new(point)),
                    11..=13 => self.matrix[y][x] = Entity::Lion(Lion::new(point)),
                    31..=40 => self.matrix[y][x] = Entity::Boar(Boar::new(point)),
                    71..=100 => self.matrix[y][x] = Entity::Grass(Grass::new(point)),
                    _ => self.matrix[y][x] = Entity::Wasteland(Wasteland::new(point)),
                }
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
