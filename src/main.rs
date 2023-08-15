use cp437::Assets;
use error_iter::ErrorIter as _;
use log::error;
use map::Map;
use pixels::{Error, Pixels, SurfaceTexture};
use rltk::Point;
use screen::Screen;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use worldgen::basic_fill;

pub mod map;
pub mod screen;
pub mod worldgen;
pub mod cp437;

// const WIDTH: i32 = 1280;
// const HEIGHT: i32 = 960;
const WIDTH: i32 = 320;
const HEIGHT: i32 = 320;

struct World {
    pub map: Map,
    pub screen: Screen,
    pub assets: Assets,
}

impl World {
    fn new() -> Self {
        Self {
            map: Map::new(map::TileType::Water, (WIDTH, HEIGHT)),
            screen: Screen::new((WIDTH, HEIGHT), (0, 0)),
            assets: Assets::new(),
        }
    }

    /// Update the `World` internal state
    fn update(&mut self) {
        // if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
        //     self.velocity_x *= -1;
        // }
        // if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
        //     self.velocity_y *= -1;
        // }

        // self.box_x += self.velocity_x;
        // self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        self.screen.draw(frame, &self.map);

        self.screen.draw_box(&self.assets, frame, Point { x: WIDTH * 1/3 - 8, y: HEIGHT/2 - 4 - 8 }, Point { x: 12 * 8, y: 2 * 8 });
        self.screen.print_string(&self.assets, frame, "Hello World", Point { x: WIDTH * 1/3, y: HEIGHT/2 - 4 });

        // let sprite = &self.assets.cp437[(self.count / 30 % 255) as usize];
        // blit(frame, &Point{ x:0, y:0 }, sprite);

        // for x in 0..16 {
        //     for y in 0..16 {
        //         let idx = x+y * 16;
        //         let sprite = &self.assets.cp437[idx];
        //         blit(frame, &Point{ x:(x*8) as i32, y:(y*8) as i32 }, sprite);

        //     }
        // }

        // let str = "Hello world!";
        // let chars = string_to_cp437(str);

        // for (idx, ch) in chars.iter().enumerate() {
        //     let sprite = &self.assets.cp437[*ch as usize];
        //     blit(frame, &Point{ x:(WIDTH as usize*1/3 + idx * 8) as i32, y:(HEIGHT/2) as i32 }, sprite);
        // }
        
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
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

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    // Generate a world map
    let mut world = World::new();
    // basic_fill(&mut world.map);

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
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
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