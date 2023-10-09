use assets::Assets;
use engine::{worldgen, Engine};
use error_iter::ErrorIter as _;
use input_handler::{handle_input, Action};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};

use screen::Screen;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub mod assets;
pub mod colors;
pub mod input_handler;
pub mod screen;

const SCALE: usize = 2;
const WIDTH: usize = 640 * SCALE;
const HEIGHT: usize = 480 * SCALE;

type Image = (Vec<[u8; 4]>, (usize, usize));

pub struct Game {
    pub engine: Engine,
    pub screen: Screen,
    pub assets: Assets,
    pub tick: i32,
    pub game_log: Vec<String>
}

impl Game {
    fn new() -> Self {
        Self {
            engine: Engine::new((WIDTH, HEIGHT)),
            screen: Screen::new((WIDTH, HEIGHT)),
            assets: Assets::new(),
            tick: 0,
            game_log: Vec::new(),
        }
    }

    /// Update the `World` internal state
    fn update(&mut self) {
        self.tick += 1;
        if self.tick % 100 == 0 {
            self.game_log.push(format!("Test {}", self.tick / 100));
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
    let mut game = Game::new();
    worldgen::basic_fill(&mut game.engine.map);
    game.screen.setup_consoles();
    game.game_log.push("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string());

    // Generate a texture
    // world.image = worldgen::world::basic();

    // main event loop
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            game.draw(pixels.frame_mut());
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

            match handle_input(&input, &mut game) {
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
            game.update();
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
