pub mod animals;
pub mod food;
pub mod other;
pub mod traits;

use crate::traits::{LookAround, Positionable};
use animals::{Boar, Lion};
use food::{Grass, Meat};
use other::{Virus, Wasteland};
use rand::Rng;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::fmt;
use traits::Movable;

#[derive(Debug, Clone)]
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

#[derive(Clone, Debug)]
pub struct Field {
    height: usize, //y
    width: usize,  //x
    matrix: Vec<Vec<Entity>>,
}

impl Field {
    ///Создание поля заполенного "пустырями"
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

    ///Заполнене поля случайными объектами
    pub fn fill(&mut self, rng: &mut ThreadRng) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x, y);
                match rng.gen_range(1..=100) {
                    1..=3 => self.matrix[y][x] = Entity::Virus(Virus::new(point)),
                    11..=13 => self.matrix[y][x] = Entity::Lion(Lion::new(point)),
                    // 31..=40 => self.matrix[y][x] = Entity::Boar(Boar::new(point)),
                    31..=50 => self.matrix[y][x] = Entity::Boar(Boar::new(point)),
                    71..=100 => self.matrix[y][x] = Entity::Grass(Grass::new(point)),
                    _ => self.matrix[y][x] = Entity::Wasteland(Wasteland::new(point)),
                }
            }
        }
    }

    ///Получение размеров поля
    pub fn size(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    // pub fn get_by_point(&self, point: Point) -> &Entity {
    pub fn get_by_coords(&self, x: usize, y: usize) -> &Entity {
        // let (x, y) = point.coords();
        &self.matrix[y][x]
    }

    // pub fn start_new_life(&mut self) {
    //     for row in self.matrix.iter_mut() {
    //         for entity in row.iter_mut() {
    //             match entity {
    //                 Entity::Boar(ref mut boar) => {
    //                     let available_directions = { boar.look_around((self.height, self.width)) };
    //                     println!("{:?}", available_directions);
    //                     let move_to = boar.go_to_direction(available_directions);
    //                     if let Some(point_to_move) = move_to {
    //                         println!("cur_pos = {:?}", boar.get_position());
    //                         if !boar.is_moved() {
    //                             let new_boar = Boar::new(point_to_move);
    //                             println!("after_move = {:?}", new_boar.get_position());
    //                             // *entity = Entity::Boar(new_boar);
    //                             *boar = new_boar;
    //                             println!("{:?}", entity);
    //                         } else {
    //                             boar.mark_as_immovable();
    //                         }
    //                     }
    //                 }
    //                 _ => (),
    //             }
    //         }
    //     }
    // }

    pub fn start_new_life(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.matrix[y][x] {
                    Entity::Boar(ref mut boar) => {
                        let available_directions = { boar.look_around((self.height, self.width)) };
                        println!("{:?}", available_directions);
                        let move_to = boar.go_to_direction(available_directions);
                        if let Some(point_to_move) = move_to {
                            println!("cur_pos = {:?}", boar.get_position());
                            if !boar.is_moved() {
                                let new_boar = Boar::new(point_to_move);
                                println!("after_move = {:?}", new_boar.get_position());
                                let (to_x, to_y) = point_to_move.coords();
                                self.matrix[to_y][to_x] = Entity::Boar(new_boar);
                                self.matrix[y][x] =
                                    Entity::Wasteland(Wasteland::new(Point::new(x, y)));
                                println!("{:?}", self.matrix[y][x]);
                            } else {
                                boar.mark_as_immovable();
                            }
                        }
                    }
                    _ => (),
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
