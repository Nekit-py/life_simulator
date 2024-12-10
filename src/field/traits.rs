use crate::{Field, Point};

pub trait Positionable {
    fn get_position(&self) -> Point;
}

pub trait Movable {
    fn right(&mut self, field: &mut Field) {}
    fn left(&mut self, field: &mut Field) {}
    fn up(&mut self, field: &mut Field) {}
    fn down(&mut self, field: &mut Field) {}
}
