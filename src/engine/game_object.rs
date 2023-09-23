use super::types::Coords;

pub trait GameObject {
    fn weight(&self) -> i32;
    fn bounciness(&self) -> i32;
    fn draw(&self) -> Vec<Vec<u32>>;

    fn get_coords(&self) -> &Coords;
    fn set_coords(&mut self, coords: Coords);
}
