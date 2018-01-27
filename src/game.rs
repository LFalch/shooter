use ::*;

#[derive(Debug, Serialize, Deserialize)]
/// All the objects in the current world
pub struct World {
    player: PhysObj,
    asteroids: Vec<RotatableObj>,
}

/// The state of the game
pub struct State {
    input: InputState,
    assets: Assets,
    width: u32,
    height: u32,
    on_time: f32,
    rebound: bool,
    lines: bool,
    spawn_coords: Option<Point2>,
    offset: Vector2,
    world: World,
    rot_text: PosText,
    vel_text: PosText,
    pos_text: PosText,
    acc_text: PosText,
}

impl State {
    /// Make a new state object
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());
        let assets = Assets::new(ctx)?;

        let width = ctx.conf.window_mode.width;
        let height = ctx.conf.window_mode.height;

        let acc_text = assets.text(ctx, Point2::new(2.0, 0.0), "Acc: (0.00, 0.00)")?;
        let pos_text = assets.text(ctx, Point2::new(2.0, 14.0), "Pos: (0.00, 0.00)")?;
        let vel_text = assets.text(ctx, Point2::new(2.0, 28.0), "Vel: (0.00, 0.00)")?;
        let rot_text = assets.text_ra(ctx, width as f32 - 5.0, 2.0, "Rotation: {:6.2}")?;

        Ok(State {
            input: Default::default(),
            assets,
            width,
            height,
            on_time: 0.,
            spawn_coords: None,
            rebound: false,
            lines: true,
            rot_text,
            pos_text,
            vel_text,
            acc_text,
            offset: Vector2::new(0., 0.),
            world: World {
                asteroids: vec![RotatableObj::new(Point2::new(150., 150.), Sprite::Asteroid, 0.1)],
                player: PhysObj::new(Point2::new(width as f32 / 2., height as f32 / 2.), Sprite::ShipOff)
            }
        })
    }
    fn update_ui(&mut self, ctx: &mut Context) {
        let pos_str = format!("Pos: ({:8.2}, {:8.2})", self.world.player.obj.pos.x, self.world.player.obj.pos.y);
        let vel_str = format!("Vel: ({:8.2}, {:8.2}) (Mag: {:4.1})", self.world.player.vel.x, self.world.player.vel.y, self.world.player.vel.norm());
        let acc_str = format!("Acc: ({:8.2}, {:8.2}) (Mag: {:4.1}) Asteroids: {:2}", self.world.player.acc.x, self.world.player.acc.y, self.world.player.acc.norm(), self.world.asteroids.len());
        self.world.player.obj.rot %= 2.*::std::f32::consts::PI;
        let mut rot = self.world.player.obj.rot * 180./::std::f32::consts::PI;
        if rot < 0. {
            rot += 360.;
        }
        let rot_str = format!("Rotation: {:6.2}", rot);
        self.pos_text.update_text(&self.assets, ctx, &pos_str).unwrap();
        self.vel_text.update_text(&self.assets, ctx, &vel_str).unwrap();
        self.acc_text.update_text(&self.assets, ctx, &acc_str).unwrap();
        self.rot_text.update_text(&self.assets, ctx, &rot_str).unwrap();
    }
    fn focus_on(&mut self, p: Point2) {
        self.offset = -p.coords + 0.5 * Vector2::new(self.width as f32, self.height as f32);
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        let width = self.width as f32;
        let height = self.height as f32;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            const DELTA: f32 = 1. / DESIRED_FPS as f32;

            if self.input.ver != 0 {
                self.on_time += DELTA;
                self.world.player.obj.spr = Sprite::ShipOn;
            } else {
                self.on_time = 0.;
                self.world.player.obj.spr = Sprite::ShipOff;
            }
            let acc;
            if self.on_time > 0.5 {
                if self.on_time > 1.5 {
                    if self.on_time > 2.3 {
                        self.world.player.obj.spr = Sprite::ShipSpeed3;
                        acc = 65.;
                    } else {
                        self.world.player.obj.spr = Sprite::ShipSpeed2;
                        acc = 50.;
                    }
                } else {
                    self.world.player.obj.spr = Sprite::ShipSpeed1;
                    acc = 30.;
                }
            } else {
                acc = 10.;
            }
            self.world.player.obj.rot += 1.7 * self.input.hor() * DELTA;
            self.world.player.acc = acc * angle_to_vec(self.world.player.obj.rot) * self.input.ver();

            self.world.player.update(DELTA);
            if self.rebound {
                self.world.player.rebound(width, height);
            }

            for i in 0..self.world.asteroids.len() {
                for j in i+1..self.world.asteroids.len() {
                    let mut oth = std::mem::replace(&mut self.world.asteroids[j], RotatableObj::new(Point2::new(0., 0.), Sprite::Asteroid, 0.));
                    if self.world.asteroids[i].collides(&oth) {
                        self.world.asteroids[i].uncollide(&mut oth);
                        self.world.asteroids[i].elastic_collide(&mut oth);
                    }
                    self.world.asteroids[j] = oth;
                }
            }

            for ast in &mut self.world.asteroids {
                ast.update(DELTA);
                if self.rebound {
                    ast.rebound(width, height);
                }
                if self.world.player.collides(&ast) {
                    self.world.player.uncollide(ast);
                    self.world.player.elastic_collide(ast);
                }
            }
        }

        self.update_ui(ctx);
        if !self.rebound {
            let p = self.world.player.pos;
            self.focus_on(p);
        } else {
            self.offset = Vector2::new(0., 0.);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::push_transform(ctx, Some(Matrix4::new_translation(&self.offset.fixed_resize(0.))));
        graphics::apply_transformations(ctx)?;

        self.world.player.draw(ctx, &self.assets)?;
        for ast in &self.world.asteroids {
            ast.draw(ctx, &self.assets)?;
        }

        if self.lines {
            for ast in &self.world.asteroids {
                ast.draw_lines(ctx)?;
            }
            self.world.player.draw_lines(ctx)?;
        }

        graphics::pop_transform(ctx);
        graphics::apply_transformations(ctx)?;

        if let Some(proto_pos) = self.spawn_coords {
            let params = graphics::DrawParam {
                dest: proto_pos,
                offset: Point2::new(0.5, 0.5),
                color: Some(TRANS),
                .. Default::default()
            };
            graphics::draw_ex(ctx, self.assets.get_img(Sprite::Asteroid), params)?;
        }

        graphics::set_color(ctx, graphics::WHITE)?;
        self.pos_text.draw_text(ctx)?;
        self.vel_text.draw_text(ctx)?;
        self.acc_text.draw_text(ctx)?;
        self.rot_text.draw_text(ctx)?;

        // Then we flip the screen...
        graphics::present(ctx);

        timer::yield_now();
        Ok(())
    }
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _: Mod, repeat: bool) {
        if repeat {
            return
        }
        use Keycode::*;
        match keycode {
            W | Up => self.input.ver += 1,
            S | Down => self.input.ver -= 1,
            A | Left => self.input.hor -= 1,
            D | Right => self.input.hor += 1,
            Escape => ctx.quit().unwrap(),
            _ => return,
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, repeat: bool) {
        if repeat {
            return
        }
        use Keycode::*;
        match keycode {
            W | Up => self.input.ver -= 1,
            S | Down => self.input.ver += 1,
            A | Left => self.input.hor += 1,
            D | Right => self.input.hor -= 1,
            K => self.rebound.toggle(),
            L => self.lines.toggle(),
            R => self.world.asteroids.clear(),
            Z => save::save("save.sav", &self.world),
            X => save::load("save.sav", &mut self.world),
            _ => return,
        }
    }
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, btn: MouseButton, x: i32, y: i32) {
        if let MouseButton::Left = btn {
            self.spawn_coords = Some(Point2::new(x as f32, y as f32));
        }
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, btn: MouseButton, x: i32, y: i32) {
        if let MouseButton::Left = btn {
            if let Some(p) = ::std::mem::replace(&mut self.spawn_coords, None) {
                let mut ast = RotatableObj::new(p - self.offset, Sprite::Asteroid, -0.2);
                ast.vel = ast.pos - Point2::new(x as f32, y as f32) + self.offset;
                if self.offset != Vector2::new(0., 0.) {
                    ast.vel += self.world.player.vel;
                }
                self.world.asteroids.push(ast);
            }
        }
        if let MouseButton::Middle = btn {
            self.world.player.pos = Point2::new(x as f32, y as f32) - self.offset;
        }
        if let MouseButton::Right = btn {
            self.world.player.vel = Point2::new(x as f32, y as f32) - self.offset - self.world.player.pos;
        }
    }
}
