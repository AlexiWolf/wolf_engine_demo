use std::time::{Instant, Duration};

use wolf_engine::*;
use wolf_engine_sdl2::*;
use log::*;
use sdl2::{pixels::Color, gfx::primitives::DrawRenderer};

fn main() {
    logging::initialize_logging(LevelFilter::Debug);    
    EngineBuilder::new()
        .with_plugin(Box::from(SdlPlugin::new(SdlWindowSettings::default())))
        .build()
        .run(Box::from(MainState::new()));
}

pub struct MainState {
    last_color_change: Instant,
    color: Color,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            last_color_change: Instant::now(),
            color: Color::RED,
        }
    }
}

impl State for MainState {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        if self.last_color_change.elapsed() > Duration::from_secs(1) {
            if self.color == Color::RED {
                self.color = Color::BLUE; 
            } else {
                self.color = Color::RED;
            }
            self.last_color_change = Instant::now();
        }
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        if let Some(Ok(mut sdl_graphics)) = context.try_borrow_mut::<SdlVideoContext>() {
            sdl_graphics.canvas.set_draw_color(self.color);
            sdl_graphics.canvas.clear();
            sdl_graphics.canvas.string(10, 10, "Hello, World!", Color::WHITE).unwrap();
            sdl_graphics.canvas.present();
        }
    }
}
