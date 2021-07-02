use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use pixels::{Pixels, SurfaceTexture};

use image::imageops;
use image::ImageFormat;
use image::RgbaImage;
use imageproc::drawing::draw_text_mut;

use rusttype::{Font, Scale};

use crate::api::APIHandle;
use crate::input_handler::InputHandler;

const FONT_DATA: &[u8] = include_bytes!("./Calibri.ttf");

const PADLOCK_IMG: &[u8] = include_bytes!("./padlock.png");

pub struct Overlay {
    event_loop: EventLoop<()>,
    window: Window,
    pixels: Pixels,
    text: String,
    font: Font<'static>,
    input_handler: InputHandler,
}

impl Overlay {
    //Creates a new overlay with a window and event loop
    pub fn new(size: (u32, u32)) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_transparent(true)
            .with_decorations(false)
            .with_always_on_top(true)
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(size.0, size.1))
            .build(&event_loop)
            .unwrap();

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()
        };

        let font = Font::try_from_bytes(FONT_DATA).unwrap();

        Overlay {
            event_loop,
            window,
            pixels,
            text: String::new(),
            font,
            input_handler: InputHandler::new(),
        }
    }

    //Runs the main loop for the overlay
    pub fn run(self, api_handle: APIHandle) {
        let window = self.window;
        let mut pixels = self.pixels;
        let mut text = self.text;
        let font = self.font;
        let mut input_handler = self.input_handler;

        let lock = image::load_from_memory_with_format(PADLOCK_IMG, ImageFormat::Png).unwrap();

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                //Exits the program when required
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                //Updates overlay state
                Event::MainEventsCleared => {
                    api_handle.update_focus(&window);

                    //Set overlay position
                    if api_handle.is_game_focused() {
                        let win_rect = api_handle.get_win_rect();
                        
                        let win_size = window.inner_size();

                        let top = win_rect.bottom - win_size.height as i32;
                        window.set_outer_position(PhysicalPosition::new(win_rect.left, top));
                    } else {
                        window.set_outer_position(PhysicalPosition::new(-32000, -32000));
                    }

                    if api_handle.is_game_focused() {
                        input_handler.update(&mut text, &api_handle);
                    }
                }

                Event::RedrawRequested(_) => {
                    //Draws the text to the canvas
                    let win_size = window.inner_size();
                    let mut canvas = RgbaImage::new(win_size.width, win_size.height);

                    if api_handle.is_game_focused() {
                        let lines: Vec<&str> = text.split('\n').collect();
                        for (i, line) in lines.iter().enumerate() {
                            let y_pos = i * 50;
                            draw_text_mut(
                                &mut canvas,
                                [255, 255, 255, 255].into(),
                                30,
                                y_pos as u32,
                                Scale::uniform(40.0),
                                &font,
                                line,
                            );
                        }

                        for (i, parameter) in input_handler.parameters.iter().enumerate() {
                            let y_pos = i * 50;
                            if parameter.locked.is_some() {
                                imageops::overlay(&mut canvas, &lock, 5, y_pos as u32 + 2);
                            }
                        }
                    }

                    //Copies the canvas to the window buffer
                    let frame = pixels.get_frame();
                    frame.copy_from_slice(canvas.as_raw());

                    pixels.render().unwrap();
                }

                _ => (),
            }
        });
    }
}
