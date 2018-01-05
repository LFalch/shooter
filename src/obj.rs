use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2, Vector2};
use ggez::nalgebra as na;

use super::{Assets, Sprite};

pub struct Obj {
    pub pos: Point2,
    pub spr: Sprite,
    pub rot: f32,
}

impl Obj {
    pub fn new(pos: Point2, sprite: Sprite) -> Self {
        Obj {
            pos,
            spr: sprite,
            rot: 0.
        }
    }
    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        let drawparams = graphics::DrawParam {
            dest: self.pos,
            rotation: self.rot,
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        };
        graphics::draw_ex(ctx, assets.get_img(self.spr), drawparams)
    }
}

pub struct PhysObj {
    pub obj: Obj,
    pub vel: Vector2,
    pub acc: Vector2,
}

impl PhysObj {
    pub fn new(pos: Point2, sprite: Sprite) -> Self {
        PhysObj {
            obj: Obj::new(pos, sprite),
            vel: na::zero(),
            acc: na::zero(),
        }
    }
    #[inline]
    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        self.obj.draw(ctx, assets)
    }
    pub fn update(&mut self, dt: f32) {
        self.obj.pos += 0.5 * self.acc * dt * dt + self.vel * dt;
        self.vel += self.acc * dt;
    }
}
