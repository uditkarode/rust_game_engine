use minifb::Key;

use super::types::{ObjectInfo, XYPair};

pub enum CollisionShape {
    Circle(f64),
}

pub trait GameObject {
    fn weight_factor(&self) -> f64;
    fn draw(&self) -> Vec<Vec<u32>>;
    fn collision_shape(&self) -> CollisionShape;

    fn get_coords(&self) -> &XYPair;
    fn set_coords(&mut self, coords: &XYPair);

    fn get_velocities(&self) -> &XYPair;
    fn set_velocities(&mut self, velocities: &XYPair);

    fn get_object_info(&self) -> Option<&ObjectInfo>;
    fn set_object_info(&mut self, object_info: &ObjectInfo);

    fn handle_input(&mut self, _keys: &[Key]) {}
}
