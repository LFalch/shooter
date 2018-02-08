use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2};
use ggez::nalgebra as na;

use super::{Assets, Sprite};

#[derive(Debug, Serialize, Deserialize)]
/// A simple object that can be drawn to the screen
pub struct Obj {
    #[serde(serialize_with = "::save::point_ser", deserialize_with = "::save::point_des")]
    /// The position of the object
    pub pos: Point2,
    /// The sprite to draw the object with
    pub spr: Sprite,
    /// The rotation of the obejct in radians
    pub rot: f32,
}

use std::f32::consts::FRAC_PI_2;

impl Obj {
    /// Make a new object with a sprite and a position
    pub fn new(pos: Point2, sprite: Sprite) -> Self {
        Obj {
            pos,
            spr: sprite,
            rot: 0.
        }
    }
    /// Draw the object
    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        let drawparams = graphics::DrawParam {
            dest: self.pos,
            // Add half pi to make it consistent with maths functions
            rotation: self.rot + FRAC_PI_2,
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        };
        graphics::draw_ex(ctx, assets.get_img(self.spr), drawparams)
    }
    /// Check if it collides with another object (circle collision)
    pub fn collides(&self, oth: &Self) -> bool {
        // If the distance is lower than the sum of the radii of the objects, they're colliding
        na::distance(&self.pos, &oth.pos) <= self.spr.radius() + oth.spr.radius()
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

        let diff = (self.spr.radius()+oth.spr.radius())/2. * dir;

        self.pos = center + diff;
        oth.pos = center - diff;
    }
}
