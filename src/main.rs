extern crate ggez;
extern crate nalgebra as na;
extern crate random;
extern crate rustics_2d;

use ggez::conf;
use ggez::event::*;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::timer;
use rustics_2d::*;
use random::Source;

struct MainState {
    world: World,
    cam_pos: Vector3<f32>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut source = random::default().seed([42, 69]);

        let mut s = MainState {
            world: World::new(),
            cam_pos: Vector3::new(0.0, 0.0, 0.0),
        };

        for _ in 0..25 {
            s.world.bodies.push(Body::new(Vector2::new(
                source.read::<f32>() * 10.0 - 5.0,
                source.read::<f32>() * 10.0 - 5.0,
            )));
        }

        if let Ok(text) = graphics::get_renderer_info(ctx) {
            println!("{}", text);
        }

        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::duration_to_f64(timer::get_delta(ctx));
        //println!("FPS: {:?}", timer::get_fps(ctx));
        self.world.step_for(dt, 1.0 / 100.0, 10);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let _dt = timer::duration_to_f64(timer::get_time_since_start(ctx));

        graphics::clear(ctx);

        for body in &self.world.bodies {
            // this is horrible! Need to figure out something!
            let c: Vec<Point2<f32>> = unsafe { std::mem::transmute(body.bbox.get_corners()) };
            graphics::polygon(ctx, graphics::DrawMode::Line(0.01), &c[0..4])?;
        }

        graphics::present(ctx);
        timer::yield_now();

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        // setup view/projection
        let a = width as f32 / height as f32;
        let world_height = 20.0;
        let world_width = 20.0;

        graphics::set_view(ctx, na::Matrix4::new_translation(&self.cam_pos));
        set_orthographic(ctx, world_width * a, world_height, 0.0, 1.0);
        graphics::apply_transformations(ctx).unwrap();
    }

    /// A keyboard button was pressed.
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up => {
                self.cam_pos.y += 1.0;
            }
            Keycode::Down => {
                self.cam_pos.y -= 1.0;
            }
            Keycode::Left => {
                self.cam_pos.x -= 1.0;
            }
            Keycode::Right => {
                self.cam_pos.x += 1.0;
            }
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
        graphics::set_view(ctx, na::Matrix4::new_translation(&self.cam_pos));
        graphics::apply_transformations(ctx).unwrap();
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            _ => (), // Do nothing
        }
    }
}

fn set_orthographic(ctx: &mut Context, width: f32, height: f32, near: f32, far: f32) {
    // unit projection with 0,0 at center
    graphics::set_projection(
        ctx,
        na::Matrix4::new(
            2.0 / width,        // m11
            0.0,                // m12
            0.0,                // m13
            0.0,                // m14
            0.0,                // m21
            2.0 / height,       // m22
            0.0,                // m23
            0.0,                // m24
            0.0,                // m31
            0.0,                // m32
            1.0 / (near - far), // m33
            0.0,                // m34
            0.0,                // m41
            0.0,                // m42
            0.0 / (near - far), // m43
            1.0,                // m44
        ),
    );
}

pub fn main() {
    use ggez::event::EventHandler;

    let width = 800;
    let height = 600;

    let mut c = conf::Conf::new();
    c.window_title = String::from("Rustics-2D Testbed");
    c.window_mode.vsync = false;
    c.window_width = width;
    c.window_height = height;
    c.window_mode.resizable = false;
    c.window_mode.samples = conf::NumSamples::from_u32(16).unwrap();
    let ctx = &mut Context::load_from_conf("Rustics-2D Testbed", "Matt", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    state.resize_event(ctx, width, height);
    run(ctx, state).unwrap();
}
