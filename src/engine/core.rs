use minifb::{Key, ScaleMode, Window, WindowOptions};

use super::{
    constants::*,
    game_object::GameObject,
    types::{Coords, WindowSize},
};

struct InternalGameObject {
    object: Box<dyn GameObject>,
    vx: f64,
    vy: f64,
}

pub struct Engine {
    window: Option<Window>,
    buffer: Vec<u32>,
    window_size: WindowSize,
    objects: Vec<InternalGameObject>,
}

// public functions
impl Engine {
    pub fn new(window_size: &WindowSize) -> Result<Self, anyhow::Error> {
        Ok(Self {
            buffer: vec![0; window_size.width * window_size.height],
            window: None,
            window_size: window_size.clone(),
            objects: Vec::new(),
        })
    }

    pub fn run(&mut self, window_title: &str) -> Result<(), anyhow::Error> {
        self.window = Some(Window::new(
            window_title,
            self.window_size.width,
            self.window_size.height,
            WindowOptions {
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )?);

        while self.window.as_ref().unwrap().is_open()
            && !self.window.as_ref().unwrap().is_key_down(Key::Escape)
        {
            let start_time = std::time::Instant::now();

            // clear the display buffer
            self.buffer.iter_mut().for_each(|p| *p = 0);

            for object in self.objects.iter_mut() {
                // re-calculate the velocities of the object
                Engine::calc_velocities(object);

                // apply the velocities of the object
                Engine::apply_velocities(object);

                // perform collision checks on objects
                Engine::collision_checks(object);

                // draw the object
                Engine::draw(&mut self.buffer, &self.window_size, object);
            }

            // reflect the display buffer changes
            self.window.as_mut().unwrap().update_with_buffer(
                &self.buffer,
                self.window_size.width,
                self.window_size.height,
            )?;

            // we've done everything we needed to this frame,
            // so we can sleep until the next frame is needed.
            std::thread::sleep(
                std::time::Duration::from_secs_f64(DT).saturating_sub(start_time.elapsed()),
            );
        }

        Ok(())
    }

    pub fn add_game_object(&mut self, game_object: impl GameObject + 'static) {
        self.objects.push(InternalGameObject {
            object: Box::new(game_object),
            vx: 0.0,
            vy: 0.0,
        })
    }
}

// internal functions
impl Engine {
    fn calc_velocities(internal_object: &mut InternalGameObject) {
        // TODO: air resistance
        // internal_object.vy += GRAVITY * DT;
    }

    fn apply_velocities(internal_object: &mut InternalGameObject) {
        let coords = internal_object.object.get_coords();
        let x = coords.x + internal_object.vx;
        let y = coords.y + internal_object.vy;
        internal_object.object.set_coords(Coords { x, y });
    }

    fn collision_checks(internal_object: &mut InternalGameObject) {}

    fn draw(buffer: &mut Vec<u32>, window_size: &WindowSize, internal_object: &InternalGameObject) {
        let object = &internal_object.object;
        let coords = object.get_coords();
        let raster_vecs = object.draw();

        Engine::draw_at(
            buffer,
            window_size.width,
            window_size.height,
            raster_vecs,
            coords,
        );
    }
}

// internal utils
impl Engine {
    fn draw_at(
        buffer: &mut Vec<u32>,
        buffer_width: usize,
        buffer_height: usize,
        raster_vecs: Vec<Vec<u32>>,
        coords: &Coords,
    ) {
        let object_height = raster_vecs.len();
        let object_width = raster_vecs.iter().map(|row| row.len()).max().unwrap_or(0); // Get max width

        for (dy, row) in raster_vecs.iter().enumerate() {
            for dx in 0..object_width {
                let x = (coords.x + dx as f64) as usize;
                let y = (coords.y + dy as f64) as usize;

                // make sure this is not out of the buffer
                if x < buffer_width && y < buffer_height {
                    let index = y * buffer_width + x;

                    let maybe_pixel = row.get(dx).cloned();
                    if let Some(pixel) = maybe_pixel {
                        buffer[index] = pixel;
                    }
                }
            }
        }
    }
}
