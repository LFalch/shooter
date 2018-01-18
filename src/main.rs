extern crate ggez;

// use ggez::audio;
use ggez::conf;
use ggez::event::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::timer;
use ggez::graphics::{self, Vector2, Point2, Text, Color};

mod obj;
use obj::*;
mod tex;
use tex::*;

#[derive(Debug, Default)]
struct InputState {
    hor: i8,
    ver: i8,
}

impl InputState {
    #[inline]
    fn hor(&self) -> f32 {
        self.hor.signum() as f32
    }
    #[inline]
    fn ver(&self) -> f32 {
        self.ver.signum() as f32
    }
}

struct State {
    input: InputState,
    assets: Assets,
    width: u32,
    height: u32,
    on_time: f32,
    player: PhysObj,
    asteroids: Vec<RotatableObj>,
    rot_text: PosText,
    vel_text: PosText,
    pos_text: PosText,
    acc_text: PosText,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.print_resource_stats();
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());
        let assets = Assets::new(ctx)?;

        let width = ctx.conf.window_mode.width;
        let height = ctx.conf.window_mode.height;

        let acc_text = assets.text(ctx, Point2::new(2.0, 0.0), "Acc: (0.00, 0.00)")?;
        let pos_text = assets.text(ctx, Point2::new(2.0, 14.0), "Pos: (0.00, 0.00)")?;
        let vel_text = assets.text(ctx, Point2::new(2.0, 28.0), "Vel: (0.00, 0.00)")?;
        let rot_text = assets.text_ra(ctx, width as f32 - 5.0, 2.0, "Rotation: {:6.2}")?;

        Ok(State {
            input: Default::default(),
            assets,
            width,
            height,
            on_time: 0.,
            rot_text,
            pos_text,
            vel_text,
            acc_text,
            asteroids: vec![RotatableObj::new(Point2::new(50., 50.), Sprite::Asteroid, 0.1)],
            player: PhysObj::new(Point2::new(width as f32 / 2., height as f32 / 2.), Sprite::ShipOff)
        })
    }
    fn update_ui(&mut self, ctx: &mut Context) {
        let pos_str = format!("Pos: ({:8.2}, {:8.2})", self.player.obj.pos.x, self.player.obj.pos.y);
        let vel_str = format!("Vel: ({:8.2}, {:8.2})", self.player.vel.x, self.player.vel.y);
        let acc_str = format!("Acc: ({:8.2}, {:8.2})", self.player.acc.x, self.player.acc.y);
        let mut rot = self.player.obj.rot * 180./::std::f32::consts::PI;
        while rot < 0. {
            rot += 360.;
        }
        while rot > 360. {
            rot -= 360.;
        }
        let rot_str = format!("Rotation: {:6.2}", rot);
        self.pos_text.update_text(&self.assets, ctx, &pos_str).unwrap();
        self.vel_text.update_text(&self.assets, ctx, &vel_str).unwrap();
        self.acc_text.update_text(&self.assets, ctx, &acc_str).unwrap();
        self.rot_text.update_text(&self.assets, ctx, &rot_str).unwrap();
        self.rot_text.update_ra(self.width as f32 - 5.0);
    }
}

fn angle_to_vec(angle: f32) -> Vector2 {
    let (sin, cos) = angle.sin_cos();
    Vector2::new(sin, -cos)
}

pub fn angle_from_vec(v: &Vector2) -> f32 {
    let x = v.x;
    let y = v.y;

    x.atan2(-y)
}

pub const GREEN: Color = Color{r:0.,g:1.,b:0.,a:0.5};
pub const RED: Color = Color{r:1.,g:0.,b:0.,a:0.5};

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            const DELTA: f32 = 1. / DESIRED_FPS as f32;

            if self.input.ver != 0 {
                self.on_time += DELTA;
                self.player.obj.spr = Sprite::ShipOn;
            } else {
                self.on_time = 0.;
                self.player.obj.spr = Sprite::ShipOff;
            }
            let acc;
            if self.on_time > 0.5 {
                if self.on_time > 1.5 {
                    if self.on_time > 2.3 {
                        self.player.obj.spr = Sprite::ShipSpeed3;
                        acc = 65.;
                    } else {
                        self.player.obj.spr = Sprite::ShipSpeed2;
                        acc = 50.;
                    }
                } else {
                    self.player.obj.spr = Sprite::ShipSpeed1;
                    acc = 30.;
                }
            } else {
                acc = 10.;
            }
            self.player.obj.rot += 1.7 * self.input.hor() * DELTA;
            self.player.acc = acc * angle_to_vec(self.player.obj.rot) * self.input.ver();

            self.player.update(DELTA);

            for ast in &mut self.asteroids {
                ast.update(DELTA);
                if self.player.collides(&ast) {
                    println!("ARGH!");
                }
            }
        }

        self.update_ui(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Our drawing is quite simple.
        // Just clear the screen...
        graphics::clear(ctx);

        self.player.draw(ctx, &self.assets)?;
        for ast in &self.asteroids {
            ast.draw(ctx, &self.assets)?;
        }

        self.pos_text.draw_text(ctx)?;
        self.vel_text.draw_text(ctx)?;
        self.acc_text.draw_text(ctx)?;
        self.rot_text.draw_text(ctx)?;

        // Then we flip the screen...
        graphics::present(ctx);

        timer::yield_now();
        Ok(())
    }
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _: Mod, repeat: bool) {
        if repeat {
            return
        }
        use Keycode::*;
        match keycode {
            W | Up => self.input.ver += 1,
            S | Down => self.input.ver -= 1,
            A | Left => self.input.hor -= 1,
            D | Right => self.input.hor += 1,
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
            W | Up => self.input.ver -= 1,
            S | Down => self.input.ver += 1,
            A | Left => self.input.hor += 1,
            D | Right => self.input.hor -= 1,
            _ => return,
        }
    }
}

fn main() {
    let mut ctx = ContextBuilder::new("shooter", "LFalch")
        .window_setup(conf::WindowSetup::default().title("Shooter"))
        .window_mode(conf::WindowMode::default().dimensions(1000, 750))
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
