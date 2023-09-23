use minifb::Key;

use super::types::Coords;

pub enum CollisionShape {
    Circle(f64),
}

pub trait GameObject {
    fn weight_factor(&self) -> f64;
    fn draw(&self) -> Vec<Vec<u32>>;
    fn collision_shape(&self) -> CollisionShape;

    fn get_coords(&self) -> &Coords;
    fn set_coords(&mut self, coords: Coords);

    fn handle_input(&mut self, _keys: &[Key]) {}
}
