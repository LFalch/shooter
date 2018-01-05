use ggez::{Context, GameResult};
use ggez::graphics::{Image, Font};

#[derive(Debug, Copy, Clone)]
pub enum Sprite {
    ShipOld,
    ShipOn,
    ShipOff,
    ShipSpeed1,
    ShipSpeed2,
    ShipSpeed3,
}

impl Sprite {
    pub fn toggle(&mut self) {
        use Sprite::*;
        *self = match *self {
            ShipOld => ShipOld,

            ShipOn => ShipSpeed1,
            ShipSpeed1 => ShipSpeed2,
            ShipSpeed2 => ShipSpeed3,
            ShipSpeed3 => ShipOff,
            ShipOff => ShipOn,
        }
    }
}

pub struct Assets {
    ship: Image,
    ship_on: Image,
    ship_off: Image,
    ship_speed1: Image,
    ship_speed2: Image,
    ship_speed3: Image,
    pub font: Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let ship = Image::new(ctx, "/ship.png")?;
        let ship_on = Image::new(ctx, "/ship_on.png")?;
        let ship_off = Image::new(ctx, "/ship_off.png")?;
        let ship_speed1 = Image::new(ctx, "/ship_speed1.png")?;
        let ship_speed2 = Image::new(ctx, "/ship_speed2.png")?;
        let ship_speed3 = Image::new(ctx, "/ship_speed3.png")?;

        Ok(Assets {
            ship,
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
            ShipOld => &self.ship,
            ShipOn => &self.ship_on,
            ShipOff => &self.ship_off,
            ShipSpeed1 => &self.ship_speed1,
            ShipSpeed2 => &self.ship_speed2,
            ShipSpeed3 => &self.ship_speed3,
        }
    }
}
