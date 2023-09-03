use assets::Assets;
use error_iter::ErrorIter as _;
use input_handler::{handle_input, Action};
use log::error;
use map::Map;
use pixels::{Error, Pixels, SurfaceTexture};

use screen::Screen;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use worldgen::basic_fill;

pub mod assets;
pub mod colors;
pub mod input_handler;
pub mod map;
pub mod screen;
pub mod worldgen;

const SCALE: usize = 2;
const WIDTH: usize = 640 * SCALE;
const HEIGHT: usize = 480 * SCALE;
// const WIDTH: usize = 320;
// const HEIGHT: usize = 320;

type Image = (Vec<[u8; 4]>, (usize, usize));


#[derive(Debug, Clone, Copy)]
pub struct Point {
    /// The point's X location
    pub x: usize,
    /// The point's Y location
    pub y: usize,
}

pub struct World {
    pub map: Map,
    pub screen: Screen,
    pub assets: Assets,
    pub tick: i32,
    pub image: Image,
}

impl World {
    fn new() -> Self {
        Self {
            map: Map::new(map::TileType::Water, (WIDTH, HEIGHT)),
            screen: Screen::new((WIDTH, HEIGHT), (0, 0)),
            assets: Assets::new(),
            tick: 0,
            image: (Vec::new(), (0, 0)),
        }
    }

    /// Update the `World` internal state
    fn update(&mut self) {
        self.tick += 1;
        if self.tick % 100 == 0 {
            // let mut rng = RandomNumberGenerator::new();

            // let x = rng.roll_dice(1, self.map.size.0);
            // let y = rng.roll_dice(1, self.map.size.1);

            // self.screen.pos = (x, y);
        }
    }

    /// Draw the `World` state to the frame buffer.
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        // clear screen
        for (_, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = [0x00, 0x00, 0x00, 0x00];
            pixel.copy_from_slice(&rgba);
        }

        self.screen.draw(frame, &self);
        // self.screen.draw_image(&self.image, frame, Point{ x: 0, y: 0 })
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();

    // create the window
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // init pixels frame buffer with window
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    // Generate a world map
    let mut world = World::new();
    basic_fill(&mut world.map);
    world.screen.setup_consoles();

    // Generate a texture
    // world.image = worldgen::world::basic();

    // main event loop
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            match handle_input(&input, &mut world) {
                Action::None => {}
                Action::Exit => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
