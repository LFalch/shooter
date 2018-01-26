use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2};
use ggez::nalgebra as na;

use super::{Assets, Sprite};

#[derive(Debug)]
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
    pub fn collides(&self, oth: &Self) -> bool {
        na::distance(&self.pos, &oth.pos) <= self.spr.radius() + oth.spr.radius()
    }
    pub fn uncollide(&mut self, oth: &mut Self) {
        let center = na::center(&self.pos, &oth.pos);
        let diff_vec = self.pos - oth.pos;
        let dir = na::normalize(&diff_vec);

        let diff = (self.spr.radius()+oth.spr.radius())/2. * dir;

        self.pos = center + diff;
        oth.pos = center - diff;
    }
}
