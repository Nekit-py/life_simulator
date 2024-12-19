pub mod entities;
pub mod traits;

use crate::entities::{Entities, Entity};
use crate::traits::{Action, Health, Movable, Positionable};
use crossterm::{
    event::KeyCode,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use entities::animals::{Boar, Lion, BOAR_VIEW, LION_VIEW};
use entities::food::{Grass, Meat, GRASS_VIEW, MEAT_VIEW};
use entities::other::{Virus, Wasteland, VIRUS_VIEW, WASTELAND_VIEW};
use rand::thread_rng;
use rand::Rng;
use std::hash::Hash;
use std::{collections::HashMap, fmt};
use std::{thread, time};

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
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

//Есть точка достаем по ней объект, мутируем его, пишем по точке, обновляем мапу
#[derive(Debug)]
pub struct Field {
    height: usize, //y
    width: usize,  //x
    matrix: Vec<Vec<Entity>>,
}

impl Field {
    pub fn from_test_case(matrix: Vec<Vec<Entity>>) -> Self {
        let (height, width) = (3usize, 3usize);
        Self {
            height,
            width,
            matrix,
        }
    }

    ///Создание поля заполненными случайными сущностями
    pub fn new(height: usize, width: usize) -> Self {
        let mut rng = thread_rng();

        let matrix: Vec<Vec<Entity>> = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let point = Point::new(x, y);
                        match rng.gen_range(1..=100) {
                            1..=3 => Entity::Virus(Virus::new(point)),
                            10..=12 => Entity::Lion(Lion::new(point)),
                            18..=40 => Entity::Boar(Boar::new(point)),
                            41..=80 => Entity::Grass(Grass::new(point)),
                            85..=90 => Entity::Meat(Meat::new(point)),
                            _ => Entity::Wasteland(Wasteland::new(point)),
                        }
                    })
                    .collect()
            })
            .collect();

        Self {
            height,
            width,
            matrix,
        }
    }

    pub fn get_entities(&self) -> Entities {
        let mut entities = Entities::new(HashMap::new());
        for y in 0..self.height {
            for x in 0..self.width {
                entities.add(self.matrix[y][x].clone());
            }
        }
        entities
    }

    pub fn simulate(&mut self, entities: &mut Entities) -> Result<(), std::io::Error> {
        // let mut stdout = std::io::stdout();
        // terminal::enable_raw_mode()?;
        let delay = time::Duration::from_millis(50);

        {
            thread::sleep(delay);
            println!("{}", self);
        }
        for y in 0..self.height {
            // {
            //     thread::sleep(delay);
            //     println!("{}", self);
            // }
            for x in 0..self.width {
                //получаем текущую точку
                let point = Point::new(x, y);
                //Получаем сущность по координатам (точке)
                let mut entity = entities.pop(&point);
                let entity_view = entity.view();

                if entity_view == BOAR_VIEW || entity_view == LION_VIEW {
                    //Мутируем сущность совершая действие
                    entity.action(self.height, self.width, entities);
                    //Создаем пустое поле на месте текущей точки
                    let wasteland = Entity::Wasteland(Wasteland::new(point));
                    self.matrix[y][x] = wasteland.clone();
                    //Обновляем мапу заместив удаленную сущность на пустырь
                    entities.add(wasteland.clone());

                    match entity.is_alive() {
                        Some(true) => {
                            //Получаем координаты куда будет установлена обновленная сущность и ставим ее туда
                            let (to_x, to_y) = entity.get_position().coords();
                            self.matrix[to_y][to_x] = entity.clone();
                            //Обновляем мапу добавив по координатам обновленную сущность
                            entities.add(entity);
                        }
                        Some(false) => {
                            //Получаем координаты куда будет установлена "пустота" после смерти и ставим ее туда
                            let dead_entity_position = entity.get_position();
                            let (to_x, to_y) = dead_entity_position.coords();
                            self.matrix[to_y][to_x] = wasteland;
                            entities.animal_died();
                            //Обновляем мапу заместив мертвое животное на мясо
                            entities.add(Entity::Meat(Meat::new(dead_entity_position)));
                        }
                        None => {}
                    }
                } else {
                    entities.add(entity);
                }
            }
            // stdout.execute(terminal::Clear(ClearType::All))?;
        }
        Ok(())
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
