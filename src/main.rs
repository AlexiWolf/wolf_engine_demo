use colors_transform::{Color, Hsl};
use hecs::World;
use hecs_schedule::{SubWorld, Schedule};
use log::*;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color as SdlColor};
use wolf_engine::*;
use wolf_engine_sdl2::*;

fn main() {
    logging::initialize_logging(LevelFilter::Debug);
    EngineBuilder::new()
        .with_plugin(Box::from(SdlPlugin::new(SdlWindowSettings::default())))
        .with_subcontext(HecsContext::new())
        .build()
        .run(Box::from(MainState::new()));
}

pub struct MainState {
    hew: f32,
    schedule: Schedule,
}

impl MainState {
    pub fn new() -> Self {
        Self { 
            hew: 0.0,
            schedule: Schedule::builder()
                .add_system(movement_system)
                .build()
        }
    }
}

impl State for MainState {
    fn update(&mut self, context: &mut Context) -> OptionalTransition {
        self.hew = (self.hew + 1.0) % 360.0;
        let mut hecs = context.try_borrow_mut::<HecsContext>()
            .expect("No HecsContext")
            .expect("Failed to borrow HecsContext");
        self.schedule.execute((&mut hecs.world,)).unwrap();
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        if let Some(Ok(mut sdl_graphics)) = context.try_borrow_mut::<SdlVideoContext>() {
            sdl_graphics
                .canvas
                .set_draw_color(convert_hew_to_color(self.hew));
            sdl_graphics.canvas.clear();
            sdl_graphics
                .canvas
                .string(10, 10, "Hello, World!", SdlColor::WHITE)
                .unwrap();
            sdl_graphics.canvas.present();
        }
    }
}

pub fn convert_hew_to_color(hew: f32) -> SdlColor {
    let color = Hsl::from(hew, 100.0, 50.0);
    SdlColor::RGB(
        color.get_red() as u8,
        color.get_green() as u8,
        color.get_blue() as u8,
    )
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
}

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
}

pub fn movement_system(world: SubWorld<(&mut Position, &Velocity)>) {
    world.query::<(&mut Position, &Velocity)>().iter().for_each(|(_, (position, velocity))| {
        position.x += velocity.x;
        position.y += velocity.y;
    });
}

pub struct HecsContext {
    pub world: World,
}

impl HecsContext {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
}

impl Subcontext for HecsContext {}
