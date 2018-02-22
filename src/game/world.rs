use ::*;
use self_compare::SliceCompareExt;

#[derive(Debug, Serialize, Deserialize)]
/// All the objects in the current world
pub struct World {
    pub(super) player: ThrustedObj,
    pub(super) asteroids: Objects<DestructableObj>,
    pub(super) fuels: Objects<Object>,
    pub(super) bullets: Objects<Object>,
}

use std::ops::{Deref, DerefMut};

#[derive(Debug, Serialize, Deserialize)]
/// A collection of objects with the same sprite face
pub struct Objects<T: AsObject> {
    inner: Vec<T>,
    sprite: Sprite,
}

impl<T: AsObject> Objects<T> {
    #[inline]
    /// Create new instance
    pub fn new(v: Vec<T>, sprite: Sprite) -> Self {
        Objects {
            inner: v,
            sprite
        }
    }
    /// Draw all objects
    #[inline]
    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        for obj in &self.inner {
            obj.as_obj().draw(ctx, assets.get_img(self.sprite))?;
        }
        Ok(())
    }
    /// Draw lines of all objects
    #[inline]
    pub fn draw_lines(&self, ctx: &mut Context) -> GameResult<()> {
        for obj in &self.inner {
            obj.draw_lines(ctx)?;
        }
        Ok(())
    }
    #[inline]
    /// Hanldes collision of objects in the collection with each other
    fn self_collision(&mut self) {
        self.inner.compare_self_mut(|a, b| check_and_resolve(a.as_obj_mut(), b.as_obj_mut()));
    }
    #[inline]
    /// Update, compare and remove spat out indices
    fn iterate_rmv_indices<F: FnMut(&mut T) -> bool>(&mut self, mut f: F) -> usize {
        let mut to_remove = Vec::new();
        for (i, obj) in self.inner.iter_mut().enumerate().rev() {
            if f(obj) {
                to_remove.push(i);
            }
        }
        let len = to_remove.len();
        for i in to_remove {
            self.inner.remove(i);
        }
        len
    }
    /// Update and compare
    fn iterate<F: FnMut(&mut T)>(&mut self, mut f: F) {
        for obj in &mut self.inner {
            f(obj)
        }
    }
}

impl<T: AsObject> Deref for Objects<T> {
    type Target = Vec<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T: AsObject> DerefMut for Objects<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Collision detection commands
#[inline]
fn check_and_resolve(o1: &mut Object, o2: &mut Object) {
    if o1.collides(&o2) {
        resolve(o1, o2);
    }
}
fn resolve(o1: &mut Object, o2: &mut Object) {
    o1.uncollide(o2);
    o1.elastic_collide(o2);
}

impl World {
    pub(super) fn physics_update(&mut self, input_state: &InputState) {
        let &mut World {
            ref mut player,
            ref mut asteroids,
            ref mut fuels,
            ref mut bullets,
        } = self;

        player.obj.rot += 1.7 * input_state.hor() * DELTA;

        player.thruster.power = input_state.ver() == 1.;
        player.thruster.throttle(input_state.throttle() as f64 * 17. * DDELTA);

        player.update();

        let consumed_fuel = fuels.iterate_rmv_indices(|fuel| {
            fuel.update();
            if player.collides(&fuel) {
                if (fuel.vel - player.vel).norm() <= 30. {
                    true
                } else {
                    resolve(player, fuel);
                    false
                }
            } else {
                false
            }
        });
        player.thruster.fuel += 200. * consumed_fuel as f64;
        asteroids.iterate(|ast| {
            ast.update();
            check_and_resolve(player, ast);
        });

        bullets.iterate_rmv_indices(|bullet| {
            bullet.update();
            if player.collides(&bullet) {
                player.hit(5.);
                true
            } else {
                for ast in asteroids.iter_mut() {
                    if ast.collides(&bullet) {
                        ast.hit(5.);
                        return true;
                    }
                }
                false
            }
        });
        asteroids.retain(|ast| !ast.is_dead());

        bullets.self_collision();
        asteroids.self_collision();
        fuels.self_collision();

        fuels.iterate(|fuel| {
            asteroids.iterate(|ast| {
                check_and_resolve(fuel, ast);
            });
            bullets.iterate(|bul| {
                check_and_resolve(fuel, bul);
            });
        });
    }
}
