use ggez::{Context, GameResult};
use ggez::graphics::{Image, Font};

#[derive(Debug, Copy, Clone)]
pub enum Sprite {
    ShipOn,
    ShipOff,
    ShipSpeed1,
    ShipSpeed2,
    ShipSpeed3,
}

pub struct Assets {
    ship_on: Image,
    ship_off: Image,
    ship_speed1: Image,
    ship_speed2: Image,
    ship_speed3: Image,
    pub font: Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let ship_on = Image::new(ctx, "/ship_on.png")?;
        let ship_off = Image::new(ctx, "/ship_off.png")?;
        let ship_speed1 = Image::new(ctx, "/ship_speed1.png")?;
        let ship_speed2 = Image::new(ctx, "/ship_speed2.png")?;
        let ship_speed3 = Image::new(ctx, "/ship_speed3.png")?;

        Ok(Assets {
            ship_on,
            ship_off,
            ship_speed1,
            ship_speed2,
            ship_speed3,
            font: Font::new(ctx, "/FiraMono.ttf", 13)?,
        })
    }
    pub fn get_img(&self, s: Sprite) -> &Image {
        use Sprite::*;

        match s {
            ShipOn => &self.ship_on,
            ShipOff => &self.ship_off,
            ShipSpeed1 => &self.ship_speed1,
            ShipSpeed2 => &self.ship_speed2,
            ShipSpeed3 => &self.ship_speed3,
        }
    }
}
