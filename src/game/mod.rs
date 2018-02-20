use ::*;

/// Stuff related to things in the world
pub mod world;

use self::world::*;

/// The state of the game
pub struct State {
    input: InputState,
    assets: Assets,
    width: u32,
    height: u32,
    mouse: Point2,
    lines: bool,
    ast_spawn_coords: Option<Point2>,
    fuel_spawn_coords: Option<Point2>,
    offset: Vector2,
    world: World,
    fuel_text: PosText,
    fuel_usg_text: PosText,
    health_text: PosText,
}

const DESIRED_FPS: u32 = 60;

pub(crate) const DELTA: f32 = 1. / DESIRED_FPS as f32;
pub(crate) const DDELTA: f64 = 1. / DESIRED_FPS as f64;

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
        let fuel_text = assets.text(ctx, Point2::new(2.0, 0.0), "Fuel: 99999.99 L")?;
        let fuel_usg_text = assets.text(ctx, Point2::new(2.0, 16.0), "Throttle: 33.3 L/s")?;
        let health_text = assets.text_ra(ctx, width as f32 - 5.0, 18.0, "Health: 999")?;

        Ok(State {
            input: Default::default(),
            assets,
            width,
            height,
            ast_spawn_coords: None,
            fuel_spawn_coords: None,
            lines: false,
            fuel_text,
            fuel_usg_text,
            health_text,
            mouse: Point2::new(0., 0.),
            offset: Vector2::new(0., 0.),
            world: World {
                bullets: Vec::new(),
                // The world starts of with one asteroid at (150, 150)
                asteroids: vec![make_asteroid(Point2::new(150., 150.))],
                // Initalise the player in the middle of the screen
                player: make_player(Point2::new(width as f32 / 2., height as f32 / 2.)),
                fuels: Vec::new(),
            }
        })
    }
    /// Update the text objects
    fn update_ui(&mut self, ctx: &mut Context) {
        // Using formatting to round of the numbers to 2 decimals (the `.2` part)
        let fuel_str = format!("Fuel: {:8.2} L", self.world.player.thruster.fuel);
        let fuel_usg_str = format!("Throttle: {:2.1} L/s", self.world.player.thruster.throttle_usage);
        let health_str = format!("Health: {:3.0}", self.world.player.health);

        self.fuel_text.update_text(&self.assets, ctx, &fuel_str).unwrap();
        self.fuel_usg_text.update_text(&self.assets, ctx, &fuel_usg_str).unwrap();
        self.health_text.update_text(&self.assets, ctx, &health_str).unwrap();
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
        // Run this for every 1/60 of a second has passed since last update
        // Can in theory become slow
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.world.physics_update(&self.input);
        }

        self.world.player.obj.rot %= 2.*::std::f32::consts::PI;

        // Update the UI
        self.update_ui(ctx);
        // Center the camera on the player
        let p = self.world.player.pos;
        self.focus_on(p);

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
        let s = self.world.player.thruster.sprite();
        self.world.player.draw(ctx, self.assets.get_img(s))?;
        for ast in &self.world.asteroids {
            ast.draw(ctx, self.assets.get_img(Sprite::Asteroid))?;
        }
        for fuel in &self.world.fuels {
            fuel.draw(ctx, self.assets.get_img(Sprite::Fuel))?;
        }
        for bullet in &self.world.bullets {
            bullet.draw(ctx, self.assets.get_img(Sprite::Bullet))?;
        }

        // If lines is turned on, draw lines for the velocity and acceleration vectors from the objects
        if self.lines {
            for ast in &self.world.asteroids {
                ast.draw_lines(ctx)?;
            }
            for fuel in &self.world.fuels {
                fuel.draw_lines(ctx)?;
            }
            for bullet in &self.world.bullets {
                bullet.draw_lines(ctx)?;
            }
            self.world.player.draw_lines(ctx)?;
        }

        // Pop the offset tranformation to draw the UI on the screen
        graphics::pop_transform(ctx);
        graphics::apply_transformations(ctx)?;

        // Check if an asteroid is being spawned
        if let Some(proto_pos) = self.ast_spawn_coords {
            // Draw the asteroid transparently so you can see you're making an asteroid
            let params = graphics::DrawParam {
                dest: proto_pos,
                offset: Point2::new(0.5, 0.5),
                color: Some(TRANS),
                .. Default::default()
            };
            graphics::draw_ex(ctx, self.assets.get_img(Sprite::Asteroid), params)?;
            if self.lines {
                graphics::set_color(ctx, GREEN)?;
                graphics::line(ctx, &[proto_pos, proto_pos - (self.mouse-proto_pos)], 2.)?;
            }
        }
        if let Some(proto_pos) = self.fuel_spawn_coords {
            // Draw the asteroid transparently so you can see you're making an asteroid
            let params = graphics::DrawParam {
                dest: proto_pos,
                offset: Point2::new(0.5, 0.5),
                color: Some(TRANS),
                .. Default::default()
            };
            graphics::draw_ex(ctx, self.assets.get_img(Sprite::Fuel), params)?;
            if self.lines {
                graphics::set_color(ctx, GREEN)?;
                graphics::line(ctx, &[proto_pos, proto_pos - (self.mouse-proto_pos)], 2.)?;
            }
        }

        // Draw the text in white
        graphics::set_color(ctx, graphics::WHITE)?;
        self.fuel_text.draw_text(ctx)?;
        self.fuel_usg_text.draw_text(ctx)?;
        self.health_text.draw_text(ctx)?;

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
            LShift => self.input.throttle += 1,
            LCtrl => self.input.throttle -= 1,
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
        // Toggle lines on L
        // Clear all asteroids on R
        // Save the current `world` on Z
        // Load the last save on X
        match keycode {
            W | Up => self.input.ver -= 1,
            S | Down => self.input.ver += 1,
            A | Left => self.input.hor += 1,
            D | Right => self.input.hor -= 1,
            LShift => self.input.throttle -= 1,
            LCtrl => self.input.throttle += 1,
            L => self.lines.toggle(),
            R => self.world.asteroids.clear(),
            I => self.world.player.thruster.throttle_usage = 0.,
            Z => save::save("save.sav", &self.world).unwrap(),
            X => save::load("save.sav", &mut self.world).unwrap(),
            Space => {
                let d = angle_to_vec(self.world.player.rot);

                let mut bullet = make_bullet(self.world.player.pos + 34. * d);
                bullet.vel = self.world.player.vel + 200. * d;
                bullet.rot = self.world.player.rot;
                self.world.bullets.push(bullet);
            }
            _ => return,
        }
    }
    /// Handle mouse down event
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, btn: MouseButton, x: i32, y: i32) {
        // Set the spawn_coords so we can spawn an asteroid when the button is released
        if let MouseButton::Left = btn {
            self.ast_spawn_coords = Some(Point2::new(x as f32, y as f32));
        }
        if let MouseButton::Right = btn {
            self.fuel_spawn_coords = Some(Point2::new(x as f32, y as f32));
        }
    }
    /// Handle mouse release events
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, btn: MouseButton, x: i32, y: i32) {
        if let MouseButton::Left = btn {
            // Get the spawn_coords and replace them with `None`
            if let Some(p) = ::std::mem::replace(&mut self.ast_spawn_coords, None) {
                // Make a new asteroid object wherever the mouse pointed when the button was pressed down
                let mut ast = make_asteroid(p - self.offset);
                // Set the velocity so it moves towards where the mouse is now
                ast.vel = ast.pos - Point2::new(x as f32, y as f32) + self.offset;
                ast.vel += self.world.player.vel;
                // Push it to the asteroids vector ("dynamic array" not a maths vector)
                self.world.asteroids.push(ast);
            }
        }
        if let MouseButton::Right = btn {
            // Get the spawn_coords and replace them with `None`
            if let Some(p) = ::std::mem::replace(&mut self.fuel_spawn_coords, None) {
                // Make a new object wherever the mouse pointed when the button was pressed down
                let mut fuel = make_fuel(p - self.offset);
                // Set the velocity so it moves towards where the mouse is now
                fuel.vel = fuel.pos - Point2::new(x as f32, y as f32) + self.offset;
                fuel.vel += self.world.player.vel;
                self.world.fuels.push(fuel);
            }
        }
    }
    /// Handles mouse movement events
    fn mouse_motion_event(&mut self, _: &mut Context, _: MouseState, x: i32, y: i32, _: i32, _: i32) {
        self.mouse = Point2::new(x as f32, y as f32);
    }
    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        println!("Closing, auto-saving game");
        // Save the world state to a file
        save::save("autosave.sav", &self.world).unwrap();

        false
    }
}
