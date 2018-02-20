use ::*;
use self_compare::SliceCompareExt;

#[derive(Debug, Serialize, Deserialize)]
/// All the objects in the current world
pub struct World {
    pub(super) player: ThrustedObj,
    pub(super) asteroids: Vec<DestructableObj>,
    pub(super) fuels: Vec<PhysObj>,
    pub(super) bullets: Vec<PhysObj>,
}

impl World {
    pub(super) fn physics_update(&mut self, input_state: &InputState) {
        self.player.obj.rot += 1.7 * input_state.hor() * DELTA;

        self.player.update();

        if input_state.ver() == 1. {
            self.player.thruster.power = true;
        } else {
            self.player.thruster.power = false;
        }
        self.player.thruster.throttle(input_state.throttle() as f64 * 4.5 * DDELTA);

        let mut consumed_fuel = Vec::new();
        for (i, fuel) in self.fuels.iter_mut().enumerate().rev() {
            fuel.update(DELTA);
            if self.player.collides(&fuel) {
                if (fuel.vel - self.player.vel).norm() <= 30. {
                    consumed_fuel.push(i);
                } else {
                    self.player.uncollide(fuel);
                    self.player.elastic_collide(fuel);
                }
            }
        }
        self.player.thruster.fuel += 200. * consumed_fuel.len() as f64;
        for i in consumed_fuel.into_iter() {
            self.fuels.remove(i);
        }
        for ast in &mut self.asteroids {
            ast.update(DELTA);
            if self.player.collides(&ast) {
                self.player.uncollide(ast);
                self.player.elastic_collide(ast);
            }
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
        self.bullets.compare_self_mut(|bul, oth| {
            if bul.collides(&oth) {
                bul.uncollide(oth);
                bul.elastic_collide(oth);
            }
        });

        self.asteroids.compare_self_mut(|ast, oth| {
            if ast.collides(&oth) {
                ast.uncollide(oth);
                ast.elastic_collide(oth);
            }
        });
        self.fuels.compare_self_mut(|fuel, oth| {
            if fuel.collides(&oth) {
                fuel.uncollide(oth);
                fuel.elastic_collide(oth);
            }
        });
        for fuel in &mut self.fuels {
            for ast in &mut self.asteroids {
                if fuel.collides(&ast) {
                    fuel.uncollide(ast);
                    fuel.elastic_collide(ast);
                }
            }
            for bul in &mut self.bullets {
                if fuel.collides(&bul) {
                    fuel.uncollide(bul);
                    fuel.elastic_collide(bul);
                }
            }
        }
    }
}
