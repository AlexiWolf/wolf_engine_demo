use log::*;
use wolf_engine::*;
use wolf_engine_sdl2::*;
use colors_transform::{Color, Hsl};
use sdl2::{pixels::Color as SdlColor, gfx::primitives::DrawRenderer};

fn main() {
    logging::initialize_logging(LevelFilter::Debug);    
    EngineBuilder::new()
        .with_plugin(Box::from(SdlPlugin::new(SdlWindowSettings::default())))
        .build()
        .run(Box::from(MainState::new()));
}

pub struct MainState {
    hew: f32, 
}

impl MainState {
    pub fn new() -> Self {
        Self {
            hew: 0.0,
        }
    }
}

impl State for MainState {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        self.hew = (self.hew + 1.0) % 360.0;
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        if let Some(Ok(mut sdl_graphics)) = context.try_borrow_mut::<SdlVideoContext>() {
            sdl_graphics.canvas.set_draw_color(convert_hew_to_color(self.hew));
            sdl_graphics.canvas.clear();
            sdl_graphics.canvas.string(10, 10, "Hello, World!", SdlColor::WHITE).unwrap();
            sdl_graphics.canvas.present();
        }
    }
}

pub fn convert_hew_to_color(hew: f32) -> SdlColor {
    SdlColor::BLUE
}
