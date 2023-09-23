#[derive(Clone)]
pub struct WindowSize {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone)]
pub struct XYPair {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct ObjectInfo {
    // in case we want to support window resizing in the
    // future, the objects might want to know the size.
    pub window_size: WindowSize,
}
