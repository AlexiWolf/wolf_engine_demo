use colors_transform::{Color, Hsl};
use hecs::World;
use hecs_schedule::{SubWorld, Schedule};
use log::*;
use rand::Rng;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color as SdlColor, render::Canvas, surface::Surface, rect::{Rect, Point}};
use wolf_engine::{*, utils::profile_scope};
use wolf_engine_sdl2::*;

fn main() {
    logging::initialize_logging(LevelFilter::Debug);
    #[cfg(feature = "http_profiling")]
    EngineBuilder::new()
        .with_plugin(Box::from(SdlPlugin::new(SdlWindowSettings::default())))
        .with_plugin(Box::from(wolf_engine::plugins::PuffinPlugin))
        .with_subcontext(HecsContext::new())
        .build()
        .run(Box::from(MainState::new()));

    #[cfg(not(feature = "http_profiling"))]
    EngineBuilder::new()
        .with_plugin(Box::from(SdlPlugin::new(SdlWindowSettings::default())))
        .with_subcontext(HecsContext::new())
        .build()
        .run(Box::from(MainState::new()));
}

pub struct MainState {
    hew: f32,
    schedule: Schedule,
    entity_count: usize,
}

impl MainState {
    pub fn new() -> Self {
        Self { 
            hew: 0.0,
            schedule: Schedule::builder()
                .add_system(random_walk_system)
                .add_system(movement_system)
                .build(),
            entity_count: 100_000,
        }
    }
}

impl State for MainState {
    fn setup(&mut self, context: &mut Context) {
        let mut hecs = context.borrow_mut::<HecsContext>().expect("No HecsContext");
        let mut entities = Vec::new();
        for _ in 0..self.entity_count {
            let position = Position::new(400.0, 300.0);
            let velocity = Velocity::new(0.0, 0.0);
            entities.push((position, velocity));
        }
        hecs.world.spawn_batch(entities);
    }

    fn update(&mut self, context: &mut Context) -> OptionalTransition {
        self.hew = (self.hew + 1.0) % 360.0;
        if let Some(Ok(mut hecs)) = context.try_borrow_mut::<HecsContext>() {
            self.schedule.execute((&mut hecs.world,)).unwrap();
        }
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        if let Some(Ok(mut sdl_graphics)) = context.try_borrow_mut::<SdlVideoContext>() {
            sdl_graphics
                .canvas
                .set_draw_color(convert_hew_to_color(self.hew));
            sdl_graphics.canvas.clear();
            if let Some(Ok(mut hecs)) = context.try_borrow_mut::<HecsContext>() {
                draw_system(&mut hecs.world, &mut sdl_graphics); 
            }
            sdl_graphics
                .canvas
                .string(10, 10, format!("Hello, World! {} entities", self.entity_count).as_str(), SdlColor::WHITE)
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
        profile_scope!("movement system");
        position.x += velocity.x;
        position.y += velocity.y;
    });
}

pub fn random_walk_system(world: SubWorld<&mut Velocity>) {
    let mut rng = rand::thread_rng();
    world.query::<&mut Velocity>().iter().for_each(|(_, velocity)| {
        profile_scope!("random walk system");
        velocity.x = rng.gen_range(-5.0..5.0);
        velocity.y = rng.gen_range(-5.0..5.0);
    });
}

pub fn draw_system(world: &mut World, sdl: &mut SdlVideoContext) {
    profile_scope!("draw entities system");
    let surface = Surface::new(32, 32, sdl2::pixels::PixelFormatEnum::RGBA8888).expect("failed to create surface");
    let sample_rect = Rect::new(0, 0, 32, 32);
    let mut texture_canvas = Canvas::from_surface(surface).expect("failed to create canvas");
    texture_canvas.set_draw_color(SdlColor::RGBA(0, 0, 0, 0));
    texture_canvas.filled_circle(16, 16, 15, SdlColor::WHITE).unwrap();
    texture_canvas.circle(16, 16, 15, SdlColor::BLACK).unwrap();
    texture_canvas.line(16, 0, 16, 16, SdlColor::BLACK).unwrap();
    texture_canvas.present();

    let texture_creator = sdl.canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(texture_canvas.surface())
        .expect("Failed to create the texture");
    
    world.query_mut::<&mut Position>().into_iter().for_each(|(_, position)|{
        let dest = Rect::new(position.x.round() as i32, position.y.round() as i32, 32, 32);
        sdl.canvas.copy_ex(&texture, Some(sample_rect), Some(dest), 90.0, Point::new(16, 16), false, false).unwrap();
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
