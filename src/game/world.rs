use ::*;
use self_compare::SliceCompareExt;

#[derive(Debug, Serialize, Deserialize)]
/// All the objects in the current world
pub struct World {
    pub(super) player: ThrustedObj,
    pub(super) asteroids: Vec<DestructableObj>,
    pub(super) fuels: Vec<Object>,
    pub(super) bullets: Vec<Object>,
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
        self.player.obj.rot += 1.7 * input_state.hor() * DELTA;

        self.player.update();

        self.player.thruster.power = input_state.ver() == 1.;
        self.player.thruster.throttle(input_state.throttle() as f64 * 4.5 * DDELTA);

        let mut consumed_fuel = Vec::new();
        for (i, fuel) in self.fuels.iter_mut().enumerate().rev() {
            fuel.update(DELTA);
            if self.player.collides(&fuel) {
                if (fuel.vel - self.player.vel).norm() <= 30. {
                    consumed_fuel.push(i);
                } else {
                    resolve(&mut self.player, fuel);
                }
            }
        }
        self.player.thruster.fuel += 200. * consumed_fuel.len() as f64;
        for i in consumed_fuel.into_iter() {
            self.fuels.remove(i);
        }
        for ast in &mut self.asteroids {
            ast.update(DELTA);
            check_and_resolve(&mut self.player, ast);
        }
        let mut dead_bullets = Vec::new();
        for (i, bullet) in self.bullets.iter_mut().enumerate().rev() {
            bullet.update(DELTA);
            if self.player.collides(&bullet) {
                self.player.hit(5.);
                dead_bullets.push(i);
            } else {
                for ast in &mut self.asteroids {
                    if ast.collides(&bullet) {
                        ast.hit(5.);
                        dead_bullets.push(i);
                    }
                }
            }
        }
        self.asteroids.retain(|ast| !ast.is_dead());
        for i in dead_bullets.into_iter() {
            self.bullets.remove(i);
        }
        self.bullets.compare_self_mut(check_and_resolve);

        self.asteroids.compare_self_mut(|a, b| check_and_resolve(a, b));
        self.fuels.compare_self_mut(check_and_resolve);
        for fuel in &mut self.fuels {
            for ast in &mut self.asteroids {
                check_and_resolve(fuel, ast);
            }
            for bul in &mut self.bullets {
                check_and_resolve(fuel, bul);
            }
        }
    }
}
