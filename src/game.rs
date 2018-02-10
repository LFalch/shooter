use ::*;

#[derive(Debug, Serialize, Deserialize)]
/// All the objects in the current world
pub struct World {
    player: PhysObj,
    asteroids: Vec<PhysObj>,
    fuels: Vec<PhysObj>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// The mode the engine is turned into
pub enum EngineMode {
    /// Thrust 1
    Thrust1,
    /// Thrust 2
    Thrust2,
    /// Engine is on
    On,
    /// Engine is off
    Off,
}

impl EngineMode {
    /// The fuel usage from the current engine mode
    pub fn fuel_usage(&self) -> f64 {
        match *self {
            EngineMode::Off => 0.,
            EngineMode::On => 0.01,
            EngineMode::Thrust1 => 10.,
            EngineMode::Thrust2 => 25.,
        }
    }
    /// The amount of thrust from the current engine mode
    pub fn acceleration(&self) -> f32 {
        match *self {
            EngineMode::Off => 0.,
            EngineMode::On => 0.1,
            EngineMode::Thrust1 => 35.,
            EngineMode::Thrust2 => 72.,
        }
    }
    /// The sprite of the ship with the current engine mode
    pub fn sprite(&self) -> Sprite {
        match *self {
            EngineMode::Off => Sprite::ShipOff,
            EngineMode::On => Sprite::ShipOn,
            EngineMode::Thrust1 => Sprite::ShipSpeed2,
            EngineMode::Thrust2 => Sprite::ShipSpeed3,
        }
    }
    /// Toggles the engine on and off
    pub fn toggle(&mut self) {
        match *self {
            EngineMode::Off => *self = EngineMode::On,
            _ => *self = EngineMode::Off,
        }
    }
    /// Turns the engine up a mode (saturating)
    pub fn up(&mut self) {
        match *self {
            EngineMode::Off | EngineMode::Thrust2 => (),
            EngineMode::On => *self = EngineMode::Thrust1,
            EngineMode::Thrust1 => *self = EngineMode::Thrust2,
        }
    }
    /// Turns the engine down a mode (saturating)
    pub fn down(&mut self) {
        match *self {
            EngineMode::Off | EngineMode::On => (),
            EngineMode::Thrust1 => *self = EngineMode::On,
            EngineMode::Thrust2 => *self = EngineMode::Thrust1,
        }
    }
}

/// The state of the game
pub struct State {
    input: InputState,
    assets: Assets,
    width: u32,
    height: u32,
    lines: bool,
    ast_spawn_coords: Option<Point2>,
    fuel_spawn_coords: Option<Point2>,
    offset: Vector2,
    world: World,
    engine: EngineMode,
    fuel: f64,
    fuel_text: PosText,
    fuel_usg_text: PosText,
    engine_mode_text: PosText,
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
        let fuel_text = assets.text(ctx, Point2::new(2.0, 0.0), "Fuel: 99999.99 L")?;
        let fuel_usg_text = assets.text(ctx, Point2::new(2.0, 14.0), "Fuel Usage: 33.3 L/s")?;
        let engine_mode_text = assets.text_ra(ctx, width as f32 - 5.0, 2.0, "Engine mode: thrust1")?;

        Ok(State {
            input: Default::default(),
            assets,
            width,
            height,
            ast_spawn_coords: None,
            fuel_spawn_coords: None,
            lines: false,
            fuel: 2e4,
            fuel_text,
            fuel_usg_text,
            engine_mode_text,
            offset: Vector2::new(0., 0.),
            engine: EngineMode::Off,
            world: World {
                // The world starts of with one asteroid at (150, 150)
                asteroids: vec![PhysObj::new(Point2::new(150., 150.), Sprite::Asteroid.radius())],
                // Initalise the player in the middle of the screen
                player: PhysObj::new(Point2::new(width as f32 / 2., height as f32 / 2.), Sprite::ShipOff.radius()),
                fuels: Vec::new(),
            }
        })
    }
    /// Update the text objects
    fn update_ui(&mut self, ctx: &mut Context) {
        // Using formatting to round of the numbers to 2 decimals (the `.2` part)
        let fuel_str = format!("Fuel: {:8.2} L", self.fuel);
        let fuel_usg_str = format!("Fuel Usage: {:4} L/s", self.engine.fuel_usage());
        let engine_mode_str = format!("Engine mode: {:7?}", self.engine);

        self.fuel_text.update_text(&self.assets, ctx, &fuel_str).unwrap();
        self.fuel_usg_text.update_text(&self.assets, ctx, &fuel_usg_str).unwrap();
        self.engine_mode_text.update_text(&self.assets, ctx, &engine_mode_str).unwrap();
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

        // Run this for every 1/60 of a second has passed since last update
        // Can in theory become slow
        while timer::check_update_time(ctx, DESIRED_FPS) {
            const DELTA: f32 = 1. / DESIRED_FPS as f32;
            const DDELTA: f64 = 1. / DESIRED_FPS as f64;

            let acc = self.engine.acceleration();
            // Rotate player if horizontal keys (A,D or left, right arrows)
            self.world.player.obj.rot += 1.7 * self.input.hor() * DELTA;
            // Set the acceleration of the player object according to the direction pointed and the vertical input axis
            self.world.player.acc = acc * angle_to_vec(self.world.player.obj.rot);
            self.fuel -= self.engine.fuel_usage() * DDELTA;

            // Update the player object
            self.world.player.update(DELTA);

            for fuel in &mut self.world.fuels {
                fuel.update(DELTA);
                if self.world.player.collides(&fuel) {
                    self.world.player.uncollide(fuel);
                    self.world.player.elastic_collide(fuel);
                }
            }
            for ast in &mut self.world.asteroids {
                ast.update(DELTA);
                if self.world.player.collides(&ast) {
                    self.world.player.uncollide(ast);
                    self.world.player.elastic_collide(ast);
                }
            }

            // Compare each asteroid with the other to see if they collide
            for i in 0..self.world.asteroids.len() {
                for j in i+1..self.world.asteroids.len() {
                    // To avoid having two mutable references to the same object we have to move it out first
                    let mut oth = std::mem::replace(&mut self.world.asteroids[j], PhysObj::new(Point2::new(0., 0.), Sprite::Asteroid.radius()));
                    // Check and resolve collision
                    if self.world.asteroids[i].collides(&oth) {
                        self.world.asteroids[i].uncollide(&mut oth);
                        self.world.asteroids[i].elastic_collide(&mut oth);
                    }
                    // Reset the asteroid we pulled out
                    self.world.asteroids[j] = oth;
                }
            }
            for i in 0..self.world.fuels.len() {
                for j in i+1..self.world.fuels.len() {
                    // To avoid having two mutable references to the same object we have to move it out first
                    let mut oth = std::mem::replace(&mut self.world.fuels[j], PhysObj::new(Point2::new(0., 0.), Sprite::Fuel.radius()));
                    // Check and resolve collision
                    if self.world.fuels[i].collides(&oth) {
                        self.world.fuels[i].uncollide(&mut oth);
                        self.world.fuels[i].elastic_collide(&mut oth);
                    }
                    // Reset the asteroid we pulled out
                    self.world.fuels[j] = oth;
                }
            }
            for fuel in &mut self.world.fuels {
                for ast in &mut self.world.asteroids {
                    if fuel.collides(&ast) {
                        fuel.uncollide(ast);
                        fuel.elastic_collide(ast);
                    }
                }
            }
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
        let s = self.engine.sprite();
        self.world.player.draw(ctx, self.assets.get_img(s))?;
        for ast in &self.world.asteroids {
            ast.draw(ctx, self.assets.get_img(Sprite::Asteroid))?;
        }
        for fuel in &self.world.fuels {
            fuel.draw(ctx, self.assets.get_img(Sprite::Fuel))?;
        }

        // If lines is turned on, draw lines for the velocity and acceleration vectors from the objects
        if self.lines {
            for ast in &self.world.asteroids {
                ast.draw_lines(ctx)?;
            }
            for fuel in &self.world.fuels {
                fuel.draw_lines(ctx)?;
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
        }

        // Draw the text in white
        graphics::set_color(ctx, graphics::WHITE)?;
        self.fuel_text.draw_text(ctx)?;
        self.fuel_usg_text.draw_text(ctx)?;
        self.engine_mode_text.draw_text(ctx)?;

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
        // Toggle lines on L
        // Clear all asteroids on R
        // Save the current `world` on Z
        // Load the last save on X
        match keycode {
            W | Up => self.input.ver -= 1,
            S | Down => self.input.ver += 1,
            A | Left => self.input.hor += 1,
            D | Right => self.input.hor -= 1,
            L => self.lines.toggle(),
            R => self.world.asteroids.clear(),
            I => self.engine.toggle(),
            Q => self.engine.down(),
            E => self.engine.up(),
            Z => save::save("save.sav", &self.world).unwrap(),
            X => save::load("save.sav", &mut self.world).unwrap(),
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
                let mut ast = PhysObj::new(p - self.offset, Sprite::Asteroid.radius());
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
                let mut fuel = PhysObj::new(p - self.offset, Sprite::Fuel.radius());
                // Set the velocity so it moves towards where the mouse is now
                fuel.vel = fuel.pos - Point2::new(x as f32, y as f32) + self.offset;
                fuel.vel += self.world.player.vel;
                self.world.fuels.push(fuel);
            }
        }
    }
    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        println!("Closing, auto-saving game");
        // Save the world state to a file
        save::save("autosave.sav", &self.world).unwrap();

        false
    }
}
