pub mod animals;
pub mod food;
pub mod other;
pub mod traits;

use crate::traits::Action;
use animals::{Boar, Lion};
use core::fmt::Display;
use food::{Grass, Meat};
use other::{Virus, Wasteland};
use rand::Rng;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::fmt;

#[derive(Clone, Copy, Default, Debug)]
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
    matrix: Vec<Vec<Box<dyn Action>>>,
}

impl Field {
    ///Создание поля заполенного "пустырями"
    pub fn new(height: usize, width: usize) -> Self {
        let mut matrix = vec![];
        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
                let p = Point::new(x, y);
                row.push(Box::new(Wasteland::new(p)) as Box<dyn Action>);
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
        for x in 0..self.width {
            for y in 0..self.height {
                let point = Point::new(x, y);
                match rng.gen_range(1..=100) {
                    1..=3 => self.matrix[x][y] = Box::new(Virus::new(point)),
                    11..=13 => self.matrix[x][y] = Box::new(Lion::new(point)),
                    31..=40 => self.matrix[x][y] = Box::new(Boar::new(point)),
                    71..=100 => self.matrix[x][y] = Box::new(Grass::new(point)),
                    _ => self.matrix[x][y] = Box::new(Wasteland::new(point)),
                }
            }
        }
    }

    // ///Назначить на точку сущность
    // pub fn assign_to_point(&mut self, entity: Box<&mut dyn Action>) {
    pub fn assign_to_point(&mut self, entity: &mut Box<(dyn Action + 'static)>) {
        let (cur_x, cur_y) = entity.get_position().coords();
        self.matrix[cur_x][cur_y] = Box::new(Wasteland::new(Point::new(cur_x, cur_y)));
        entity.action(self.height, self.width);
        let (to_x, to_y) = entity.get_position().coords();
        println!("{}", entity);
        // self.matrix[to_x][to_y] = Box::new(Box::into_inner(entity));
    }

    // //Антипаттерн полиморфизму? + Рассмотреть мапу
    pub fn start_new_life(&mut self) {
        for row in self.matrix.iter_mut() {
            for entity in row.iter_mut() {
                // println! {"{}", entity}
                self.assign_to_point(entity);
            }
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.matrix {
            let row_display: Vec<String> = row
                .iter()
                .map(|entity| format!("{}", entity))
                .collect::<Vec<_>>();
            writeln!(f, "{}", row_display.join(""))?;
        }
        Ok(())
    }
}
