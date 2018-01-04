extern crate ggez;

// use ggez::audio;
use ggez::conf;
use ggez::event::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics;
use ggez::timer;
use ggez::graphics::{Vector2, Point2};
use ggez::nalgebra as na;

struct Assets {
    ship: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let ship = graphics::Image::new(ctx, "ship.png")?;

        Ok(Assets {
            ship
        })
    }
}

struct State {
    assets: Assets,
    width: u32,
    height: u32,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.print_resource_stats();
       graphics::set_background_color(ctx, (0, 0, 0, 255).into());

       println!("Game resource path: {:?}", ctx.filesystem);

        Ok(State {
            assets: Assets::new(ctx)?,
            width: ctx.conf.window_mode.width,
            height: ctx.conf.window_mode.height,
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {

        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Our drawing is quite simple.
        // Just clear the screen...
        graphics::clear(ctx);

        // Then we flip the screen...
        graphics::present(ctx);

        let pos = Point2::new(0., 0.);
        let image = &self.assets.ship;
        let drawparams = graphics::DrawParam {
            dest: pos,
            rotation: 0.,
            offset: graphics::Point2::new(0.5, 0.5),
            .. Default::default()
        };
        graphics::draw_ex(ctx, image, drawparams)?;

        // And yield the timeslice
        // This tells the OS that we're done using the CPU but it should
        // get back to this program as soon as it can.
        // This ideally prevents the game from using 100% CPU all the time
        // even if vsync is off.
        // The actual behavior can be a little platform-specific.
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    let cb = ContextBuilder::new("shooter", "LFalch")
        .window_setup(conf::WindowSetup::default().title("Shooter"))
        .window_mode(conf::WindowMode::default().dimensions(800, 600));
        // .add_resource_path("res");

    let mut ctx = cb.build().unwrap();

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
