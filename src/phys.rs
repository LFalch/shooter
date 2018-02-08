use std::ops::{Deref, DerefMut};
use ::*;

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
}

impl PhysObj {
    /// Make a new physical object
    pub fn new(pos: Point2, sprite: Sprite) -> Self {
        PhysObj {
            obj: Obj::new(pos, sprite),
            vel: na::zero(),
            acc: na::zero(),
        }
    }
    #[inline]
    /// Draw it
    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        self.obj.draw(ctx, assets)
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
        const M1: f32 = 1.;
        const M2: f32 = 1.;
        let v1 = self.vel;
        let v2 = oth.vel;

        let mass_quotient = 2./(M1+M2);

        let dist = self.pos - oth.pos;
        let dist_n2 = dist.norm_squared();

        let vel_diff = mass_quotient * na::dot(&(v1-v2), &dist)/dist_n2*dist;

        let vel1_diff = M2 * vel_diff;
        self.vel -= vel1_diff;
        let vel2_diff = M1 * -vel_diff;
        oth.vel -= vel2_diff;

        (vel1_diff, vel2_diff)
    }
    /// Rebound off edge
    pub fn rebound(&mut self, width: f32, height: f32) {
        let w = self.spr.width()/2.;
        let h = self.spr.height()/2.;

        if self.pos.x - w < 0. {
            self.pos.x = -self.pos.x + 2.*w;
            self.vel.x = -self.vel.x;
        } else if self.pos.x + w > width {
            self.pos.x = self.pos.x - 2.*(self.pos.x + w - width);
            self.vel.x = -self.vel.x;
        }

        if self.pos.y - h < 0. {
            self.pos.y = -self.pos.y + 2.*h;
            self.vel.y = -self.vel.y;
        } else if self.pos.y + h > height {
            self.pos.y = self.pos.y - 2.*(self.pos.y + h-height);
            self.vel.y = -self.vel.y;
        }
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
#[derive(Debug, Serialize, Deserialize)]
/// An object with a rotational velocity
///
/// Dereferences to a `PhysObj`
pub struct RotatableObj {
    /// The rotation in radians per second
    pub rot_vel: f32,
    /// The inner physics object
    pub obj: PhysObj,
}

impl Deref for RotatableObj {
    type Target = PhysObj;
    fn deref(&self) -> &PhysObj {
        &self.obj
    }
}
impl DerefMut for RotatableObj {
    fn deref_mut(&mut self) -> &mut PhysObj {
        &mut self.obj
    }
}

impl RotatableObj {
    /// Make a rotatable object
    pub fn new(pos: Point2, sprite: Sprite, rot_vel: f32) -> Self {
        RotatableObj {
            obj: PhysObj::new(pos, sprite),
            rot_vel
        }
    }
    /// Update its rotation and the inner physics object
    pub fn update(&mut self, dt: f32) {
        self.obj.update(dt);
        self.rot += self.rot_vel * dt;
    }
}
