/// Extensions for booleans
pub trait BoolExt {
    /// Toggle the value
    fn toggle(&mut self);
}

impl BoolExt for bool {
    fn toggle(&mut self) {
        // so `true` becomes `false` and vice versa
        *self = !*self;
    }
}

#[derive(Debug, Default)]
/// Tracks how many buttons are being pressed in specific directions
pub struct InputState {
    /// Buttons in the left-right direction
    pub hor: i8,
    /// Buttons in the up-down direction
    pub ver: i8,
}

impl InputState {
    #[inline]
    /// Returns `-1`, `0` or `1` depending on whether `self.hor` is negative, zero or positive
    pub fn hor(&self) -> f32 {
        self.hor.signum() as f32
    }
    /// Returns `-1`, `0` or `1` depending on whether `self.ver` is negative, zero or positive
    #[inline]
    pub fn ver(&self) -> f32 {
        self.ver.signum() as f32
    }
}
