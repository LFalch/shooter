use ggez::{Context, GameResult};
use ggez::graphics::{Image, Font};

#[derive(Debug, Copy, Clone)]
pub enum Sprite {
    Ship
}

pub struct Assets {
    ship: Image,
    pub font: Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let ship = Image::new(ctx, "/ship.png")?;

        Ok(Assets {
            ship,
            font: Font::new(ctx, "/FiraMono.ttf", 13)?,
        })
    }
    pub fn get_img(&self, s: Sprite) -> &Image {
        use Sprite::*;

        match s {
            Ship => &self.ship
        }
    }
}
