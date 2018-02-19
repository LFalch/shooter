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
    /// The mass
    pub mass: f32,
}

impl PhysObj {
    /// Make a new physical object
    pub fn new(pos: Point2, radius: f32) -> Self {
        PhysObj {
            obj: Obj::new(pos, radius),
            vel: na::zero(),
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
        graphics::line(ctx, &[self.pos, vel], 2.)
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
/// A `PhysObj` with health
#[derive(Debug, Deserialize, Serialize)]
pub struct DestructableObj {
    /// Inner `PhysObj`
    pub obj: PhysObj,
    /// Hit points
    pub health: f32,
}

impl DestructableObj {
    /// Make new `DestructableObj`
    pub fn new(pos: Point2, radius: f32, health: f32) -> Self {
        DestructableObj {
            obj: PhysObj::new(pos, radius),
            health
        }
    }
    /// Take damage
    pub fn hit(&mut self, dmg: f32) {
        self.health -= dmg;
        if self.health <= 0. {
            self.health = 0.;
        }
    }
    /// Check whether it's destroyed
    pub fn is_dead(&self) -> bool {
        // To account for NaN, we use not higher than
        !(self.health > 0.)
    }
}

impl Deref for DestructableObj {
    type Target = PhysObj;
    fn deref(&self) -> &PhysObj {
        &self.obj
    }
}
impl DerefMut for DestructableObj {
    fn deref_mut(&mut self) -> &mut PhysObj {
        &mut self.obj
    }
}

/// Makes a `PhysObj` with the size of bullet
pub fn make_bullet(p: Point2) -> PhysObj {
    PhysObj::new(p, Sprite::Bullet.radius())
}
/// Makes a `PhysObj` with the size of fuel
pub fn make_fuel(p: Point2) -> PhysObj {
    PhysObj::new(p, Sprite::Fuel.radius())
}
/// Makes a `DestructableObj` with the size of asteroid
pub fn make_asteroid(p: Point2) -> DestructableObj {
    DestructableObj::new(p, Sprite::Asteroid.radius(), 100.)
}
