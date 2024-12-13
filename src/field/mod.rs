pub mod animals;
pub mod food;
pub mod other;
pub mod traits;

use crate::traits::{Action, Positionable};
use animals::{Boar, Lion};
use food::{Grass, Meat};
use other::{Virus, Wasteland};
use rand::thread_rng;
use rand::Rng;
use std::hash::Hash;
use std::{collections::HashMap, fmt};
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

// impl Entity {
//     fn is_moved(&self) -> bool {
//         match self {
//             Entity::Boar(boar) => boar.is_moved(),
//             Entity::Lion(lion) => lion.is_moved(),
//             _ => false,
//         }
//     }
// }

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

pub struct Entities {
    collection: HashMap<Point, Entity>,
}

impl Entities {
    fn new(collection: HashMap<Point, Entity>) -> Self {
        Self { collection }
    }

    fn pop(&mut self, point: &Point) -> Entity {
        self.collection.remove(point).unwrap()
    }

    fn add(&mut self, entity: Entity) {
        let point_key = entity.get_position();
        self.collection.insert(point_key, entity);
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
        let mut matrix: Vec<Vec<Entity>> = vec![];

        //TODO: Перемсотреть записб по координатам
        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
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

    pub fn get_entities(&self) -> Entities {
        let mut collection = HashMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                collection.insert(Point::new(x, y), self.matrix[y][x].clone());
            }
        }
        Entities::new(collection)
    }

    //TODO проверить, как заменяется на пустую клетку, правильно ли меняются флаги хода, как удаляются и добовляются сущности в мапу
    pub fn simulate(&mut self, entities: &mut Entities) {
        for y in 0..self.height {
            for x in 0..self.width {
                //получаем текущую точку
                let point = Point::new(x, y);
                //Получаем сущность по координатам (точке)
                let mut entity = entities.pop(&point);
                //Мутируем сущность
                entity.action(self.height, self.width);
                //Создаем пустое поле на месте текущей точки
                let wasteland = Entity::Wasteland(Wasteland::new(point));
                self.matrix[y][x] = wasteland.clone();
                //Получаем координаты куда будет установлена обновленная сущность и ставим ее туда
                let (to_x, to_y) = entity.get_position().coords();
                self.matrix[to_y][to_x] = entity.clone();
                //Обновляем мапу заместив удаленную сущность на пустую землю
                entities.add(wasteland);
                //Обновляем мапу добавив по координатам обновленную сущность
                entities.add(entity);
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
