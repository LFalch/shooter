use ggez::{Context, GameResult};
use ggez::graphics::{Image, Font, Text, Point2, Drawable};

macro_rules! sprites {
    ($(
        $name:ident,
        $tex:ident,
        $width:expr,
        $height:expr,
        $radius: expr,
    )*) => (
        #[derive(Debug, Copy, Clone)]
        pub enum Sprite {
            $($name,)*
        }

        impl Sprite {
            pub fn width(&self) -> f32 {
                match *self {
                    $(
                        Sprite::$name => $width,
                    )*
                }
            }
            pub fn height(&self) -> f32 {
                match *self {
                    $(
                        Sprite::$name => $height,
                    )*
                }
            }
            pub fn radius(&self) -> f32 {
                match *self {
                    $(
                        Sprite::$name => $radius,
                    )*
                }
            }
        }
        pub struct Assets {
            $(
                $tex: Image,
            )*
            pub font: Font,
        }

        impl Assets {
            pub fn new(ctx: &mut Context) -> GameResult<Self> {
                $(
                    let $tex = Image::new(ctx, concat!("/", stringify!($tex), ".png"))?;
                )*

                Ok((Assets {
                    $(
                        $tex,
                    )*
                    font: Font::new(ctx, "/FiraMono.ttf", 13)?,
                }))
            }
            pub fn get_img(&self, s: Sprite) -> &Image {
                match s {
                    $(
                        Sprite::$name => &self.$tex,
                    )*
                }
            }
        }
    );
}

sprites! {
    ShipOn, ship_on, 48., 48., 20.,
    ShipOff, ship_off, 48., 48., 20.,
    ShipSpeed1, ship_speed1, 48., 48., 20.,
    ShipSpeed2, ship_speed2, 48., 48., 20.,
    ShipSpeed3, ship_speed3, 48., 48., 20.,
    Asteroid, asteroid, 48., 48., 24.,
}

impl Assets {
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
    pub fn update_text(&mut self, a: &Assets, ctx: &mut Context, text: &str) -> GameResult<()> {
        self.text = Text::new(ctx, text, &a.font)?;
        Ok(())
    }
}
