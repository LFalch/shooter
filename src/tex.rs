use ggez::{Context, GameResult};
use ggez::graphics::{Image, Font, Text, Point2, Drawable, BlendMode, DrawParam};

#[derive(Debug, Copy, Clone)]
pub enum Sprite {
    ShipOn,
    ShipOff,
    ShipSpeed1,
    ShipSpeed2,
    ShipSpeed3,
    Asteroid,
}

pub struct Assets {
    ship_on: Image,
    ship_off: Image,
    ship_speed1: Image,
    ship_speed2: Image,
    ship_speed3: Image,
    asteroid: Image,
    pub font: Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let ship_on = Image::new(ctx, "/ship_on.png")?;
        let ship_off = Image::new(ctx, "/ship_off.png")?;
        let ship_speed1 = Image::new(ctx, "/ship_speed1.png")?;
        let ship_speed2 = Image::new(ctx, "/ship_speed2.png")?;
        let ship_speed3 = Image::new(ctx, "/ship_speed3.png")?;
        let asteroid = Image::new(ctx, "/asteroid.png")?;

        Ok(Assets {
            ship_on,
            ship_off,
            ship_speed1,
            ship_speed2,
            ship_speed3,
            asteroid,
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
            Asteroid => &self.asteroid,
        }
    }
    pub fn text(&self, context: &mut Context, pos: Point2, text: &str) -> GameResult<PosText> {
        let text = Text::new(context, text, &self.font)?;
        Ok(PosText {
            pos,
            text
        })
    }
    pub fn text_ra(&self, context: &mut Context, x: f32, y: f32, text: &str) -> GameResult<PosText> {
        let text = Text::new(context, text, &self.font)?;
        Ok(PosText{
            pos: Point2::new(x - text.width() as f32, y),
            text
        })
    }
}

#[derive(Debug, Clone)]
pub struct PosText {
    pos: Point2,
    text: Text
}

impl PosText {
    pub fn draw_text(&self, ctx: &mut Context) -> GameResult<()> {
        self.text.draw(ctx, self.pos, 0.)
    }
    pub fn update_ra(&mut self, x: f32) {
        self.pos = Point2::new(x - self.text.width() as f32, self.pos.y);
    }
    pub fn update_text(&mut self, a: &Assets, ctx: &mut Context, text: &str) -> GameResult<()> {
        self.text = Text::new(ctx, text, &a.font)?;
        Ok(())
    }
}

impl Drawable for PosText {
    #[inline]
    fn draw_ex(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        self.text.draw_ex(ctx, param)
    }
    #[inline]
    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.text.set_blend_mode(mode)
    }
    #[inline]
    fn get_blend_mode(&self) -> Option<BlendMode> {
        self.text.get_blend_mode()
    }
}
