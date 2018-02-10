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
        #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
        /// An object to reference a sprite in the `Assets`
        #[allow(missing_docs)]
        pub enum Sprite {
            $($name,)*
        }

        impl Sprite {
            /// Width of the sprite
            pub fn width(&self) -> f32 {
                match *self {
                    $(
                        Sprite::$name => $width,
                    )*
                }
            }
            /// Height of the sprite
            pub fn height(&self) -> f32 {
                match *self {
                    $(
                        Sprite::$name => $height,
                    )*
                }
            }
            /// Radius to use with collision physics
            pub fn radius(&self) -> f32 {
                match *self {
                    $(
                        Sprite::$name => $radius,
                    )*
                }
            }
        }
        /// All the assets
        pub struct Assets {
            $(
                $tex: Image,
            )*
            /// The font used for all the text
            pub font: Font,
        }

        impl Assets {
            /// Initialises the assets with the context
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
            /// Gets the `Image` to draw from the sprite
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

/// Load all assets and specify their dimensions
sprites! {
    ShipOn, ship_on, 48., 48., 20.,
    ShipOff, ship_off, 48., 48., 20.,
    ShipLit, ship_lit, 48., 48., 20.,
    ShipSpeed2, ship_speed2, 48., 48., 20.,
    ShipSpeed3, ship_speed3, 48., 48., 20.,
    Asteroid, asteroid, 48., 48., 24.,
    StarsBg, stars_bg, 2560., 1440., 0./0.,
    Fuel, fuel, 32., 32., 16.,
}

impl Assets {
    /// Make a positional text object
    pub fn text(&self, context: &mut Context, pos: Point2, text: &str) -> GameResult<PosText> {
        let text = Text::new(context, text, &self.font)?;
        Ok(PosText {
            pos,
            text
        })
    }
    /// Make a postional text object from the right side of the screen
    pub fn text_ra(&self, context: &mut Context, x: f32, y: f32, text: &str) -> GameResult<PosText> {
        let text = Text::new(context, text, &self.font)?;
        Ok(PosText{
            pos: Point2::new(x - text.width() as f32, y),
            text
        })
    }
}

#[derive(Debug, Clone)]
/// A text with a position
///
/// Used for convenience so it's easier to update the text and rememeber their coordinates on the screen
pub struct PosText {
    pos: Point2,
    text: Text
}

impl PosText {
    /// Draw the text
    pub fn draw_text(&self, ctx: &mut Context) -> GameResult<()> {
        self.text.draw(ctx, self.pos, 0.)
    }
    /// Update the text
    pub fn update_text(&mut self, a: &Assets, ctx: &mut Context, text: &str) -> GameResult<()> {
        if text != self.text.contents() {
            self.text = Text::new(ctx, text, &a.font)?;
        }
        Ok(())
    }
}
