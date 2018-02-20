use std::ops::{Deref, DerefMut};

use ggez::graphics::{self, Point2, Vector2};
use ggez::{Context, GameResult};
use {RED, Object, DDELTA, DELTA, Sprite, angle_to_vec};
use super::AsObject;

/// A `PhysObj` with health
#[derive(Debug, Deserialize, Serialize)]
pub struct DestructableObj {
    /// Inner `PhysObj`
    pub obj: Object,
    /// Hit points
    pub health: f32,
}

impl DestructableObj {
    /// Make new `DestructableObj`
    pub fn new(pos: Point2, radius: f32, health: f32) -> Self {
        DestructableObj {
            obj: Object::new(pos, radius),
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
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
impl DerefMut for DestructableObj {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}

/// A self acceleratable `DestructableObj`
#[derive(Debug, Deserialize, Serialize)]
pub struct ThrustedObj {
    obj: DestructableObj,
    #[serde(serialize_with = "::save::vec_ser", deserialize_with = "::save::vec_des")]
    acc: Vector2,
    /// Thruster of the object
    pub thruster: Thruster,
}

impl ThrustedObj {
    /// Creats a new `ThrustedObj`
    pub fn new(pos: Point2, radius: f32, health: f32, thruster: Thruster) -> Self {
        ThrustedObj {
            obj: DestructableObj::new(pos, radius, health),
            acc: Vector2::new(0., 0.),
            thruster,
        }
    }
}

impl AsObject for ThrustedObj {
    fn as_obj(&self) -> &Object {
        &self.obj
    }
    fn as_obj_mut(&mut self) -> &mut Object {
        &mut self.obj
    }
    fn update(&mut self) {
        if self.thruster.power {
            self.acc = self.thruster.burn() * angle_to_vec(self.rot);
        } else {
            self.acc = Vector2::new(0., 0.);
        }
        self.pos += 0.5 * self.acc * DELTA + self.vel * DELTA;
        self.vel += self.acc;
    }
    fn draw_lines(&self, ctx: &mut Context) -> GameResult<()> {
        self.obj.draw_lines(ctx)?;
        let vel = self.pos+self.vel;

        graphics::set_color(ctx, RED)?;
        graphics::line(ctx, &[vel, vel+self.acc/DELTA], 2.)
    }
}

impl Deref for ThrustedObj {
    type Target = DestructableObj;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
impl DerefMut for ThrustedObj {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}


#[derive(Debug, Serialize, Deserialize)]
/// The engine
pub struct Thruster {
    /// The current fuel
    pub fuel: f64,
    /// The current level of throttle (0...max_throttle)
    pub throttle_usage: f64,
    /// Whether it is turned on or off
    pub power: bool,
    efficiency: f32,
    max_throttle: f64,
}

impl Thruster {
    /// New `Thruster`
    pub fn new(fuel: f64, efficiency: f32, max_throttle: f64) -> Self {
        Thruster {
            fuel,
            efficiency,
            max_throttle,
            throttle_usage: 0.,
            power: false,
        }
    }
    /// Burn fuel and return the acceleration provided
    pub fn burn(&mut self) -> f32 {
        let mut usg = self.throttle_usage * DDELTA;
        if usg > self.fuel {
            self.power = false;
            usg = self.fuel;
        }
        self.fuel -= usg;

        self.efficiency * usg as f32
    }
    /// Changes throttle by adding the amount and making sure its in bounds
    pub fn throttle(&mut self, throttle: f64) {
        self.throttle_usage += throttle;
        if self.throttle_usage < 0. {
            self.throttle_usage = 0.;
        } else if self.throttle_usage > self.max_throttle {
            self.throttle_usage = self.max_throttle;
        }
    }
    /// The sprite of the ship with the current engine mode
    pub fn sprite(&self) -> Sprite {
        if !self.power || self.throttle_usage <= 0. {
             Sprite::ShipOff
        } else if self.throttle_usage <= 4.5 {
            Sprite::ShipOn
        } else if self.throttle_usage <= 9. {
            Sprite::ShipLit
        } else if self.throttle_usage <= 22.5 {
            Sprite::ShipSpeed2
        } else {
            Sprite::ShipSpeed3
        }
    }
}

const PLAYER_ENGINE: Thruster = Thruster {
    fuel: 2e3,
    throttle_usage: 0.,
    power: false,
    efficiency: 7.3,
    max_throttle: 45.,
};

/// Make a player
pub fn make_player(p: Point2) -> ThrustedObj {
    ThrustedObj::new(p, Sprite::ShipOff.radius(), 40., PLAYER_ENGINE)
}
/// Makes a `PhysObj` with the size of bullet
pub fn make_bullet(p: Point2) -> Object {
    Object::new(p, Sprite::Bullet.radius())
}
/// Makes a `PhysObj` with the size of fuel
pub fn make_fuel(p: Point2) -> Object {
    Object::new(p, Sprite::Fuel.radius())
}
/// Makes a `DestructableObj` with the size of asteroid
pub fn make_asteroid(p: Point2) -> DestructableObj {
    DestructableObj::new(p, Sprite::Asteroid.radius(), 100.)
}
