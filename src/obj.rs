use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2, Vector2};
use ggez::nalgebra as na;

use super::{Assets, Sprite, GREEN, RED};

#[derive(Debug)]
pub struct Obj {
    pub pos: Point2,
    pub spr: Sprite,
    pub rot: f32,
}

impl Obj {
    pub fn new(pos: Point2, sprite: Sprite) -> Self {
        Obj {
            pos,
            spr: sprite,
            rot: 0.
        }
    }
    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        let drawparams = graphics::DrawParam {
            dest: self.pos,
            rotation: self.rot,
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        };
        graphics::draw_ex(ctx, assets.get_img(self.spr), drawparams)
    }
    pub fn collides(&self, oth: &Self) -> bool {
        na::distance(&self.pos, &oth.pos) <= self.spr.radius() + oth.spr.radius()
    }
    pub fn uncollide(&mut self, oth: &mut Self) {
        let center = na::center(&self.pos, &oth.pos);
        let diff_vec = self.pos - oth.pos;
        let dir = na::normalize(&diff_vec);

        let diff = (self.spr.radius()+oth.spr.radius())/2. * dir;

        self.pos = center + diff;
        oth.pos = center - diff;
    }
}

#[derive(Debug)]
pub struct RotatableObj {
    pub rot_vel: f32,
    pub obj: PhysObj,
}

use std::ops::{Deref, DerefMut};

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
    pub fn new(pos: Point2, sprite: Sprite, rot_vel: f32) -> Self {
        RotatableObj {
            obj: PhysObj::new(pos, sprite),
            rot_vel
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.obj.update(dt);
        self.obj.obj.rot += self.rot_vel * dt;
    }
}

#[derive(Debug)]
pub struct PhysObj {
    pub obj: Obj,
    pub vel: Vector2,
    pub acc: Vector2,
}

impl PhysObj {
    pub fn new(pos: Point2, sprite: Sprite) -> Self {
        PhysObj {
            obj: Obj::new(pos, sprite),
            vel: na::zero(),
            acc: na::zero(),
        }
    }
    #[inline]
    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        self.obj.draw(ctx, assets)?;

        let vel = self.pos+self.vel;

        graphics::set_color(ctx, GREEN)?;
        graphics::line(ctx, &[self.pos, vel], 2.)?;
        graphics::set_color(ctx, RED)?;
        graphics::line(ctx, &[vel, vel + self.acc], 2.)?;
        graphics::set_color(ctx, graphics::WHITE)
    }
    pub fn update(&mut self, dt: f32) {
        self.obj.pos += 0.5 * self.acc * dt * dt + self.vel * dt;
        self.vel += self.acc * dt;
    }
    /// Realistic elastic colliosin
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
