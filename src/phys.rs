use std::ops::{Deref, DerefMut};
use ::*;
use ggez::graphics::Image;

#[derive(Debug, Deserialize, Serialize)]
/// An object with physical behaviour such as velocity and acceleration
pub struct PhysObj {
    /// The inner `Obj` that it dereferences to
    pub obj: Obj,
    #[serde(serialize_with = "::save::vec_ser", deserialize_with = "::save::vec_des")]
    /// The velocity of the object
    pub vel: Vector2,
    #[serde(serialize_with = "::save::vec_ser", deserialize_with = "::save::vec_des")]
    /// The acceleration of the object
    pub acc: Vector2,
    /// The mass
    pub mass: f32,
}

impl PhysObj {
    /// Make a new physical object
    pub fn new(pos: Point2, radius: f32) -> Self {
        PhysObj {
            obj: Obj::new(pos, radius),
            vel: na::zero(),
            acc: na::zero(),
            mass: radius.powi(2) * std::f32::consts::PI,
        }
    }
    #[inline]
    /// Draw it
    pub fn draw(&self, ctx: &mut Context, img: &Image) -> GameResult<()> {
        self.obj.draw(ctx, img)
    }
    /// Draw vectors of the velocity and acceleration from this object
    pub fn draw_lines(&self, ctx: &mut Context) -> GameResult<()> {
        let vel = self.pos+self.vel;

        graphics::set_color(ctx, GREEN)?;
        graphics::line(ctx, &[self.pos, vel], 2.)?;
        graphics::set_color(ctx, RED)?;
        graphics::line(ctx, &[vel, vel + self.acc], 2.)
    }
    /// Update its position and velocity using basic physics
    pub fn update(&mut self, dt: f32) {
        self.obj.pos += 0.5 * self.acc * dt * dt + self.vel * dt;
        self.vel += self.acc * dt;
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

impl Deref for PhysObj {
    type Target = Obj;
    fn deref(&self) -> &Obj {
        &self.obj
    }
}
impl DerefMut for PhysObj {
    fn deref_mut(&mut self) -> &mut Obj {
        &mut self.obj
    }
}
