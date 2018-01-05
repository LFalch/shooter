extern crate ggez;

// use ggez::audio;
use ggez::conf;
use ggez::event::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::timer;
use ggez::graphics::{self, Vector2, Point2, Text};

mod obj;
use obj::*;
mod tex;
use tex::*;

#[derive(Debug, Default)]
struct InputState {
    hor: f32,
    ver: f32,
}

struct State {
    input: InputState,
    assets: Assets,
    width: u32,
    height: u32,
    alt_time: f32,
    player: PhysObj,
    rot_text: Text,
    vel_text: Text,
    pos_text: Text,
    acc_text: Text,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.print_resource_stats();
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());
        let assets = Assets::new(ctx)?;

        let rot_text = Text::new(ctx, "Rotation: {:6.2}", &assets.font)?;
        let pos_text = Text::new(ctx, "Pos: (0.00, 0.00)", &assets.font)?;
        let vel_text = Text::new(ctx, "Vel: (0.00, 0.00)", &assets.font)?;
        let acc_text = Text::new(ctx, "Acc: (0.00, 0.00)", &assets.font)?;
        let width = ctx.conf.window_mode.width;
        let height = ctx.conf.window_mode.height;

        Ok(State {
            input: Default::default(),
            assets,
            width,
            height,
            alt_time: 0.,
            rot_text,
            pos_text,
            vel_text,
            acc_text,
            player: PhysObj::new(Point2::new(width as f32 / 2., height as f32 / 2.), Sprite::ShipOff)
        })
    }
    fn update_ui(&mut self, ctx: &mut Context) {
        let pos_str = format!("Pos: ({:8.2}, {:8.2})", self.player.obj.pos.x, self.player.obj.pos.y);
        let vel_str = format!("Vel: ({:8.2}, {:8.2})", self.player.vel.x, self.player.vel.y);
        let acc_str = format!("Acc: ({:8.2}, {:8.2}). Sprite: {:?}", self.player.acc.x, self.player.acc.y, self.player.obj.spr);
        let mut rot = self.player.obj.rot * 180./::std::f32::consts::PI;
        while rot < 0. {
            rot += 360.;
        }
        while rot > 360. {
            rot -= 360.;
        }
        let rot_str = format!("Rotation: {:6.2}", rot);
        self.pos_text = Text::new(ctx, &pos_str, &self.assets.font).unwrap();
        self.vel_text = Text::new(ctx, &vel_str, &self.assets.font).unwrap();
        self.acc_text = Text::new(ctx, &acc_str, &self.assets.font).unwrap();
        self.rot_text = Text::new(ctx, &rot_str, &self.assets.font).unwrap();
    }
}

fn angle_to_vec(angle: f32) -> Vector2 {
    let (sin, cos) = angle.sin_cos();
    Vector2::new(sin, -cos)
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            const DELTA: f32 = 1. / DESIRED_FPS as f32;
            self.alt_time += DELTA;

            self.player.obj.rot += 1.7 * self.input.hor * DELTA;
            self.player.acc = 50. * angle_to_vec(self.player.obj.rot) * self.input.ver;

            if self.alt_time > 2. {
                self.player.obj.spr.toggle();
                self.alt_time -= 2.;
            }

            self.player.update(DELTA);
        }

        self.update_ui(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Our drawing is quite simple.
        // Just clear the screen...
        graphics::clear(ctx);

        self.player.draw(ctx, &self.assets)?;

        let pos_dest = Point2::new(2.0, 0.0);
        let vel_dest = Point2::new(2.0, 14.0);
        let acc_dest = Point2::new(2.0, 28.0);
        let rot_dest = Point2::new(self.width as f32 - self.rot_text.width() as f32 - 5.0, 2.0);
        graphics::draw(ctx, &self.pos_text, pos_dest, 0.0)?;
        graphics::draw(ctx, &self.vel_text, vel_dest, 0.0)?;
        graphics::draw(ctx, &self.acc_text, acc_dest, 0.0)?;
        graphics::draw(ctx, &self.rot_text, rot_dest, 0.0)?;

        // Then we flip the screen...
        graphics::present(ctx);

        // And yield the timeslice
        // This tells the OS that we're done using the CPU but it should
        // get back to this program as soon as it can.
        // This ideally prevents the game from using 100% CPU all the time
        // even if vsync is off.
        // The actual behavior can be a little platform-specific.
        timer::yield_now();
        Ok(())
    }
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _: Mod, repeat: bool) {
        if repeat {
            return
        }
        use Keycode::*;
        match keycode {
            W | Up => self.input.ver += 1.,
            S | Down => self.input.ver -= 1.,
            A | Left => self.input.hor -= 1.,
            D | Right => self.input.hor += 1.,
            Escape => ctx.quit().unwrap(),
            _ => return,
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, repeat: bool) {
        if repeat {
            return
        }
        use Keycode::*;
        match keycode {
            W | Up => self.input.ver -= 1.,
            S | Down => self.input.ver += 1.,
            A | Left => self.input.hor += 1.,
            D | Right => self.input.hor -= 1.,
            _ => return,
        }
    }
}

fn main() {
    let mut ctx = ContextBuilder::new("shooter", "LFalch")
        .window_setup(conf::WindowSetup::default().title("Shooter"))
        .window_mode(conf::WindowMode::default().dimensions(800, 600))
        .build().unwrap();

    if let Ok(manifest_dir) = ::std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = ::std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    match State::new(&mut ctx) {
        Err(e) => {
            println!("Couldn't load game {}", e);
        }
        Ok(mut game) => {
            match run(&mut ctx, &mut game) {
                Ok(_) => println!("Clean exit"),
                Err(e) => println!("Error occured: {}", e)
            }
        }
    }
}
