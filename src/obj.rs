use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2};

use super::Assets;

pub struct Obj {
    pub pos: Point2,
    pub rot: f32,
}

impl Obj {
    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        let drawparams = graphics::DrawParam {
            dest: self.pos,
            rotation: 0.,
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        };
        graphics::draw_ex(ctx, &assets.ship, drawparams)
    }
}
