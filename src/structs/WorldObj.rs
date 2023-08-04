use crate::structs::vector2::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct Wall{
    pub p1: Vector2,
    pub p2: Vector2,
}

#[derive(Clone, Debug)]
pub struct World {
    pub walls: Vec<Wall>
}
