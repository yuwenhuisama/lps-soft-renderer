use crate::lps::rasterize::render_target::RenderTarget;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};
use sdl2::{EventPump, Sdl};
use std::time::Duration;

pub struct RenderWindow {
    title: String,
    width: u32,
    height: u32,
    canvas: Option<WindowCanvas>,
    sdl_context: Option<Sdl>,
    event_pump: Option<EventPump>,
}

impl RenderWindow {
    pub fn create(title: String, width: u32, height: u32) -> RenderWindow {
        RenderWindow {
            title,
            width,
            height,
            canvas: None,
            sdl_context: None,
            event_pump: None,
        }
    }

    pub fn init(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");

        self.sdl_context = Some(sdl_context);
        self.canvas = Some(canvas);
        self.event_pump = Some(event_pump);
    }

    pub fn update(&mut self, render_target: &RenderTarget) -> bool {
        let mut canvas = self.canvas.as_mut().unwrap();
        let mut event_pump = self.event_pump.as_mut().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return true;
                }
                _ => {}
            }
        }

        for i in 0..render_target.width() {
            for j in 0..render_target.height() {
                let color = render_target.get_pixel(i, j);
                canvas.set_draw_color(Color::RGB(color.r, color.g, color.b));
                canvas.draw_point((i as i32, j as i32)).unwrap();
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        return false;
    }
}
