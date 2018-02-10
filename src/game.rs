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
    // To keep track of how long the engines have been running
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
        // Background colour is black
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());
        // Initialise assets
        let assets = Assets::new(ctx)?;

        // Get the window's dimensions
        let width = ctx.conf.window_mode.width;
        let height = ctx.conf.window_mode.height;

        // Initialise the text objects
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
                // The world starts of with one asteroid at (150, 150)
                asteroids: vec![RotatableObj::new(Point2::new(150., 150.), Sprite::Asteroid, 0.1)],
                // Initalise the player in the middle of the screen
                player: PhysObj::new(Point2::new(width as f32 / 2., height as f32 / 2.), Sprite::ShipOff)
            }
        })
    }
    /// Update the text objects
    fn update_ui(&mut self, ctx: &mut Context) {
        // Using formatting to round of the numbers to 2 decimals (the `.2` part)
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
    /// Sets the offset so that the given point will be centered on the screen
    fn focus_on(&mut self, p: Point2) {
        self.offset = -p.coords + 0.5 * Vector2::new(self.width as f32, self.height as f32);
    }
    /// Draws repeating parallax background
    pub fn draw_bg(&mut self, ctx: &mut Context, scale: f32, s: Sprite) -> GameResult<()> {
        let p = scale * self.offset;
        let mut x = p.x;
        let mut y = p.y;

        let (w, h) = (s.width(), s.height());

        while x < 0. {
            x += w;
        }
        x %= w;
        while y < 0. {
            y += h;
        }
        y %= h;

        let img = self.assets.get_img(s);
        graphics::draw(ctx, img, Point2::new(x, y), 0.)?;
        graphics::draw(ctx, img, Point2::new(x-w, y), 0.)?;
        graphics::draw(ctx, img, Point2::new(x, y-h), 0.)?;
        graphics::draw(ctx, img, Point2::new(x-w, y-h), 0.)?;

        Ok(())
    }
}


impl EventHandler for State {
    // Handle the game logic
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        let width = self.width as f32;
        let height = self.height as f32;

        // Run this for every 1/60 of a second has passed since last update
        // Can in theory become slow
        while timer::check_update_time(ctx, DESIRED_FPS) {
            const DELTA: f32 = 1. / DESIRED_FPS as f32;

            // If there is no input on the vertical axis (W,S or up,down arrows)
            // set the `on_time` zero and make the sprite a turned off ship
            // otherwise turn on the ship and add onto its `on_time`
            if self.input.ver != 0 {
                self.on_time += DELTA;
                self.world.player.obj.spr = Sprite::ShipOn;
            } else {
                self.on_time = 0.;
                self.world.player.obj.spr = Sprite::ShipOff;
            }
            // Set the acceleration and sprite based on the `on_time`
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
            // Rotate player if horizontal keys (A,D or left, right arrows)
            self.world.player.obj.rot += 1.7 * self.input.hor() * DELTA;
            // Set the acceleration of the player object according to the direction pointed and the vertical input axis
            self.world.player.acc = acc * angle_to_vec(self.world.player.obj.rot) * self.input.ver();

            // Update the player object
            self.world.player.update(DELTA);
            // If rebound is turned on, check rebound of the player
            if self.rebound {
                self.world.player.rebound(width, height);
            }

            // Compare each asteroid with the other to see if they collide
            for i in 0..self.world.asteroids.len() {
                for j in i+1..self.world.asteroids.len() {
                    // To avoid having two mutable references to the same object we have to move it out first
                    let mut oth = std::mem::replace(&mut self.world.asteroids[j], RotatableObj::new(Point2::new(0., 0.), Sprite::Asteroid, 0.));
                    // Check and resolve collision
                    if self.world.asteroids[i].collides(&oth) {
                        self.world.asteroids[i].uncollide(&mut oth);
                        self.world.asteroids[i].elastic_collide(&mut oth);
                    }
                    // Reset the asteroid we pulled out
                    self.world.asteroids[j] = oth;
                }
            }

            // Update each asteroid and check for collisions and rebound
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

        // Update the UI
        self.update_ui(ctx);
        // If rebound is turned off, center the camera on the player
        if !self.rebound {
            let p = self.world.player.pos;
            self.focus_on(p);
        } else {
            self.offset = Vector2::new(0., 0.);
        }

        Ok(())
    }

    // Draws everything
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the screen first
        graphics::clear(ctx);
        self.draw_bg(ctx, 0.1, Sprite::StarsBg)?;

        // Offset the current drawing with a translation from the `offset`
        graphics::push_transform(ctx, Some(Matrix4::new_translation(&self.offset.fixed_resize(0.))));
        graphics::apply_transformations(ctx)?;

        // Draw player and asteroids
        self.world.player.draw(ctx, &self.assets)?;
        for ast in &self.world.asteroids {
            ast.draw(ctx, &self.assets)?;
        }

        // If lines is turned on, draw lines for the velocity and acceleration vectors from the objects
        if self.lines {
            for ast in &self.world.asteroids {
                ast.draw_lines(ctx)?;
            }
            self.world.player.draw_lines(ctx)?;
        }

        // Pop the offset tranformation to draw the UI on the screen
        graphics::pop_transform(ctx);
        graphics::apply_transformations(ctx)?;

        // Check if an asteroid is being spawned
        if let Some(proto_pos) = self.spawn_coords {
            // Draw the asteroid transparently so you can see you're making an asteroid
            let params = graphics::DrawParam {
                dest: proto_pos,
                offset: Point2::new(0.5, 0.5),
                color: Some(TRANS),
                .. Default::default()
            };
            graphics::draw_ex(ctx, self.assets.get_img(Sprite::Asteroid), params)?;
        }

        // Draw the text in white
        graphics::set_color(ctx, graphics::WHITE)?;
        self.pos_text.draw_text(ctx)?;
        self.vel_text.draw_text(ctx)?;
        self.acc_text.draw_text(ctx)?;
        self.rot_text.draw_text(ctx)?;

        // Flip the buffers to see what we just drew
        graphics::present(ctx);

        // Give the computer some time to do other things
        timer::yield_now();
        Ok(())
    }
    /// Handle key down events
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _: Mod, repeat: bool) {
        // If this is a repeat event, we don't care
        if repeat {
            return
        }
        use Keycode::*;
        // Update input axes and quit game on Escape
        match keycode {
            W | Up => self.input.ver += 1,
            S | Down => self.input.ver -= 1,
            A | Left => self.input.hor -= 1,
            D | Right => self.input.hor += 1,
            Escape => ctx.quit().unwrap(),
            _ => return,
        }
    }
    /// Handle key release events
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, repeat: bool) {
        // Still don't care about repeats
        if repeat {
            return
        }
        use Keycode::*;
        // Update input axes in the opposite direction
        // Toggle rebound and lines on respectively K and L
        // Clear all asteroids on R
        // Save the current `world` on Z
        // Load the last save on X
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
    /// Handle mouse down event
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, btn: MouseButton, x: i32, y: i32) {
        // Set the spawn_coords so we can spawn an asteroid when the button is released
        if let MouseButton::Left = btn {
            self.spawn_coords = Some(Point2::new(x as f32, y as f32));
        }
    }
    /// Handle mouse release events
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, btn: MouseButton, x: i32, y: i32) {
        if let MouseButton::Left = btn {
            // Get the spawn_coords and replace them with `None`
            if let Some(p) = ::std::mem::replace(&mut self.spawn_coords, None) {
                // Make a new asteroid object wherever the mouse pointed when the button was pressed down
                let mut ast = RotatableObj::new(p - self.offset, Sprite::Asteroid, -0.2);
                // Set the velocity so it moves towards where the mouse is now
                ast.vel = ast.pos - Point2::new(x as f32, y as f32) + self.offset;
                if self.offset != Vector2::new(0., 0.) {
                    // If we're following the player we want it to move relative to the player
                    ast.vel += self.world.player.vel;
                }
                // Push it to the asteroids vector ("dynamic array" not a maths vector)
                self.world.asteroids.push(ast);
            }
        }
        // Move the player to the mouse cursor
        if let MouseButton::Middle = btn {
            self.world.player.pos = Point2::new(x as f32, y as f32) - self.offset;
        }
        // Set the player velocity to go towards the mouse
        if let MouseButton::Right = btn {
            self.world.player.vel = Point2::new(x as f32, y as f32) - self.offset - self.world.player.pos;
        }
    }
    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        println!("Closing, auto-saving game");
        // Save the world state to a file
        save::save("autosave.sav", &self.world);

        false
    }
}
