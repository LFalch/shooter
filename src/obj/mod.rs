pub(super) mod phys;

use std::f32::consts::PI;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2, Vector2, Image};
use ggez::nalgebra as na;

use GREEN;

#[derive(Debug, Serialize, Deserialize)]
/// A simple object that can be drawn to the screen
pub struct Object {
    #[serde(serialize_with = "::save::point_ser", deserialize_with = "::save::point_des")]
    /// The position of the object
    pub pos: Point2,
    /// The radius of the object used for collision
    pub rad: f32,
    /// The rotation of the obejct in radians
    pub rot: f32,
    #[serde(serialize_with = "::save::vec_ser", deserialize_with = "::save::vec_des")]
    /// The velocity of the object
    pub vel: Vector2,
    /// The mass
    pub mass: f32,
}

impl Object {
    /// Make a new physics object
    pub fn new(pos: Point2, rad: f32) -> Self {
        Object {
            pos,
            rad,
            rot: 0.,
            vel: na::zero(),
            mass: rad.powi(2) * PI,
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
    /// Draw vectors of the velocity from this object
    pub fn draw_lines(&self, ctx: &mut Context) -> GameResult<()> {
        let vel = self.pos+self.vel;

        graphics::set_color(ctx, GREEN)?;
        graphics::line(ctx, &[self.pos, vel], 2.)
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
    /// Update its position and velocity using basic physics
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }
    /// Realistic elastic collision
    pub fn elastic_collide(&mut self, oth: &mut Self) -> (Vector2, Vector2) {
        let m1 = self.mass;
        let m2 = oth.mass;
        let v1 = self.vel;
        let v2 = oth.vel;

        let mass_quotient = 2./(m1+m2);

        let dist = self.pos - oth.pos;
        let dist_n2 = dist.norm_squared();

        let vel_diff = mass_quotient * na::dot(&(v1-v2), &dist)/dist_n2*dist;

        let vel1_diff = m2 * vel_diff;
        self.vel -= vel1_diff;
        let vel2_diff = m1 * -vel_diff;
        oth.vel -= vel2_diff;

        (vel1_diff, vel2_diff)
    }
}
