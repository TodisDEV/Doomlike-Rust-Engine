#[derive(Clone, Copy, Debug)]
pub struct Vector2{
    pub x: f32,
    pub y: f32
}

pub fn get_distance_from(p1: Vector2, p2: Vector2) -> f32{
    return ((p1.x - p2.x)*(p1.x - p2.x) + (p1.y - p2.y)*(p1.y - p2.y)).sqrt();
}