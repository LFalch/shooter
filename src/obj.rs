use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2, Image};
use ggez::nalgebra as na;

#[derive(Debug, Serialize, Deserialize)]
/// A simple object that can be drawn to the screen
pub struct Obj {
    #[serde(serialize_with = "::save::point_ser", deserialize_with = "::save::point_des")]
    /// The position of the object
    pub pos: Point2,
    /// The radius of the object used for collision
    pub rad: f32,
    /// The rotation of the obejct in radians
    pub rot: f32,
}

impl Obj {
    /// Make a new object with a sprite and a position
    pub fn new(pos: Point2, radius: f32) -> Self {
        Obj {
            pos,
            rad: radius,
            rot: 0.
        }
    }
    /// Draw the object
    pub fn draw(&self, ctx: &mut Context, img: &Image) -> GameResult<()> {
        let drawparams = graphics::DrawParam {
            dest: self.pos,
            rotation: self.rot,
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        };
        graphics::draw_ex(ctx, img, drawparams)
    }
    /// Check if it collides with another object (circle collision)
    pub fn collides(&self, oth: &Self) -> bool {
        // If the distance is lower than the sum of the radii of the objects, they're colliding
        na::distance(&self.pos, &oth.pos) <= self.rad + oth.rad
    }
    /// Move the objects so they only barely don't collide
    ///
    /// This is acheived by finding the center between them and moving them so the both only
    /// barely touch each other on the sides
    ///
    /// This could be done more correctly done by taking into account their velocity and finding
    /// out where they actually would've met if the timesteps were infinitesimal
    pub fn uncollide(&mut self, oth: &mut Self) {
        let center = na::center(&self.pos, &oth.pos);
        let diff_vec = self.pos - oth.pos;
        let dir = na::normalize(&diff_vec);

        let diff = (self.rad + oth.rad) / 2. * dir;

        self.pos = center + diff;
        oth.pos = center - diff;
    }
}
